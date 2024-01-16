pub mod cell;
pub mod command;

use std::time::Duration;
//use std::ops::Deref;
use std::{env, fs};
use std::net::TcpStream;
use std::io::{Read, Write};
// use std::process::{exit, ExitStatus};
use std::process::exit;

use bevy::transform::commands;

use crate::command::Command::{Command, AVANCE};
//use std::str::from_utf8;

const BUF_SIZE: usize = 160;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let teamname = args[1].clone();

/*****************************************************************************/
// champion dans fichier texte -> PROVISOIRE
    let tmp_text = fs::read_to_string(args[2].clone())
                                    .expect("Should have been able to read the file");
    let vec_command = extract_lines(&tmp_text);
    println!("vec command --> {:?}", vec_command);
/*****************************************************************************/
    
    let mut stream = TcpStream::connect("localhost:1312")
                                                .expect("Couldn't connect to the server");
    stream.set_read_timeout(Some(Duration::new(1, 0)))
            .expect("set_read_timeout call failed");
    let mut data = [0 as u8; BUF_SIZE]; // using 6 byte buffer
    connect_to_server(& mut stream, & mut data, &teamname);
    
    player_exec(& mut stream, & mut data);

    println!("Terminated.");
}

fn extract_lines(buffer: &str) -> Vec<String>
{
    buffer.lines().map(String::from).collect()
}

fn flush(data: &mut [u8])
{
    for i in & mut *data{
        *i = 0;
    }
    //println!("{:?}", data);
}

fn connect_to_server(stream: & mut TcpStream, data: & mut [u8], teamname: &String)
{
    loop 
    {
        if let Ok(_) = stream.read(data)
        {
            let string_buffer = String::from_utf8(data.to_vec()).expect("ok"); // expect ok BOF
            let string_buffer = string_buffer.trim_matches(char::from(0));
            if let "Bienvenue" = string_buffer
            {
                println!("Reply is ok! Send back teamname = {:?}", teamname);
                stream.write(teamname.as_bytes()); // on envoie le teamname
                flush(data);
                break;
            }
        }       
    }

}

fn player_exec(stream: & mut TcpStream, data: & mut [u8])
{
    let mut cmd_snt = 0;
    loop
    {
        println!("rentre dans la boucle");
        if let Ok(_) = stream.read(data)
        {
            //convert data(buffer) into string and flush (overkill)
            let string_buffer = String::from_utf8(data.to_vec()).expect("ok");
            let string_buffer = string_buffer.trim_matches(char::from(0));  
            //println!("string buffer --> {:?}", string_buffer);
            
            match string_buffer
            {
                "ok" =>
                {
                    cmd_snt -= 1;
                    println!("ACTION EXECUTEE");
                },
                "Endconnection" =>
                {
                    exit(0);
                },
                _ => 
                {
                    // println!("Failed to receive data: {}", e);
                    ;
                },
            } 
        }
        if cmd_snt < 10
        {
            //send_command(&mut stream, &vec_command, &mut number_command_send);
            println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
            let command = Command::new(AVANCE);
            command.send(stream);
            println!("ACTION ENVOYEE");
            println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
            cmd_snt += 1;
        }
        print!("cmd_snt = {}", cmd_snt);
    }
}

/*
fn send_command(stream: &mut TcpStream, vec_string: &Vec<String>, number_command_sent: &mut u8)
{
    for command in vec_string
    {
        //println!("vec string {:?}", vec_string);
        if *number_command_sent < vec_string.len() as u8
        {
            //stream.write(command.as_bytes());
            let mut array = Vec::with_capacity(16);
            array.extend(command.chars());
            array.extend(std::iter::repeat('0').take(16 - command.len()));
            //println!("our fucking array ------------> {:?}", array);
            
            let mut result_array = [0u8; 16];
            for (i, &c) in array.iter().enumerate()
            {
                result_array[i] = c as u8;
            }
            println!("{:?}", result_array);
            stream.write(&result_array);
            // stream.write(b"]");
            // use std::thread::sleep as sleep;
            // use std::time::Duration as dudu;
            // sleep(dudu::from_secs(2));
            *number_command_sent += 1;
        }
    }
}
*/