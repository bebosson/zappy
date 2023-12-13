use std::env;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::process::exit;
use std::str::from_utf8;


fn flush(data: &mut [u8])
{
    for i in & mut *data{
        *i = 0;
    }
    //println!("{:?}", data);
}



fn main() {

    let args: Vec<String> = env::args().collect();
    let teamname = args[1].clone();
    let mut data = [0 as u8; 256]; // using 6 byte buffer
    println!("{:?}", args);
    
    
    match TcpStream::connect("localhost:1312") 
    {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            // let msg = teamname;

            // stream.write(msg).unwrap();
            // stream.write(msg).unwrap();
            // println!("Sent Hello, awaiting reply...");
            
            loop
            {
                match stream.read(&mut data) 
                {
                    Ok(_) => {
                        // let our_str = from_utf8(&data).unwrap();
                        let string_buffer = String::from_utf8(data.to_vec()).expect("ok");
                        let string_buffer = string_buffer.trim_matches(char::from(0));
                        
                        match string_buffer
                        {
                            "Bienvenue" => 
                            {
                                println!("Reply is ok! Send back teamname = {:?}", teamname);
                                // data.;
                                // stream.write(teamname.as_bytes()).unwrap();
                                stream.write_all(teamname.as_bytes()); // on envoie le teamname
                                flush(& mut data);
                            }
                            "Endconnection" =>
                            {
                                exit(0);
                            }
                            _ => 
                            {
                                // let text = from_utf8(&data).unwrap();
                                println!("id = {}", string_buffer);
                            }   
                        } 
                    },
                    Err(e) => 
                    {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        },
        Err(e) => 
        {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");

}
