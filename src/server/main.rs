use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write, Read}, time::SystemTime, str::from_utf8, collections::HashMap, env};

use args::args::Args;

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


fn handle_connection(mut stream: TcpStream) {
    println!("here");
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}


fn send_bienvenue(stream: & mut TcpStream, msg: &[u8])
{
    // let msg = b"Bienvenue";
    stream.write(msg);
    println!("send bienvenue !");
    
}

fn receive(mut stream: TcpStream, hashmap: & mut HashMap<String, u8>, team_names: &Vec<String>)
{
    let mut buffer = [0 as u8; 32];
    let mut string_schrink: String = String::new();
    if let  Ok(_) = stream.read(& mut buffer)  {

        for i in buffer.as_slice(){
            if *i == b'\0' {break;} //bancale 
            else {
                string_schrink.push(*i as char);
            }
        }
        println!("{:?}", string_schrink);
        match team_names.contains(&string_schrink){
            true => {
                let i = hashmap
                .entry( string_schrink)
                .or_insert(0);
                *i += 1;
            }
            false => {
                println!("bad_entry");
            }
        }
       
    }
    // mut &mut hashtable 
    
}
/*let now = SystemTime::now();

   // we sleep for 2 seconds
   sleep(Duration::new(2, 0));
   match now.elapsed() {
       Ok(elapsed) => {
           // it prints '2'
           println!("{}", elapsed.as_secs());
       }
       Err(e) => {
           // an error occurred!
           println!("Error: {e:?}");
       }
   } */
   // int toto;
// int *ptr_toto = &toto;
// ptr_toto += 1; toto 


fn main() {
    let vec_args: Vec<String> = env::args().collect();
    let server_arg: Args = Args::parial_new(vec_args);
    // println!("{:?}", team_names);
    let mut table: HashMap<String, u8>= HashMap::new();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let first = SystemTime::now();
    let msg = b"Bienvenue";
        for stream in listener.incoming() {
            println!("Connection established!");
            let mut stream_wrt = stream.unwrap();
            let sec = first.elapsed().unwrap().as_secs();
            println!("{:?}", sec);

            // if sec % 5 == 0{
                // println!("{:?}", sec);
                send_bienvenue(& mut stream_wrt, msg);
                receive(stream_wrt, & mut table, &server_arg.n);
                println!("{:?}", table);
            // }
    
            // handle_connection(stream);
    
        }
    
}
