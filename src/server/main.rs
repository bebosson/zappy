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

static SERVER_PORT: u16 = 1312;



fn send_bienvenue(stream: & mut TcpStream, msg: &[u8])
{
    // let msg = b"Bienvenue";
    stream.write(msg);
    println!("send bienvenue !");
    
}

fn schrink_buffer(string_schrink: & mut String, buffer: & mut [u8; 32])
{
    for i in buffer.as_slice()
    {
        if *i == b'\0' {break} //bancale 
        else 
        {
            string_schrink.push(*i as char);
        }
    }
}

fn receive(mut stream: TcpStream, hashmap: & mut HashMap<String, u8>, args: & mut Args, id: & mut u32, game_ctrl: & mut GameController)
{
    let mut buffer = [0 as u8; 32]; //[a,r,s,e,n,a,l,\0,\0]
    let mut string_schrink: String = String::new();
    
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
                    stream.write("Endconnection".to_string().as_bytes());
                    //display arsenal/chelsea est full
                    //kick the player
                    drop(stream)
                }
                else 
                {
                    //create a new id + send 
                    *i += 1;
                    println!("{:?}", SystemTime::now());
                    *id += 1;
                    println!("id = {:?}", *id);
                    stream.write(&*id.to_string().as_bytes());
                    game_ctrl.get_team_and_push(&string_schrink, *id);
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
    let mut server_arg: Args = Args::parial_new(vec_args)?;
    Ok(server_arg)
}



fn main() -> Result<(), Box<dyn GenericError>> 
{
    let mut vec_args = parsing()?;
    println!("{:#?}", vec_args);
    let mut id: u32 = 0;
    let mut game_ctrl = GameController::new(&vec_args);
    println!("{:#?}", game_ctrl);
    // println!("{:?}", team_names);
    let mut hashmap: HashMap<String, u8>= HashMap::new();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT)).unwrap();
    let msg = b"Bienvenue";
        for tcpstream in listener.incoming() {
            println!("Connection established!");
            let mut stream = tcpstream?;
            send_bienvenue(& mut stream, msg);
            if vec_args.client_all_connect(& mut hashmap) == false
            {
                receive(stream, & mut hashmap, & mut vec_args, & mut id, & mut game_ctrl);
            }
            else 
            {
                println!("all teams are full");
                stream.write("Endconnection".to_string().as_bytes())?;
                drop(stream);
            }
    
            // handle_connection(stream);
    
        }
        Ok(())
    
}
