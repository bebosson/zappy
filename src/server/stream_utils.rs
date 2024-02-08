pub mod stream_utils
{
    use std::net::TcpStream;
    use std::io::{Write, Read};

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
            if stream.peer_addr().unwrap().port() == 1312
            {
                println!("sending pkt to gfx -----> {}", pkt);
            }
            else
            {
                println!("sending pkt to cli -----> {}", pkt);
            }
            buf = translate_string_to_buffer(pkt);
            let _ = stream.write(&buf);
        }

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