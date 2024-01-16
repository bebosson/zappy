pub mod Command
{
    use std::{net::TcpStream, io::Write};

    pub struct CommandTemplate
    {
        pub name    : &'static str,
        pub arg     : Option<String>,
        pub count   : u16,
    }

    //pub const NO_Command: CommandTemplate     = CommandTemplate{ name: "",            arg: None,                  count: 0};
    pub const AVANCE: CommandTemplate        = CommandTemplate{ name: "avance",      arg: None,                  count: 7};
    pub const DROITE: CommandTemplate        = CommandTemplate{ name: "droite",      arg: None,                  count: 7};
    pub const GAUCHE: CommandTemplate        = CommandTemplate{ name: "gauche",      arg: None,                  count: 7};
    pub const VOIR: CommandTemplate          = CommandTemplate{ name: "voir",        arg: None,                  count: 7};
    pub const INVENTAIRE: CommandTemplate    = CommandTemplate{ name: "inventaire",  arg: None,                  count: 1};
    pub const PREND: CommandTemplate         = CommandTemplate{ name: "prend",       arg: Some(String::new()),   count: 7};
    pub const POSE: CommandTemplate          = CommandTemplate{ name: "pose",        arg: Some(String::new()),   count: 7};
    pub const EXPULSE: CommandTemplate       = CommandTemplate{ name: "expulse",     arg: None,                  count: 7};
    pub const BROADCAST: CommandTemplate     = CommandTemplate{ name: "broasdcast",  arg: Some(String::new()),   count: 7};
    pub const INCANTATION: CommandTemplate   = CommandTemplate{ name: "incantation", arg: None,                  count: 300};
    pub const FORK: CommandTemplate          = CommandTemplate{ name: "fork",        arg: None,                  count: 42};
    pub const CONNECT_NBR: CommandTemplate   = CommandTemplate{ name: "connect_nbr", arg: None,                  count: 0};

    pub const COMMANDS: [CommandTemplate; 12] = [AVANCE, DROITE, GAUCHE, VOIR, INVENTAIRE, PREND, POSE, EXPULSE, BROADCAST, INCANTATION, FORK, CONNECT_NBR];//, NO_ACTION];

    pub struct Command
    {
        pub name    : &'static str,
        pub arg     : Option<&'static str>,
        pub count   : u16, // 1 Command.count = 7 server count
    }

    impl Command
    {
        pub fn new(command_template: CommandTemplate) -> Self
        {
            Command
            {
                name: command_template.name,
                arg: None,
                count: command_template.count,
            }
        }

        pub fn send(&self, stream: & mut TcpStream/* , vec_string: &Vec<String>, number_command_sent: &mut u8 */)
        {
            let i = 0 as usize;
            let mut send_buf = [48u8; 16];
            for (i, c) in self.name.chars().enumerate()
            {
                send_buf[i] = c as u8;
            }
            println!("send_buf: [{:?}]", send_buf);
            println!("i = {}", i);
            stream.write(&send_buf);
    
   /*
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
    
                    // *number_command_sent += 1;
                }
            }
*/
        }   
    }
}