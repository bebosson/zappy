use std::ops::Deref;
use std::{env, fs};
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


fn extract_lines(buffer: &str) -> Vec<String> {
    buffer.lines().map(String::from).collect()
}

fn send_command(stream: &mut TcpStream, vec_string: &Vec<String>, number_command_sent: &mut u8)
{
    for command in vec_string{
        stream.write(command.as_bytes());
        *number_command_sent += 1;
        if *number_command_sent > 3 { break ;}
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let teamname = args[1].clone();
    let mut data = [0 as u8; 256]; // using 6 byte buffer
    println!("{:?}", args);

    let contents = fs::read_to_string("test/command.txt")
        .expect("Should have been able to read the file");
    println!("{:?}", contents);
    let vec_command = extract_lines(&contents);
    println!("{:?}", vec_command);
    let mut number_command_send: u8 = 0;
    
    
    match TcpStream::connect("localhost:1312") 
    {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            loop
            {
                match stream.read(&mut data) 
                {
                    Ok(_) => {
                        //convert data(buffer) into string and flush (overkill)
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
                                println!("id = {}", string_buffer);
                                send_command(&mut stream, &vec_command, &mut number_command_send);
                                
                                // let text = from_utf8(&data).unwrap();
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
