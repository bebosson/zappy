pub mod parser{

pub enum Parse{
    Map(i32, i32),
    RessourceCase(i32, i32, u8, u8, u8, u8, u8, u8, u8),
    ConnexionPlayer(u8, u8, u8, u8, u8, String),
    MovementPlayer(u8, u8, u8, u8),
    Donothing,
    // Movemement(i32, i32, i32)
}


pub fn copy_until_char(buffer: &[u8], char: u8) -> String
{
    let string_dst = buffer
        .iter() // into_iter 
        .take_while(|&x| *x != char)
        .map(|x| *x as char)
        .collect();
    string_dst
}

pub fn parse_into_integer(content: String) -> Vec<i32>
{
    let mut iter = content.split_ascii_whitespace().skip(1);
    // println!("{:?}", iter);
    let vec : Vec<i32> =  iter.map(|x| x.parse::<i32>().ok().unwrap()).collect();
    vec
}


pub fn parse_ressource(content: String) -> Parse
{
    let vec_parsing = parse_into_integer(content);
    
    let res = Parse::RessourceCase(vec_parsing[0], 
                        vec_parsing[1], 
                        vec_parsing[2] as u8, 
                        vec_parsing[3] as u8, 
                        vec_parsing[4] as u8, 
                        vec_parsing[5] as u8, 
                        vec_parsing[6] as u8, 
                        vec_parsing[7] as u8, 
                        vec_parsing[8] as u8);
    res 
}

pub fn parse_player_movement(content: String) -> Parse
{
    let vec_parsing = parse_into_integer(content);
    let res = Parse::MovementPlayer(vec_parsing[0] as u8, vec_parsing[1] as u8, vec_parsing[2] as u8, vec_parsing[3] as u8);
    res
}

pub fn parse_connexion_player(content: String) -> Parse
{
    let mut vec_parsing_u8: Vec<u8> = vec![];
    let mut team: String = format!("");
    for i in content.split_ascii_whitespace().skip(1).enumerate()
    {
        if i.0 < 5
        {
            // println!("{:?}", i.1);
            vec_parsing_u8.push(i.1.parse::<u8>().ok().unwrap());
        }
        else {
            team = i.1.to_string().clone();
        }
    }
    Parse::ConnexionPlayer(vec_parsing_u8[0], vec_parsing_u8[1], vec_parsing_u8[2], vec_parsing_u8[3], vec_parsing_u8[4], team)
}
// dispatch what you parse 
pub fn parser_server_packet(pkt_receive: String) -> Parse
{
    // println!("{}", pkt_receive);
    let mut iter = pkt_receive.split_ascii_whitespace();
    let mut parse: Parse = Parse::Donothing;
    match iter.nth(0)
    {
        Some(content) => {
            match content{
                "msz" => {
                    parse = take_dim_map(pkt_receive);
                }
                "bct" => {
                    // println!("bct");
                    parse = parse_ressource(pkt_receive);
                }
                "tna" => {
                    todo!();
                }
                "pnw" => {
                    parse = parse_connexion_player(pkt_receive);
                }
                "ppo" => {
                    parse = parse_player_movement(pkt_receive);
                }
                "plv" => {
                    todo!();
                }
                "pin" => {
                    todo!();
                }
                "pex" => {
                    todo!();
                }
                "pic" => {
                    todo!();
                }
                "pie" => {
                    todo!();
                }
                "pfk" => {
                    todo!();
                }
                "pdr" => {
                    todo!();
                }
                "pgt" => {
                    todo!();
                }
                "pdi" => {
                    todo!();
                }
                "enw" => {
                    todo!();
                }
                "eht" => {
                    todo!();
                }
                "ebo" => {
                    todo!();
                }
                "edi" => {
                    todo!();
                }
                "sgt" => {
                    todo!();
                }
                "seg" => {
                    todo!();
                }
                "smg" => {
                    todo!();
                }
                "suc" => {
                    todo!();
                }
                "sbp" => {
                    todo!();
                }
                _ => {
                    parse = Parse::Donothing;
                }
            }
        },
        None => todo!(),
    }
    parse
}


fn take_dim_map(string_map: String) -> Parse
{
    let iter = string_map.split_ascii_whitespace().skip(1);
    let mut vec_map: Vec<i32> = vec![];
    for i in iter
    {
        let string = i;
        // println!("STRING MAP = {:?}", string);
        vec_map.push(string.parse::<i32>().ok().unwrap());
    }
    // let x = vec_map[0].parse::<u32>;
    Parse::Map {0:vec_map[0],1:vec_map[1]}
}
}