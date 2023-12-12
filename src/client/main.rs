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
    println!("{:?}", data);
}


fn main() {

    let mut args: Vec<String> = env::args().collect();
    let teamname = args[1].clone();
    let mut data = [0 as u8; 9]; // using 6 byte buffer
    
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
                        if &data == b"Bienvenue" 
                        {
                            println!("Reply is ok! Send back teamname = {:?}", teamname);
                            // data.;
                            // stream.write(teamname.as_bytes()).unwrap();
                            stream.write_all(teamname.as_bytes());
                            flush(& mut data);
                            
                            match stream.read(&mut data)
                            {
                                Ok(_) => {
                                    println!("{:?}", &data);
                                } 
                                Err(e) => {
                                    println!("Failed to receive data: {}", e);
                                }  
                            }
                        } else 
                        {
                            let text = from_utf8(&data).unwrap();
                            println!("id = {}", text);
                            // exit(1);
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
