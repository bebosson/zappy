use std::{net::{TcpListener, TcpStream, Shutdown}, io::{BufReader, BufRead, Write, Read}, time::SystemTime, str::from_utf8, collections::HashMap, env, fmt::Error, error::Error as GenericError, alloc::System};

use args::args::{Args, ParsingError};
use gamecontrol::game::GameController;

//add module in the crate root
pub mod args;
pub mod cell;
pub mod gamecontrol;
pub mod player;
pub mod ressources;
pub mod teams;
pub mod egg;
pub mod zappy;
pub mod action;
pub mod init;

static SERVER_PORT: u16 = 1312;


fn send_bienvenue(stream: & mut TcpStream, msg: &[u8])
{
    // let msg = b"Bienvenue";
    stream.write(msg);
    println!("send bienvenue !");
    
}

fn schrink_buffer(string_schrink: & mut String, buffer: & mut [u8; 32])
{
    // instead of this fucntion there is an memcpy equivalent : dst.clone_from_slice(&src);
    for i in buffer.as_slice()
    {
        if *i == b'\0' {break} //bancale 
        else 
        {
            string_schrink.push(*i as char);
        }
    }
}

fn receive(mut stream: TcpStream, hashmap: & mut HashMap<String, u8>, args: & mut Args, first: &SystemTime, game_ctrl: & mut GameController)
{
    let mut buffer = [0 as u8; 32]; //[a,r,s,e,n,a,l,\0,\0]
    let mut string_schrink: String = String::new();
    let mut id_player = 0;
    
    if let  Ok(_) = stream.read(& mut buffer)  
    {
        schrink_buffer(& mut string_schrink, &mut buffer);
    
        // let mut ref_string_shrink = &string_schrink;
        println!("{:?}", string_schrink);
        match args.n.contains(&string_schrink)
        {
            true => 
            {
                let i = hashmap
                    .entry( string_schrink.clone())
                    .or_insert(0);
                if i == & mut args.c
                {
                    println!("team {:?} is full", string_schrink);
                    //display arsenal/chelsea est full
                    //kick the player
                    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                    // drop(stream)
                }
                else 
                {
                    //create a new id + send 
                    *i += 1;
                    println!("{:?}", SystemTime::now());
                    id_player = first.elapsed().unwrap().as_millis();
                    println!("id = {:?}", id_player);
                    stream.write(id_player.to_string().as_bytes());
                    game_ctrl.get_team_and_push(&string_schrink, id_player);
                    println!("{:#?}", game_ctrl);
                    // gamecontrol.
                    //enregistrer player ? 
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


fn parsing() -> Result<Args, ParsingError> 
{
    let vec_args: Vec<String> = env::args().collect();
    let mut server_arg: Args = Args::new(vec_args)?;
    Ok(server_arg)
}

fn main() -> Result<(), Box<dyn GenericError>> 
{
    let mut table: HashMap<String, u8>= HashMap::new();
    let msg = b"Bienvenue";


    // parsing
    let mut vec_args = parsing()?;
    println!("{:#?}", vec_args);


    // game controller initialization
    let mut game_ctrl = GameController::new(&vec_args);
    println!("{:#?}", game_ctrl);


    // network initialization
    let listener = TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT)).unwrap();
    
    // get current time for game cadency
    let first = SystemTime::now();
    
    // listen for client connexion
    for stream in listener.incoming()
    {
        println!("Connection established!");
        let mut stream_wrt = stream.unwrap();
        let sec = first.elapsed().unwrap().as_secs();
        println!("{:?}", sec);

        send_bienvenue(& mut stream_wrt, msg);
        receive(stream_wrt, & mut table, & mut vec_args, &first, & mut game_ctrl);
        println!("{:?}", table);

        // handle_connection(stream);
    }


    // start game
    // ...


    Ok(())
    
}
