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
use player::player::Player;
use action::action::ReadyAction;

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
                    game_ctrl.get_team_and_push(&string_teamname_buffer, *id, &stream);
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
    let lala = match buf
    {
        txt if COMMAND_SLICE.iter().any(|&s| s == txt) =>  true,
        txt if COMMAND_SLICE.iter().any(|&s| txt.starts_with(s)) => {
            let mut tmp = buf.split_whitespace();
            let _ = tmp.next();
            is_valid_obj(tmp.next().unwrap())
        },
        _ => false,
    };
    lala
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

fn receive_action(stream: & mut TcpStream, game_ctrl: & mut GameController)
{
    let mut action_receive = [0 as u8; BUF_SIZE];

    //println!("receive action from : {:?}", stream);
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
                                player.action_push(string_command);
                            }
                        }
                    }
                }
                player.print_player_actions();
            }
        }
    }
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

fn find_player_from_id<'a, 'b>(teams: &'a mut Vec<Team>, id: &'b u32) -> Option<&'a mut Player>
{
    for team in teams
    {
        for player in & mut team.players
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

fn exec_action(ready_action: ReadyAction, game_ctrl: & mut GameController) -> bool
{
    let tmp_player = find_player_from_id(& mut game_ctrl.teams, &ready_action.id);

    //println!("INSIDE EXEC_ACTION");

    let player = tmp_player.unwrap();
    let ret = match ready_action.action.action_name.as_str()
    {
        "avance" => player.avance(&game_ctrl.x, &game_ctrl.y),
        "droite" => true,
        "gauche" => true,
        "voir" => true,
        "inventaire" => true,
        "prend" => true,
        "pose" => true,
        "expulse" => true,
        "broadcast" => true,
        "incantation" => true,
        "fork" => true,
        "connect_nbr" => true,
        _ => false,
    };
    let index_action = find_index_action(&ready_action, &player);
    player.actions.remove(index_action);
    ret
}


fn main() -> Result<(), Box<dyn GenericError>> 
{
    let mut vec_stream: Vec<TcpStream> = vec![];
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
        vec_stream.push(stream);
        if client_all_connect(vec_args.c, vec_args.n.len(), & mut hashmap)
        {
           break ;
        }
    }
    
    println!("Everybody is connected, let's start the game");
    println!("-----------------------------------------------------------------------------------------------");
    //println!("{:?}", vec_stream);

    let start_time = SystemTime::now();
    println!("start_time ---> {:?}", start_time);

    loop
    {
        for mut stream in & mut vec_stream
        {
            
            //println!("sendme");
            if check_winner(&game_ctrl.teams)
            {
                break;
            }
            let _ = stream.write(b"sendme");
            receive_action(& mut stream, & mut game_ctrl);
            break ;
        }
        
        //println!("end of tcpStream listener");

        // when command finish to wait, execute action and send packet to client and gfx
        let ready_action_list = get_ready_action_list(&game_ctrl.teams);
        if ready_action_list.len() > 0
        {
            println!("ready action list --> {:?}", ready_action_list);
            for ready_action in ready_action_list
            {
                let action_result = exec_action(ready_action, & mut game_ctrl);
                //let gfx_pkt = craft_gfx_packet(&action_result, &game_ctrl.teams);
                //let client_pkt = craft_client_packet(&action_result, &game_ctrl.teams);
                //let ret = send_pkt(gfx_pkt, client_pkt, GFX_SERVER_PORT);
            }
        }

        //println!("end of get ready action");

        if game_ctrl.update_timestamp(&start_time, vec_args.t)
        {
            game_ctrl.print_all_players();
            println!("timestamp --> {}", game_ctrl.timestamp);
            game_ctrl.update_game_datas();
            println!("\n");
        }

        //game_ctrl.print_all_players();
        //println!("\n\n");
    }
    
}
