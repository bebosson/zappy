use std::collections::HashMap;
use std::env;
use std::error::Error as GenericError;
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::ops::Deref;
use std::time::Duration;
use std::time::SystemTime;

use args::args::{Args, ParsingError};
use cell::cell::Cell;
use gamecontrol::game::GameController;
use teams::team::{Team, self};
use player::player::Player;
use action::action::{ReadyAction, Action, ActionResult, NO_ACTION, avance, droite, gauche, voir, inventaire, prend, pose, expulse, broadcast, incantation, fork, connect_nbr};
use crate::paket_crafter::paquet_crafter::{craft_gfx_packet_pre, craft_gfx_packet_post, craft_client_packet};
use crate::player::player::SimplePlayer;


//add module in the crate root
pub mod args;
pub mod cell;
pub mod gamecontrol;
pub mod player;
pub mod ressources;
pub mod teams;
pub mod zappy;
pub mod action;
pub mod init;
pub mod paket_crafter;

static GFX_SERVER_PORT: u16 = 1312;
const COMMAND_SLICE: [&'static str; 12] = ["avance", "droite", "gauche", "voir", "inventaire", "expulse", "incantation", "fork", "connect_nbr", "prend ", "pose ", "broadcast "]; 
const RESSOURCES_SLICE: [&'static str; 7] = ["food", "linemate", "deraumere", "sibure", "mendiane", "phiras", "thystame"];
const BUF_SIZE: usize = 160;


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
 * Simple implementation of cpy_from_slice use for translate the buffer receive 
 * in the stream to the teamname
 * 
 * params:
 *      buffer: [u8; 32]
 * 
 * return:
 *       String
*********************************************************************************/
fn copy_until_char(buffer: &[u8], char: u8) -> String
{
    let string_dst = buffer
        .iter() // into_iter 
        .take_while(|&x| *x != char)
        .map(|x| *x as char)
        .collect();
    string_dst
}


 /***********************************************************************************
 * Check if the packet contains the correct teamname, then proceed our handshake.
 * Kick the player if his team is full and drop the connexion
 * Generate the player in the Gamecontroller structure with a new id
 * 
 * 
 * params:
 *      mut stream: TcpStream
 *      hashmap: &mut HashMap<String, u8>
 *      args: &mut Args
 *      id: &mut u32, 
 *      game_ctrl: &mut GameController
 * 
 * return:
 *       ()
*********************************************************************************/
fn create_player_or_kick(mut stream: TcpStream, hashmap: &mut HashMap<String, u8>, args: &mut Args, id: &mut u32, game_ctrl: &mut GameController)
{
    let mut teamname_buffer = [0 as u8; BUF_SIZE];
    let string_teamname_buffer: String;
    if let  Ok(_) = stream.read(&mut teamname_buffer)  
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
                if nbr_player_in_current_team >= &mut args.c
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
                    let _ = stream.set_read_timeout(Some(Duration::new(0, 10000000)));
                    game_ctrl.get_team_and_push(&string_teamname_buffer, *id, stream, args.x, args.y);
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
fn receive_actions(game_ctrl: &mut GameController) -> Vec<Action>
{
    let mut action_receive = [0 as u8; BUF_SIZE];
    let mut actions : Vec<Action> = Vec::new();

    // println!("receive action from : {:?}", stream);
    for team in &mut game_ctrl.teams
    {
        for player in &mut team.players
        {
            if /* player.port == stream
                                .peer_addr()
                                .unwrap()
                                .port()
                && */player.actions.len() < 11
            {
                if let  Ok(_) = player.stream.read(&mut action_receive)
                {
                    // TODO : attention ici au mecanisme des 10 commandes
                    // si l'une echoue, les suivantes ne seront peut etre plus
                    // pertinentes ! il faudra donc modifier ca pour n'envoyer des cmd
                    // que lorsqu'on a une reponse
                    let mut vec_string_command: Vec<String> = Vec::with_capacity(10);
                    // cut the buffer into 10 segments of 16 bytes to retreive the 10 cmds
                    for i in 0..10
                    {
                        vec_string_command.push(copy_until_char(&action_receive[16 * i..16 * (i+1)], b'0'));
                    }
                    if vec_string_command.is_empty() == false
                    {
                        for string_command in vec_string_command
                        {
                            if is_valid_cmd(&string_command)
                            {
                                // keep the new commands into actions in order to create corresponding gfx pkt
                                actions.push(Action::new_from_string(string_command.clone()));
                                player.action_push(string_command);
                            }
                        }
                    }
                    
                }  

            }
            //player.print_player_actions();
        }
    }
    //println!("new actions receive -> {:?}", actions);
    actions
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

fn get_player_from_id(teams: &mut Vec<Team>, id: u32) -> Option<&mut Player>
{
    for team in teams {
        for player in &mut team.players {
            if id == player.id {
                return Some(player);
            }
        }
    }
    None
}

/*
**  find the ready action in the player actions list
**  params:
**      ready_action: ready action
**      player: player concerned by this ready action
**  return:
**      usize: index of the action to find into player actions list
**/
fn find_index_action(ready_action: &ReadyAction, player: &Player) -> usize
{
    let mut i: usize = 0;

    for action in &player.actions
    {
        if ready_action.action.action_name == action.action_name
            && action.count == 0
        {
            return i;
        }
        i = i + 1;
    }
    i
}

/*
**  execute action from a ReadyAction 
**/
// reste faire la premiere partie de l'incantation
// debut incantation -> others / fin -> self ???
fn action_on_self_or_cells(ready_action: &ReadyAction, player: &mut Player, cells: &mut Vec<Vec<Cell>>, width: u8, height: u8) -> (Option<ActionResult>, Option<SimplePlayer>)
{
    let ret = match ready_action.action.action_name.as_str()
    {
        // actions sur self
        "avance" => ActionResult::ActionBool(avance(width, height, player)),
        "droite" => ActionResult::ActionBool(droite(player)),
        "gauche" => ActionResult::ActionBool(gauche(player)),
        "inventaire" => ActionResult::ActionHashMap(inventaire(player)),
        "prend" => ActionResult::ActionBool(prend(&mut cells[player.coord.y as usize][player.coord.x as usize], player, ready_action.action.arg.as_ref().unwrap())),
        "pose" => ActionResult::ActionBool(pose(&mut cells[player.coord.y as usize][player.coord.x as usize], player, ready_action.action.arg.as_ref().unwrap())),
        // actions sur les autres players
        "voir" => ActionResult::ActionOthers,
        "expulse" =>  ActionResult::ActionOthers,
        "broadcast" =>  ActionResult::ActionOthers,
        "incantation" =>  ActionResult::ActionOthers,
        "fork" =>  ActionResult::ActionOthers,
        "connect_nbr" =>  ActionResult::ActionOthers,
        _ => return (None, None),
    };
    // // find the index of the executed actions
    // // TODO:    normally the ready action is on top of the
    // //          player action list, so the index is always 0
    let index_action = find_index_action(&ready_action, player);
    // // remove action from player action list
    player.actions.remove(index_action);
    
    let simple_player = SimplePlayer::new(&player);
    (Some(ret), Some(simple_player))
    // Some(ActionResult::ActionBool((true)))
}

fn action_on_others(ready_action: &ReadyAction, teams: &mut Vec<Team>, cells: &Vec<Vec<Cell>>, simple_player: &SimplePlayer, width: u8, height: u8) -> Option<ActionResult>
{
    let ret = match ready_action.action.action_name.as_str()
    {
        "voir" => ActionResult::ActionVecHashMap(voir(&simple_player, cells, teams)),
        "expulse" => ActionResult::ActionBool(expulse(teams, simple_player, width, height)),
        "broadcast" => ActionResult::ActionBool(broadcast(simple_player, teams)),
        "incantation" => ActionResult::ActionString(incantation(simple_player, teams)),
        "fork" => ActionResult::ActionBool(fork(simple_player, teams)),
        "connect_nbr" => ActionResult::ActionInt(connect_nbr(simple_player, teams)),
        _ => return None,
    };
    Some(ret)
}

fn translate_string_to_buffer(gfx_pck_string: String) -> [u8; 32]
{
    let mut array = Vec::with_capacity(32);
    array.extend(gfx_pck_string.chars());
    array.extend(std::iter::repeat('0').take(32 - gfx_pck_string.len()));
    
    let mut result_array = [0u8; 32];
    for (i, &c) in array.iter().enumerate()
    {
        result_array[i] = c as u8;
    }
    //println!("result array --> {:?}", result_array);
    result_array
}

fn get_initial_gfx_packets_from_game_ctrl(game_ctrl: &GameController) -> Vec<String>
{
    let mut all_packets : Vec<String> = vec![];
    all_packets.push(game_ctrl.packet_gfx_map_size());
    // all_packets.push(game_ctrl.packet_gfx_timestamp());
    for i in game_ctrl.packet_gfx_ressources_map()
    {
        all_packets.push(i);
    }
    for team in game_ctrl.packet_gfx_all_teams()
    {
        for player in team
        {
            all_packets.push(player);
        }
    }
    all_packets
}

fn send_to_server_gfx(gfx_pkts: Vec<String>, stream_gfx: &mut TcpStream)
{
    //println!("packet gfx to send -> {:?}", gfx_pkts);
    let mut buf: [u8; 32];
   
    for gfx_pkt in gfx_pkts
    {
        buf = translate_string_to_buffer(gfx_pkt);
        let _ = stream_gfx.write(&buf);
    }
}

pub fn first_connection_gfx() -> Option<TcpStream>
{
    match TcpStream::connect("localhost:8080")
    {
        Ok(stream) => Some(stream),
        Err(e) => 
        {
            println!("Failed to connect: {}", e);
            None
        }
    } 
}

// fn initialize_connections(listener: TcpListener)
// {
//     for tcpstream in listener.incoming()
//     {
//         // println!("{:?}", listener.incoming());
//         let mut stream = tcpstream?;
//         println!("Connection established!");
        
//         let _ = stream.write(b"Bienvenue");
//         // register the new client
//         create_player_or_kick(stream, &mut hashmap, &mut vec_args, &mut id, &mut game_ctrl);
//         // set timeout
        
//         // vec_stream.push(stream);
//         if client_all_connect(vec_args.c, vec_args.n.len(), &mut hashmap) { break ; }
//     }
// }

fn exec_and_send(ready_action: &ReadyAction, game_ctrl: &mut GameController, id: &mut u32, gfx_stream: &mut TcpStream)
{
    /*****************************************************************\
     * Nouvelle logique:
     * On separe les actions entre
     *      1/ action sur le player qui execute l'action et/ou les cells
     *      2/ action sur les autres players
    \*****************************************************************/ 
    let mut player = get_player_from_id(&mut game_ctrl.teams, *id).unwrap();
    let (option_action, option_player) = action_on_self_or_cells(&ready_action, &mut player, &mut game_ctrl.cells, game_ctrl.x, game_ctrl.y);
    let mut action_result = option_action.unwrap();
    let simple_player = option_player.unwrap();
    if action_result == ActionResult::ActionOthers {
        action_result = action_on_others(&ready_action, &mut game_ctrl.teams, &game_ctrl.cells, &simple_player, game_ctrl.x, game_ctrl.y).unwrap();
    }
    // deuxieme appel a get_player_from_id a cause des borrows mut, y a peut etre mieux a faire ?
    let client_pkt = craft_client_packet(&action_result);
    let player = get_player_from_id(&mut game_ctrl.teams, *id).unwrap();
    let _ = player.stream.write(b"ok"); // on a a nouveau le meme probleme qu'avant dans exec_action pour trouver le player vu qu'on a pas le stream du player
    
    let gfx_pkt = craft_gfx_packet_post(&ready_action, &action_result, &game_ctrl, &simple_player);
    println!("gfx pkt ready action ---> {:?}", gfx_pkt);
    if let Some(packet) = gfx_pkt {
        send_to_server_gfx(packet, gfx_stream);
    }
}

fn main() -> Result<(), Box<dyn GenericError>> 
{
    let mut gfx_stream: TcpStream;
    // use to trigger the execution
    // TODO : remplacer plus tard par un autre mechanisme ou on a pas besoin de ca
    // let mut vec_stream: Vec<TcpStream> = Vec::new();
    // let mut current_actions: Vec<Action> = Vec::new();
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
        create_player_or_kick(stream, &mut hashmap, &mut vec_args, &mut id, &mut game_ctrl);
        // set timeout
        
        // vec_stream.push(stream);
        if client_all_connect(vec_args.c, vec_args.n.len(), &mut hashmap) { break ; }
    }
    
    println!("Everybody is connected, let's start the game");
    //println!("vec stream -> {:?}", vec_stream);

    // take initial timestamp
    let start_time = SystemTime::now();
    //println!("start_time ---> {:?}", start_time);
    
    // connect to GFX server
    gfx_stream = first_connection_gfx().unwrap();
    // connexion handshake with the GFX server
    send_to_server_gfx(get_initial_gfx_packets_from_game_ctrl(&game_ctrl), &mut gfx_stream); 
    
    loop
    {
        if check_winner(&game_ctrl.teams) { return Ok(()); } // provisoire
        let current_actions = receive_actions(&mut game_ctrl);

        // when command finish to wait, execute action and send packet to client and gfx
        let ready_action_list = get_ready_action_list(&game_ctrl.teams);
        if ready_action_list.len() > 0 || current_actions.len() > 0
        {
            println!("current action list --> {:?}", current_actions);
            // for current_action in &current_actions
            // {
            //     let gfx_pkt = craft_gfx_packet_pre(&current_action, &game_ctrl.teams);
            //     //println!("gfx pre action ---> {}", gfx_pkt.unwrap());
            //     //let ret = send_pkt(gfx_pkt, None, GFX_SERVER_PORT, stream);
            // }
            println!("ready action list --> {:?}", ready_action_list);
            for ready_action in ready_action_list {
                exec_and_send(&ready_action, &mut game_ctrl, &mut id, &mut gfx_stream);
            }
        }

        //println!("end of get ready action");

        if game_ctrl.update_timestamp(&start_time, vec_args.t)
        {
            game_ctrl.print_all_players();
            println!("timestamp --> {}", game_ctrl.timestamp);
            game_ctrl.update_game_datas();
            println!("------------------------------------------------------------------------------");
            println!("------------------------------------------------------------------------------");
            println!("------------------------------------------------------------------------------");
            println!("\n");
        }

        //game_ctrl.print_all_players();
    }
    
}