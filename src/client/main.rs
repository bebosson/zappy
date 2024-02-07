use std::ops::Deref;
use std::{env, fs};
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::process::{exit, ExitStatus};
use std::str::from_utf8;

const BUF_SIZE: usize = 4096;

fn flush(data: &mut [u8])
{
    for i in & mut *data{
        *i = 0;
    }
    //println!("{:?}", data);
}


fn extract_lines(buffer: &str) -> Vec<String>
{
    buffer.lines().map(String::from).collect()
}

fn send_command(stream: &mut TcpStream, vec_string: &Vec<String>, number_command_sent: &mut u8)
{
    println!("vec string {:?}", vec_string);
    for command in vec_string
    {
        //println!("command {:?}", command);
        if *number_command_sent < vec_string.len() as u8
        {
            //stream.write(command.as_bytes());
            let mut array = Vec::with_capacity(BUF_SIZE / 10);
            array.extend(command.chars());
            array.extend(std::iter::repeat('0').take((BUF_SIZE / 10) - command.len()));
            //println!("our fucking array ------------> {:?}", array);
            
            let mut result_array = [0u8; BUF_SIZE / 10];
            for (i, &c) in array.iter().enumerate()
            {
                result_array[i] = c as u8;
            }
            //println!("result array --> {:?}", result_array);
            stream.write(&result_array);
            // stream.write(b"]");
            // use std::thread::sleep as sleep;
            // use std::time::Duration as dudu;
            // sleep(dudu::from_secs(2));
            *number_command_sent += 1;
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let teamname = args[1].clone();
    let mut data = [0 as u8; BUF_SIZE];
    //println!("{:?}", args);

    let contents = fs::read_to_string(args[2].clone())
        .expect("Should have been able to read the file");
    //println!("{:?}", contents);
    let vec_command = extract_lines(&contents);
    println!("vec command --> {:?}", vec_command);
    let mut number_command_send: u8 = 0;
    
    
    match TcpStream::connect("localhost:1312") 
    {
        Ok(mut stream) =>
        {
            println!("Successfully connected to server in port 1312");
            loop
            {
                match stream.read(&mut data) 
                {
                    Ok(_) => {
                        //convert data(buffer) into string and flush (overkill)
                        let string_buffer = String::from_utf8(data.to_vec()).expect("ok");
                        let string_buffer = string_buffer.trim_matches(char::from(0));  
                        //println!("string buffer --> {:?}", string_buffer);
                        
                        match string_buffer
                        {
                            "Bienvenue" => 
                            {
                                println!("Reply is ok! Send back teamname = {:?}", teamname);
                                // data.;
                                // stream.write(teamname.as_bytes()).unwrap();
                                stream.write(teamname.as_bytes()); // on envoie le teamname
                                flush(& mut data);
                            }
                            "Endconnection" =>
                            {
                                exit(0);
                            }
                            "sendme" => 
                            {
                                send_command(&mut stream, &vec_command, &mut number_command_send);
                                //number_command_send = 0;
                                println!("{}", number_command_send);
                                // let text = from_utf8(&data).unwrap();
                            }
                            _ => 
                            {
                                println!("{:?}", stream);
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
