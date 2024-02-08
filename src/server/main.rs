use std::collections::HashMap;
use std::env;
use std::error::Error as GenericError;
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::time::SystemTime;

use args::args::{Args, ParsingError};
use gamecontrol::game::GameController;
use teams::team::Team;
use stream_utils::stream_utils::send_pkt_to_stream;
use utils::utils::copy_until_char;
use action::action::{ReadyAction, Action, ActionResult, NO_ACTION};
use crate::action::action::SpecialActionParam;
use crate::paket_crafter::paquet_crafter::{ craft_gfx_packet_action_receive,
                                            craft_gfx_packet_action_ready,
                                            craft_gfx_packet_die,
                                            craft_client_packet_action_receive, craft_client_packet_die, craft_client_packet_action_ready};
use crate::stream_utils::stream_utils::{first_connection_gfx, get_initial_gfx_packets_from_game_ctrl};
use crate::game_utils::game_utils::{find_index_action, find_player_from_id, get_post_actions, get_pre_actions};


//add module in the crate root
pub mod args;
pub mod cell;
pub mod gamecontrol;
pub mod player;
pub mod ressources;
pub mod teams;
pub mod action;
pub mod init;
pub mod paket_crafter;
pub mod utils;
pub mod game_utils;
pub mod stream_utils;

static GFX_SERVER_PORT: u16 = 1312;
const COMMAND_SLICE: [&'static str; 12] = ["avance", "droite", "gauche", "voir", "inventaire", "expulse", "incantation", "fork", "connect_nbr", "prend ", "pose ", "broadcast "]; 
const RESSOURCES_SLICE: [&'static str; 7] = ["food", "linemate", "deraumere", "sibure", "mendiane", "phiras", "thystame"];
const BUF_SIZE: usize = 4096;


/*
**  check if their is a winner 
**  (need one guy at level 8)
**/
fn check_winner(teams: &Vec<Team>) -> bool
{
    for team in teams
    {
        for player in &team.players
        {
            if player.level == 8
            {
                return true;
            }
        }
    }
    false
}

 /***********************************************************************************
 * Check if the packet contains the correct teamname, then proceed our handshake.
 * Kick the player if his team is full and drop the connexion
 * Generate the player in the Gamecontroller structure with a new id
 * 
 * 
 * params:
 *      mut stream: TcpStream
 *      hashmap: & mut HashMap<String, u8>
 *      args: & mut Args
 *      id: & mut u32, 
 *      game_ctrl: & mut GameController
 * 
 * return:
 *       ()
*********************************************************************************/
fn create_player_or_kick(stream: & mut TcpStream, hashmap: & mut HashMap<String, u8>, args: & mut Args, id: & mut u32, game_ctrl: & mut GameController)
{
    let mut teamname_buffer = [0 as u8; BUF_SIZE];
    let string_teamname_buffer: String;
    if let  Ok(_) = stream.read(& mut teamname_buffer)  
    {
        string_teamname_buffer = copy_until_char(&teamname_buffer, b'\0');
        match args.n.contains(&string_teamname_buffer)
        {
            true => 
            {
                // Add the receive teamname to the hashtable and verify if the team is full or not
                let nbr_player_in_current_team =  hashmap
                    .entry( string_teamname_buffer.clone())
                    .or_insert(0);

                // compare the teamnames received with the teamnames parsed
                if nbr_player_in_current_team >= & mut args.c
                {
                    // send the Endconnection to kill the client
                    // kick the player
                    println!("team {:?} is full", string_teamname_buffer);
                    let _ = stream.write("Endconnection".to_string().as_bytes());
                    drop(stream)
                }
                else 
                {
                    // create a new id
                    // send back the id to the client
                    // save the player 
                    // save the stream into player
                    *nbr_player_in_current_team += 1;
                    *id += 1;
                    let _ = stream.write(&id.to_string().as_bytes());
                    game_ctrl.get_team_and_push(&string_teamname_buffer, *id, &stream, args.x, args.y);
                    //println!("{:#?}", game_ctrl);
                }
            }
            false => 
            {
                println!("bad_entry");
            }
        }
       
    }
    // mut &mut hashtable   
}



/***********************************************************************************
 * Verify if all clients in a team are connected
 * 
 * params:
 *      c: number of client / team
 *      len: number of team
 *      hashmap: represent the number of player connecteds / team --> {'team_name': nb_player}
 * 
 * return:
 *       true if everybody are connected in a team
 *********************************************************************************/
 pub fn client_all_connect(c: u8, len: usize, hashmap: &mut HashMap<String, u8>) -> bool
 {
    match hashmap
        .iter()
        .filter(|&(_, &key)| key == c)
        .map(|_| 1)
        .count()
    {
        hashmap_len if hashmap_len == len   => {return true;}
        _                                          => {return false;}
    }
}
    
/*
**  parse args
**/
fn parsing() -> Result<Args, ParsingError> 
{
    let vec_args: Vec<String> = env::args().collect();
    let server_arg: Args = Args::new(vec_args)?;
    Ok(server_arg)
}

/*
**  check if the parameter of a command is valid
**  ex: `prend food` -> true
**      `prend caca` -> false
**/
fn is_valid_obj(object: &str) -> bool
{
    match object
    {
        txt if RESSOURCES_SLICE.iter().any(|&s| s == txt) => true,
        _ => false
    }
} 

/*
**  check if a command and it's argument is valid
**  ex: `avance` -> true
**      `pisser` -> false
**/
fn is_valid_cmd(buf: &String) -> bool
{
    match buf
    {
        txt if COMMAND_SLICE.iter().any(|&s| s == txt) =>  true,
        txt if COMMAND_SLICE.iter().any(|&s| txt.starts_with(s)) => {
            let mut tmp = buf.split_whitespace();
            let _ = tmp.next();
            is_valid_obj(tmp.next().unwrap())
        },
        _ => false,
    }
}

/*
**  take the second elem of a string contains whitespace, verify
**  if the second elem is a valid ressource and return it
**  ex: `toto food` -> Some("food")
**      `avance` -> None
**      `prend phiras` -> Some("phiras")
**/
fn get_obj_from_string(command: &String) -> Option<String>
{
    match command
    {
        command if RESSOURCES_SLICE.iter().any(|&elem| command.ends_with(elem)) => 
        {
            let mut split = command.split_whitespace();
            let object = split.nth(1);
            let tmp = object.unwrap().to_string();
            Some(tmp)
        }
        _ => None
    }
} 

/*
**  receive data onto TCP stream and create Action for corresponding player
**  params:
**      stream:     the TCP stream where the data is receive
**      game_ctrl:  game datas
**  return:
**      Vec<Action>: list of all new cmd receive from stream
**/
fn receive_action(stream: & mut TcpStream, game_ctrl: & mut GameController)// -> Vec<ReadyAction>
{
    let mut action_receive = [0 as u8; BUF_SIZE];
    //let mut actions : Vec<ReadyAction> = Vec::new();

    // println!("receive action from : {:?}", stream);
    if let  Ok(_) = stream.read(& mut action_receive)  
    {
        for team in & mut game_ctrl.teams
        {
            for player in & mut team.players
            {
                if player.port == stream
                                    .peer_addr()
                                    .unwrap()
                                    .port()
                    && player.actions.len() < 11
                {
                    // TODO : attention ici au mecanisme des 10 commandes
                    // si l'une echoue, les suivantes ne seront peut etre plus
                    // pertinentes ! il faudra donc modifier ca pour n'envoyer des cmd
                    // que lorsqu'on a une reponse
                    let mut vec_string_command: Vec<String> = Vec::with_capacity(10);
                    // cut the buffer into 10 segments of BUF_SIZE/10 bytes to retreive the 10 cmds
                    for i in 0..10
                    {
                        vec_string_command.push(copy_until_char(&action_receive[(BUF_SIZE / 10) * i..(BUF_SIZE / 10) * (i+1)], b'0'));
                    }
                    if vec_string_command.is_empty() == false
                    {
                        for string_command in vec_string_command
                        {
                            if is_valid_cmd(&string_command)
                            {
                                // keep the new commands into actions in order to create corresponding gfx pkt
                                //actions.push(ReadyAction{id: player.id, action: Action::new_from_string(string_command.clone())});
                                player.action_push(string_command);
                            }
                        }
                    }
                }
                //player.print_player_actions();
            }
        }
    }
    //println!("new actions receive -> {:?}", actions);
    //actions
}

/*
**  Iterate onto each actions of each players and retrieve
**  the actions which their `count` == 0 
**
**  return:
**      Vec<ReadyAction>: list of actions ready to execute
**/
fn get_ready_action_list(teams: &Vec<Team>) -> Vec<ReadyAction>
{
    let mut ready_action: Vec<ReadyAction> = Vec::new();

    for team in teams
    {
        for player in &team.players
        {
            // TODO : instead of for loop on player.actions
            // only check player.actions[0] because the others are
            // not running
            for action in &player.actions
            {
                if action.count == 0
                {
                    let action_to_push = ReadyAction{id: player.id, action: action.clone()};
                    ready_action.push(action_to_push);
                }
            }
        }
    }
    //println!("ready actions list --> {:?}", ready_action);
    ready_action
}

/*
// same fucntion as above but more functionnal
fn get_ready_action_list(teams: &[Team]) -> Vec<ReadyAction>
{
    teams
        .iter()
        .flat_map(|team| 
        {
            team.players
                .iter()
                .flat_map(|player|
                {
                    player.actions
                        .iter()
                        .filter(|action| action.count == 0)
                        .map(|action| ReadyAction
                        {
                            id: player.id,
                            action: action.clone(),
                        })
                })
                .collect::<Vec<ReadyAction>>()
        })
        .collect()
}
*/

/*
**  execute action from a ReadyAction 
**/
fn exec_action(ready_action: &ReadyAction, game_ctrl: & mut GameController) -> Option<ActionResult>
{
    let tmp_player = find_player_from_id(game_ctrl.teams.clone(), &ready_action.id);
    let mut player = tmp_player.unwrap();
    // TODO:    checher une autre facon plus propre de faire executer mes actions
    //          ou alors changer les method par des methodes statics 
    let action = Action::new(NO_ACTION);
    let ret = match ready_action.action.action_name.as_str()
    {
        "avance" => ActionResult::ActionBool(action.avance(&game_ctrl.x, &game_ctrl.y, &mut player)),
        "droite" => ActionResult::ActionBool(action.droite(&mut player)),
        "gauche" => ActionResult::ActionBool(action.gauche(&mut player)),
        "voir" => ActionResult::ActionVecHashMap(action.voir(&mut player, &game_ctrl.cells, &game_ctrl.teams)),
        "inventaire" => ActionResult::ActionHashMap(action.inventaire(&mut player)),
        "prend" => ActionResult::ActionBool(action.prend(&mut game_ctrl.cells[player.coord.y as usize][player.coord.x as usize], &mut player, ready_action.action.arg.clone().unwrap())),
        "pose" => ActionResult::ActionBool(action.pose(&mut game_ctrl.cells[player.coord.y as usize][player.coord.x as usize], &mut player, ready_action.action.arg.clone().unwrap())),
        "expulse" => ActionResult::ActionBool(action.expulse(&mut game_ctrl.teams, &player, &game_ctrl.x, &game_ctrl.y)),
        "broadcast" => ActionResult::ActionBool(action.broadcast(&player, &game_ctrl.teams)),
        "incantation" => ActionResult::ActionBool(action.incantation(&player, &mut game_ctrl.teams)),
        "fork" => ActionResult::ActionBool(action.fork(&player, &mut game_ctrl.teams)),
        "connect_nbr" => ActionResult::ActionInt(action.connect_nbr(&player, &game_ctrl.teams)),
        _ => return None,
    };
    // find the index of the executed actions
    // TODO:    normally the ready action is on top of the
    //          player action list, so the index is always 0
    if ready_action.action.action_name != "incantation".to_string()
    {
        let index_action = find_index_action(&ready_action, &player);
        // remove action from player action list
        player.actions.remove(index_action);
    }
    
    // TODO :   find a better way to apply the modification of the
    //          player on the team directly
    for team in &mut game_ctrl.teams
    {
        for team_player in &mut team.players
        {
            if player.id == team_player.id
            {
                team_player.clone_from(&player);
            }
        }
    }
    //println!("exec action {} ---> {:?}", ready_action.action.action_name, ret);
    Some(ret)
}

fn main() -> Result<(), Box<dyn GenericError>> 
{
    let mut gfx_stream: TcpStream;
    let mut stream_hashmap: HashMap<u32, TcpStream> = HashMap::new();
    // use to trigger the execution
    let mut wait_for_answer: bool = true;
    let mut special_actions: Vec<(u32, SpecialActionParam)> = Vec::new();
    //let mut current_actions: Vec<ReadyAction> = Vec::new();
    let mut hashmap: HashMap<String, u8> = HashMap::new();
    let mut id: u32 = 0;
    //let duration: Duration = Duration::new(0, 100000000);

    // parsing
    let mut vec_args = parsing()?;

    // game controller initialization
    let mut game_ctrl = GameController::new(&vec_args);
    //println!("{:#?}", game_ctrl);

    // network initialization
    let listener = TcpListener::bind(format!("127.0.0.1:{}", vec_args.p)).unwrap();
    println!("Start server");

    // listen for client connexion
    for tcpstream in listener.incoming()
    {
        // println!("{:?}", listener.incoming());
        let mut stream = tcpstream?;
        println!("Connection established!");
        
        let _ = stream.write(b"Bienvenue");
        // register the new client
        create_player_or_kick(& mut stream, & mut hashmap, & mut vec_args, & mut id, & mut game_ctrl);
        // set timeout
        let _ = stream.set_read_timeout(Some(Duration::new(0, 10000000)));
        //game_ctrl.stream_hashmap.insert(id, Some(stream));
        stream_hashmap.insert(id, stream);
        if client_all_connect(vec_args.c, vec_args.n.len(), & mut hashmap) { break ; }
    }

    println!("Everybody is connected, let's start the game");
    //println!("vec stream -> {:?}", vec_stream);
    
    // connect to GFX server
    gfx_stream = first_connection_gfx().unwrap();
    // connexion handshake with the GFX server
    send_pkt_to_stream(get_initial_gfx_packets_from_game_ctrl(&game_ctrl), &mut gfx_stream); 
    
    // take initial timestamp
    let start_time = SystemTime::now();
    //println!("start_time ---> {:?}", start_time);

    loop
    {
        for stream in &mut stream_hashmap
        {
            if check_winner(&game_ctrl.teams) { break; }
            if wait_for_answer == true
            {
                let _ = stream.1.write(b"sendme");
                wait_for_answer = false;
            }
            //current_actions = receive_action(stream.1, &mut game_ctrl);
            receive_action(stream.1, &mut game_ctrl);
            break ;
        }

        /*
        // this part is in order to send pkt to gfx & client at the beginning of the receive cmd
        if current_actions.len() > 0
        {
            //println!("current action list --> {:?}", current_actions);
            for current_action in &current_actions
            {
                let gfx_pkt = craft_gfx_packet_action_receive(&current_action, &game_ctrl.teams);
                if let Some(gfx_pkt_tmp) = gfx_pkt
                {
                    send_pkt_to_stream(gfx_pkt_tmp, &mut gfx_stream);
                }

                let client_pkt = craft_client_packet_action_receive(&current_action.action, &game_ctrl.teams);
                if let Some(client_pkt_tmp) = client_pkt
                {
                    // l'action est une incantation, donc je recup les stream concernees par l'incantation
                    // et je boucle pour envoyer a tous ces streams
                    send_pkt_to_stream(client_pkt_tmp, stream_hashmap.get(&current_action.id).unwrap());
                }
            }
        }
        */

        // when command finish to wait, execute action and send packet to client and gfx
        let ready_action_list = get_ready_action_list(&game_ctrl.teams);
        if ready_action_list.len() > 0
        {
            //println!("ready action list --> {:?}", ready_action_list);
            for ready_action in ready_action_list
            {
                // list before = get list des id de tous les joueurs
                let action_result = exec_action(&ready_action, & mut game_ctrl);
                let gfx_pkt = craft_gfx_packet_action_ready(&ready_action, &action_result, &game_ctrl);
                //println!("gfx pkt ready action ---> {:?}", gfx_pkt);
                if let Some(packet) = gfx_pkt 
                {
                    send_pkt_to_stream(packet, &mut gfx_stream);
                }
                let client_pkt = craft_client_packet_action_ready(&ready_action, &action_result, &game_ctrl);
                if let Some(packet) = client_pkt 
                {
                    send_pkt_to_stream(packet, stream_hashmap.get(&ready_action.id).unwrap());

                    //hashmap.add_keys(ready_action.id) == stream(packet)
                }

                //let tmp_id = get_nb_total_players(teams)
                //stream.incoming()
                //{
                //    hashmap.insert(tmp_id, stream);
                //}

                // list after = get list des id de tous les joueurs
                // new list = list after - list before  == nouveau joueurs ne d'un oeuf
                // hashmap.insert(new list) --> on aura un stream = None

                

                // pour le fork, on attend les 42 cycles et on envoie ok au client
                // a ce moment le client cree un nouveau stream directement pour son oeuf
                // le serveur ecoute donc (avec timeout) si y a un stream incomming()

                // on associe le strean incomming au hashmap.id = stream

            }
        }

        //println!("end of get ready action");

        if game_ctrl.update_timestamp(&start_time, vec_args.t)
        {
            println!("timestamp --> {}", game_ctrl.timestamp);
            game_ctrl.print_all_players();
            
            // update game datas (life, counter etc) and retrieve dead players list
            let dead_players = game_ctrl.update_game_datas();

            // send dead pkt to clients and gfx
            let mut pkts = craft_gfx_packet_die(&dead_players);
            if let Some(gfx_pkt_tmp) = pkts
            {
                send_pkt_to_stream(gfx_pkt_tmp, &mut gfx_stream);
            }
            pkts = craft_client_packet_die(&dead_players);
            if let Some(client_pkt_tmp) = pkts
            {
                println!("dead players --> {:?}", dead_players);
                for i in 0..dead_players.len()
                {
                    // ici il faut envoyer le paquet mort a chaque stream concerne
                    send_pkt_to_stream(client_pkt_tmp.clone(), stream_hashmap.get(&dead_players[i].0).unwrap());
                }
            }

            // send gfx pkt for starting fork or incantation
            if let Some(special_actions) = get_pre_actions(&game_ctrl.teams)
            {
                println!("special actions --> {:?}", special_actions);
                pkts = craft_gfx_packet_action_receive(special_actions.clone(), &game_ctrl.teams);
                if let Some(gfx_pkt_tmp) = pkts
                {
                    send_pkt_to_stream(gfx_pkt_tmp, &mut gfx_stream);
                }
                pkts = craft_client_packet_action_receive(&special_actions.clone());
                if let Some(gfx_pkt_tmp) = pkts
                {
                    send_pkt_to_stream(gfx_pkt_tmp, &mut gfx_stream);
                }
            }

            // remove action with count = 0
            for team in &mut game_ctrl.teams
            {
                for player in &mut team.players
                {
                    if player.actions.len() > 0
                    {
                        if player.actions[0].action_name == "incantation".to_string()
                            && player.actions[0].count == 0
                        {
                            println!("chien du chocolat");
                            player.actions.remove(0);
                        }
                    }
                }
            }


            game_ctrl.print_all_players();
            println!("------------------------------------------------------------------------------");
            println!("------------------------------------------------------------------------------");
            println!("------------------------------------------------------------------------------");
            println!("\n");

            //*
            if game_ctrl.timestamp > 303
            {
                use std::process;
                process::exit(0);
            }
            //*/
        }

        //game_ctrl.print_all_players();
    }
    
}

//use std::process;
//process::exit(0);