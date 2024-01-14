use std::collections::HashMap;
use std::env;
use std::error::Error as GenericError;
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::time::Duration;
use std::time::SystemTime;

use args::args::{Args, ParsingError};
use gamecontrol::game::GameController;
use teams::team::Team;
use player::player::Player;
use action::action::{ReadyAction, Action, ActionResult, NO_ACTION};
use crate::paket_crafter::paquet_crafter::craft_gfx_packet;


#[derive(Debug)]
pub struct Ourstream{
    pub stream: TcpStream,
    pub wait_for_answer: bool,
}

impl Ourstream{
    pub fn new_from_stream(stream: TcpStream) -> Self
    {
        Ourstream{
            stream,
            wait_for_answer: true,
        }
    }
}

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
                    // display arsenal/chelsea est full
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
    
    
fn parsing() -> Result<Args, ParsingError> 
{
    let vec_args: Vec<String> = env::args().collect();
    let server_arg: Args = Args::new(vec_args)?;
    Ok(server_arg)
}

fn is_valid_obj(object: &str) -> bool
{
    match object
    {
        txt if RESSOURCES_SLICE.iter().any(|&s| s == txt) => true,
        _ => false
    }
} 

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

fn get_obj_from_string(command: &String) -> Option<String>
{
    match command
    {
        command if RESSOURCES_SLICE.iter().any(|&elem| command.ends_with(elem)) => 
        {
            let mut split = command.split_whitespace();
            let object = split.nth(1);
            //println!("object de la mort ---> {:?}", object);
            let tmp = object.unwrap().to_string();
            Some(tmp)
        }
        _ => None
    }
} 

fn receive_action(stream: & mut TcpStream, game_ctrl: & mut GameController) -> Vec<Action>
{
    let mut action_receive = [0 as u8; BUF_SIZE];
    let mut actions : Vec<Action> = Vec::new();

    println!("receive action from : {:?}", stream);
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
                    // println!("AAA");
                    let mut vec_string_command: Vec<String> = Vec::with_capacity(10);
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
                                actions.push(Action::new_from_string(string_command.clone()));
                                player.action_push(string_command);
                               
                            }
                        }
                    }
                }
                //player.print_player_actions();
            }
        }
    }
    //println!("actions -----------> {:?}", actions);
    actions
}


fn get_ready_action_list(teams: &Vec<Team>) -> Vec<ReadyAction>
{
    let mut ready_action: Vec<ReadyAction> = Vec::new();

    for team in teams
    {
        for player in &team.players
        {
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
    //println!("list of ready actions ---> {:?}", ready_action);
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

fn find_player_from_id(teams: Vec<Team>, id: &u32) -> Option<Player>
{
    for team in teams
    {
        for player in team.players
        {
            if id == &player.id
            {
                return Some(player);
            }
        }
    }
    None
}


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

fn exec_action(ready_action: &ReadyAction, game_ctrl: & mut GameController) -> Option<ActionResult>
{
    let tmp_player = find_player_from_id(game_ctrl.teams.clone(), &ready_action.id);

    //println!("INSIDE EXEC_ACTION");

    let mut player = tmp_player.unwrap();
    let action = Action::new(NO_ACTION);
    //let ret = match ready_action.action.action_name.as_str()
    let ret = match ready_action.action.action_name.as_str()
    {
        "avance" => ActionResult::ActionBool(action.avance(&game_ctrl.x, &game_ctrl.y, &mut player)),
        "droite" => ActionResult::ActionBool(action.droite(&mut player)),
        "gauche" => ActionResult::ActionBool(action.gauche(&mut player)),
        "voir" => ActionResult::ActionVecHashMap(action.voir(&mut player, &game_ctrl.cells, game_ctrl.teams.clone())),
        "inventaire" => ActionResult::ActionHashMap(action.inventaire(&mut player)),
        "prend" => ActionResult::ActionBool(action.prend(&mut game_ctrl.cells[player.coord.y as usize][player.coord.x as usize], &mut player, ready_action.action.arg.clone().unwrap())),
        "pose" => ActionResult::ActionBool(action.pose(&mut game_ctrl.cells[player.coord.y as usize][player.coord.x as usize], &mut player, ready_action.action.arg.clone().unwrap())),
        "expulse" => ActionResult::ActionBool(action.expulse(&mut game_ctrl.teams, &player, &game_ctrl.x, &game_ctrl.y)),
        "broadcast" => ActionResult::ActionBool(action.broadcast(&player, &game_ctrl.teams)),
        "incantation" => ActionResult::ActionString(action.incantation(&player, &game_ctrl.teams)),
        "fork" => ActionResult::ActionBool(action.fork(&player, &mut game_ctrl.teams)),
        "connect_nbr" => ActionResult::ActionInt(action.connect_nbr(&player, &game_ctrl.teams)),
        _ => return None,
    };
    let index_action = find_index_action(&ready_action, &player);
    player.actions.remove(index_action);
    
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

    println!("exec action {} ---> {:?}", ready_action.action.action_name, ret);
    
    Some(ret)
}

fn translate_string_to_buffer(gfx_pck_string: String) -> [u8; 32]
{
    let mut array = Vec::with_capacity(32);
    array.extend(gfx_pck_string.chars());
    array.extend(std::iter::repeat('0').take(32 - gfx_pck_string.len()));
    //println!("our fucking array ------------> {}", array);
    
    let mut result_array = [0u8; 32];
    for (i, &c) in array.iter().enumerate()
    {
        result_array[i] = c as u8;
    }
    println!("{:?}", result_array);
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
        all_packets.push(team);
    }
    for team in game_ctrl.packet_gfx_all_players_of_all_teams()
    {
        for player in team
        {
            all_packets.push(player);
        }
    }
    all_packets
}

fn send_to_server_gfx(game_ctrl: &GameController, vec_gfx_pck_string: Vec<String>, stream_gfx: & mut TcpStream)
{
    // println!("{:?}", string_map);
    let mut gfx_packet_to_send: [u8; 32];
   
    for gfx_pck_string in vec_gfx_pck_string
    {
        gfx_packet_to_send = translate_string_to_buffer(gfx_pck_string);
        stream_gfx.write(&gfx_packet_to_send);
    }
        // stream.write(b"BIENVENUE");
}

pub fn first_connection_gfx() -> Option<TcpStream>
{
    match TcpStream::connect("localhost:8080")
    {
        Ok(mut stream) =>
        {
            Some(stream)
        }
        Err(e) => 
        {
            println!("Failed to connect: {}", e);
            None
        }
    } 
}

fn main() -> Result<(), Box<dyn GenericError>> 
{
    let mut vec_our_stream: Vec<Ourstream> = vec![];
    let mut gfx_stream: TcpStream;
    let mut hashmap: HashMap<String, u8>= HashMap::new();
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
    
    //let index = 0; 
    // listen for client connexion
    for tcpstream in listener.incoming()
    {
        // println!("{:?}", listener.incoming());
        let mut stream = tcpstream?;
        println!("Connection established!");
        
        let _ = stream.write(b"Bienvenue");
        create_player_or_kick(& mut stream, & mut hashmap, & mut vec_args, & mut id, & mut game_ctrl);
        let _ = stream.set_read_timeout(Some(Duration::new(0, 10000000)));
        vec_our_stream.push(Ourstream::new_from_stream(stream));
        if client_all_connect(vec_args.c, vec_args.n.len(), & mut hashmap)
        {
           break ;
        }
    }
    
    println!("Everybody is connected, let's start the game");
    //println!("{:?}", vec_stream);

    let start_time = SystemTime::now();
    let mut current_actions: Vec<Action> = Vec::new();
    println!("start_time ---> {:?}", start_time);
    let mut wait_for_answer: bool = true;
    
    /*****             Send init packets \n to server_gfx                  *****/
    /*****             need to get the gfx_stream back from this func                   *****/
    gfx_stream = first_connection_gfx().unwrap();
    send_to_server_gfx(&game_ctrl,  get_initial_gfx_packets_from_game_ctrl(&game_ctrl), &mut gfx_stream); 
    for ourstream in & mut vec_our_stream
    {
        println!("{:?}", ourstream);
    }
    let mut var_tmp: u8 = 0;
    loop
    {
        for ourstream in & mut vec_our_stream
        {
            // println!("sendme");
            if check_winner(&game_ctrl.teams)
            {
                break;
            }
            if ourstream.wait_for_answer == true
            {
                let _ = ourstream.stream.write(b"sendme");
                ourstream.wait_for_answer = false;
                var_tmp += 1;
                //println!("sendme");
            }
            current_actions = receive_action(& mut ourstream.stream, & mut game_ctrl);
            if var_tmp == 3 { break;}
            // break ;
        }
        println!("{:?}", current_actions);
        //println!("end of tcpStream listener");

        // when command finish to wait, execute action and send packet to client and gfx
        let ready_action_list = get_ready_action_list(&game_ctrl.teams);
        if ready_action_list.len() > 0 || current_actions.len() > 0
        {
            println!("current action list --> {:?}", current_actions);

            for current_action in &current_actions
            {
                //let gfx_pkt = pre_craft_gfx_packet(&current_action, &game_ctrl.teams);
                //println!("gfx pre action ---> {}", gfx_pkt.unwrap());
                //let client_pkt = craft_client_packet(&action_result, &game_ctrl.teams);
                //let ret = send_pkt(gfx_pkt, client_pkt, GFX_SERVER_PORT, stream);
            }
            println!("ready action list --> {:?}", ready_action_list);
            
            for ready_action in ready_action_list
            {
                // if ready_action.id == 2 { exit(1); }
                let action_result = exec_action(&ready_action, & mut game_ctrl);
                let gfx_pkt = craft_gfx_packet(&ready_action, &action_result, &game_ctrl); // need to be a option<vec<string>>
                println!("gfx pkt ready action ---> {:?}", gfx_pkt);
                if let Some(packet) = gfx_pkt 
                {
                    send_to_server_gfx(&game_ctrl, packet, &mut gfx_stream);
                }
                //let gfx_pkt = craft_gfx_packet(&action_result, &game_ctrl.teams);
                //let client_pkt = craft_client_packet(&action_result, &game_ctrl.teams);
                //let ret = send_pkt(gfx_pkt, client_pkt, GFX_SERVER_PORT, stream);
            }
        }

        //println!("end of get ready action");

        if game_ctrl.update_timestamp(&start_time, vec_args.t)
        {
            game_ctrl.print_all_players();
            println!("timestamp --> {}", game_ctrl.timestamp);
            game_ctrl.update_game_datas();
            println!("--------------------------------------------------------------\n");
            println!("\n");
        }

        //game_ctrl.print_all_players();
    }
    
}