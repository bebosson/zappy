use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write, Read}, time::SystemTime, str::from_utf8, collections::HashMap, env, fmt::Error, error::Error as GenericError};

use args::args::{Args, ParsingError};

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

fn receive(mut stream: TcpStream, hashmap: & mut HashMap<String, u8>, args: & mut Args)
{
    let mut buffer = [0 as u8; 32]; //[a,r,s,e,n,a,l,\0,\0]
    let mut string_schrink: String = String::new();
    
    if let  Ok(_) = stream.read(& mut buffer)  {
        
        for i in buffer.as_slice(){
            if *i == b'\0' {break} //bancale 
            else {
                string_schrink.push(*i as char);
            }
        }
        // let mut ref_string_shrink = &string_schrink;
        println!("{:?}", string_schrink);
        match args.n.contains(&string_schrink){
            true => {
                let i = hashmap
                .entry( string_schrink.clone())
                .or_insert(0);
                *i += 1;
                if i > & mut args.c
                {
                    println!("team {:?} is full", string_schrink);
                    //display arsenal/chelsea est full
                    //couper la connexion 
                }
                //create a new id + send 
            }
            false => {
                println!("bad_entry");
            }
        }
       
    }
    // mut &mut hashtable 
    
}


fn parsing() -> Result<Args, ParsingError> {
    let vec_args: Vec<String> = env::args().collect();
    let mut server_arg: Args = Args::parial_new(vec_args)?;
    Ok(server_arg)
}

fn main() -> Result<(), Box<dyn GenericError>> {
    let mut vec_args = parsing()?;

    // println!("{:?}", team_names);
    let mut table: HashMap<String, u8>= HashMap::new();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT)).unwrap();
    let first = SystemTime::now();
    let msg = b"Bienvenue";
        for stream in listener.incoming() {
            println!("Connection established!");
            let mut stream_wrt = stream.unwrap();
            let sec = first.elapsed().unwrap().as_secs();
            println!("{:?}", sec);

            send_bienvenue(& mut stream_wrt, msg);
            receive(stream_wrt, & mut table, & mut vec_args);
            println!("{:?}", table);
    
            // handle_connection(stream);
    
        }
        Ok(())
    
}
