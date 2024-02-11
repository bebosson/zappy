pub mod stream_utils
{
    use std::io::{Write, Read};
    use std::collections::HashMap;
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use std::time::Duration;

    use crate::{BUF_SIZE, GFX_SERVER_PORT};
    use crate::gamecontrol::game::GameController;


    /*
    **  Convert String into buffer to send to gfx or client stream
    **/
    fn translate_string_to_buffer(gfx_pck_string: String) -> [u8; BUF_SIZE]
    {
        let mut array = Vec::with_capacity(BUF_SIZE);
        array.extend(gfx_pck_string.chars());
        array.extend(std::iter::repeat('0').take(BUF_SIZE - gfx_pck_string.len()));
        
        let mut result_array = [0u8; BUF_SIZE];
        for (i, &c) in array.iter().enumerate()
        {
            result_array[i] = c as u8;
        }
        //println!("result array --> {:?}", result_array);
        result_array
    }

    /*
    **  Send a list of string to a TCP stream
    **  params:
    **      pkts: string vector that contains the packet to send
    **      stream: destination of the TCP packets list
    **/
    pub fn send_pkt_to_stream(pkts: Vec<String>, mut stream: &TcpStream)
    {
        let mut buf: [u8; BUF_SIZE];
    
        for pkt in pkts
        {
            //println!("stream port ---> {}", stream.peer_addr().unwrap().port());
            if stream.peer_addr().unwrap().port() == GFX_SERVER_PORT
            {
                println!("sending pkt to gfx --> {}", pkt);
            }
            else
            {
                println!("sending pkt to cli --> {}", pkt);
            }
            buf = translate_string_to_buffer(pkt);
            let _ = stream.write(&buf);
        }
    }


    /*
    **  Tente d'ecouter les nouvelles connexions entrantes apres 
    **  l'eclosion d'un oeuf, pour l'instant il y a 10 tentatives 
    **  avant qu'on dise que la connexion a echoué (et on passe en mode non bloquant
    **  avec 10ms de sleep)
    */
    pub fn get_new_connexion(id: u32, listener: &TcpListener) -> Option<HashMap<u32, TcpStream>>
    {
        let mut new_connexion: HashMap<u32, TcpStream> = HashMap::new();

        if let Ok(_nb) = listener.set_nonblocking(true)
        {
            let mut i = 0;
            for tcpstream in listener.incoming()
            {
                thread::sleep(Duration::from_millis(10));
                if i > 9 { break ; } // on laisse 10 try au client pour l'y connecter au server
                match tcpstream
                {
                    Ok(stream) =>
                    {
                        //println!("new connexion is ---------> {:?}", stream);
                        new_connexion.insert(id, stream);
                    },
                    Err(_) =>
                    {
                        println!("no stream found");
                    }
                }
                i += 1;
            }
            let _ = listener.set_nonblocking(false);
        }
        
        if new_connexion.is_empty() { return None; }
        Some(new_connexion)
    }



    pub fn first_connection_gfx() -> Option<TcpStream>
    {
        let gfx_ip = format!("localhost:{}", GFX_SERVER_PORT);
        match TcpStream::connect(gfx_ip)
        {
            Ok(stream) => Some(stream),
            Err(e) => 
            {
                println!("Failed to connect: {}", e);
                None
            }
        } 
    }

    pub fn get_initial_gfx_packets_from_game_ctrl(game_ctrl: &GameController) -> Vec<String>
    {
        let mut all_packets : Vec<String> = vec![];
        all_packets.push(game_ctrl.packet_gfx_map_size());
        // all_packets.push(game_ctrl.packet_gfx_timestamp());
        for i in game_ctrl.packet_gfx_ressources_map()
        {
            all_packets.push(i);
        }
        for team in game_ctrl.packet_gfx_all_teams()
        {
            for player in team
            {
                all_packets.push(player);
            }
        }
        all_packets
    }
}