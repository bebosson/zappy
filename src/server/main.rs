use std::collections::HashMap;
use std::env;
use std::error::Error as GenericError;
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use args::args::{Args, ParsingError};
use gamecontrol::game::GameController;
use teams::team::Team;
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
use std::sync::Arc;
const COMMAND_SLICE: [&'static str; 12] = ["avance", "droite", "gauche", "voir", "inventaire", "expulse", "incantation", "fork", "connect_nbr", "prend ", "pose ", "broadcast "]; 
const RESSOURCES_SLICE: [&'static str; 7] = ["food", "linemate", "deraumere", "sibure", "mendiane", "phiras", "thystame"]; 

fn check_winner(_teams: &Vec<Team>) -> bool
{
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
fn cpy_from_slice(buffer: [u8; 32]) -> String
{
    let string_dst = buffer
        .iter() // into_iter 
        .take_while(|&x| *x != b'\0')
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
    let mut teamname_buffer = [0 as u8; 32];
    let string_teamname_buffer: String;
    if let  Ok(_) = stream.read(& mut teamname_buffer)  
    {
        string_teamname_buffer = cpy_from_slice(teamname_buffer);
        match args.n.contains(&string_teamname_buffer)
        {
            true => 
            {
                //Add the receive teamname to the hashtable and verify if the team is full or not
                let nbr_player_in_current_team =  hashmap
                    .entry( string_teamname_buffer.clone())
                    .or_insert(0);

                //compare the teamnames received with the teamnames parsed
                if nbr_player_in_current_team >= & mut args.c
                {
                    //display arsenal/chelsea est full
                    //send the Endconnection to kill the client
                    //kick the player
                    println!("team {:?} is full", string_teamname_buffer);
                    let _ = stream.write("Endconnection".to_string().as_bytes());
                    drop(stream)
                }
                else 
                {
                    //create a new id
                    //send back the id to the client
                    //save the player 
                    *nbr_player_in_current_team += 1;
                    *id += 1;
                    let _ = stream.write(&id.to_string().as_bytes());
                    game_ctrl.get_team_and_push(&string_teamname_buffer, *id);
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

fn is_valid_obj(object: &str)
{
    match object
    {
        txt if RESSOURCES_SLICE.iter().any(|&s| s == txt) => (println!("{} ok", txt)),
        _ => println!("it is ta mere")
    }
} 

fn is_valid_cmd(buf: &str)
{
    match buf
    {
        txt if COMMAND_SLICE.iter().any(|&s| s == txt) => println!("{} ok", txt),
        txt if RESSOURCES_SLICE.iter().any(|&s| txt.starts_with(s)) => {
            println!("{} ok", txt);
            is_valid_obj(&buf[txt.len()..])
        }
        _ => println!("{} PAS OK", buf) ,
        // txt if txt.starts_with("prend ") => { print!("prend ok => "); is_valid_obj(&buf[6..]);},
        // txt if txt.starts_with("pose ") => { print!("pose ok => "); is_valid_obj(&buf[5..]);},
        // txt if txt.starts_with("broadcast ") => println!("broadcast ok"),
    }
}

fn test_receive_action(stream: & mut TcpStream)
{
    let mut action_receive = [0 as u8; 32];
    println!("{:?}", stream);
    if let  Ok(_) = stream.read(& mut action_receive)  
    {
        // let string_teamname_buffer: String;
        // string_teamname_buffer = cpy_from_slice(action_receive);
        println!("{:?}", action_receive);
        let vec_string_command: Vec<String> = action_receive
            .split(|&b| b == b'\0') // Divise la slice à chaque zéro
            .map(|subslice| String::from_utf8_lossy(subslice).into_owned()) // transforme la slice de byte en string
            .collect(); // constitue le vec<string>

        println!("{:?}", vec_string_command);
        vec_string_command.iter().map(|s| is_valid_cmd(&s)).for_each(drop);
        // is_valid_cmd()
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
    println!("list of ready actions ---> {:?}", ready_action);
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
                    player
                        .actions
                        .iter()
                        .filter(|action| action.count == 0)
                        .map(move |action| ReadyAction
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


fn exec_action(ready_action: ReadyAction) -> Option<String>
{
    println!("{}", ready_action.id);
    Some(String::new())
}


fn main() -> Result<(), Box<dyn GenericError>> 
{
    let mut vec_stream: Vec<TcpStream> = vec![];
    let mut hashmap: HashMap<String, u8>= HashMap::new();
    let mut id: u32 = 0;
    let duration: Duration = Duration::new(0, 100000000);

    println!("Start server");

    // parsing
    let mut vec_args = parsing()?;


    // game controller initialization
    let mut game_ctrl = GameController::new(&vec_args);
    //println!("{:#?}", game_ctrl);


    // network initialization
    let listener = TcpListener::bind(format!("127.0.0.1:{}", vec_args.p)).unwrap();
    
    let index = 0; 
    // listen for client connexion
    for tcpstream in listener.incoming()
    {
        
        // println!("{:?}", listener.incoming());
        let mut stream = tcpstream?;
        println!("Connection established!");
        println!("{:?}", stream);
        
        stream.write(b"Bienvenue");
        create_player_or_kick(& mut stream, & mut hashmap, & mut vec_args, & mut id, & mut game_ctrl);
        vec_stream.push(stream);
        if client_all_connect(vec_args.c, vec_args.n.len(), & mut hashmap)
        {
           break ;
        }
    }
    println!("Everybody is connected, let's start the game");
    loop
    {
        // ?.0.set_read_timeout(Some(duration))?;
        // let test = listener.accept()?;
    
        for mut stream in & mut vec_stream
        {
            
            println!("sendme");
            if check_winner(&game_ctrl.teams)
            {
                break;
            }
            stream.write(b"sendme");
            
            // println!("{:?}", listener.incoming());
            test_receive_action(& mut stream);
            // println!("{:?}", stream);
            // parti de Julien
        }

        // parti de Julien : recv pkt + check valid + attach Action au player
        
        // when command finish to wait, execute action and send packet to client and gfx
        let ready_action_list = get_ready_action_list(&game_ctrl.teams);
        if ready_action_list.len() > 0
        {
            for ready_action in ready_action_list
            {
                let action_result = exec_action(ready_action);
                //let gfx_pkt = craft_gfx_packet(&action_result, &game_ctrl.teams);
                //let client_pkt = craft_client_packet(&action_result, &game_ctrl.teams);
                //let ret = send_pkt(gfx_pkt, client_pkt);
            }
        }
    }
    // marquer timestamp
    


    Ok(())
    
}
