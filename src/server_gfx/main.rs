mod map;
pub mod sprite_player;
mod Ressource;

use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{WindowTheme, PresentMode};
use bevy_pancam::PanCamPlugin;
use crossbeam_channel::bounded;
use map::map::{spawn_map, TilesPlugin, Map};
use rand::SeedableRng;
use rand::rngs::StdRng;
use sprite_player::sprite_player::{sprite_movement, setup_sprite, animate_sprite};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::SplitAsciiWhitespace;
use std::thread;

use bevy::prelude::*;
// Using crossbeam_channel instead of std as std `Receiver` is `!Sync`
use crossbeam_channel::{Receiver};
use rand::{Rng};
use std::time::{Duration, Instant};

const MAP_WIDTH: f32 = 50.;
const MAP_HEIGHT: f32 = 50.0;
const TILES_WIDTH: f32 = 100.0;
const TILES_HEIGHT: f32 = 100.0;
const BUF_SIZE: usize = 160;

#[derive(Resource)]
struct AppState {
    listener: TcpListener,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    App::new()
        // .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugins(DefaultPlugins)
        .add_plugins((DefaultPlugins, PanCamPlugin::default()))
        // .add_plugins((
            //     DefaultPlugins.set(WindowPlugin {
                //         primary_window: Some(Window {
                    //             title: "zappy_42".into(),
                    //             resolution: (1920., 1080.).into(),
                    //             // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    //             // prevent_default_event_handling: false,
                    //             // window_theme: Some(WindowTheme::Dark),
                    //             // enabled_buttons: bevy::window::EnabledButtons {
                        //                 // maximize: false,
                        //                 // ..Default::default()
                        //             // },
                        //             // This will spawn an invisible window
                        //             // The window will be made visible in the make_visible() system after 3 frames.
                        //             // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                        //             visible: true,
                        //             ..default()
                        //         }),
                        //         ..default()
                        //     }),
                        //     LogDiagnosticsPlugin::default(),
                        //     FrameTimeDiagnosticsPlugin,
        // ))
        .add_systems(Startup, setup_handle_connections)
        .add_plugins(TilesPlugin)
        .add_systems(Update, lolo_fn)
        // .add_systems(Startup, setup_sprite)
        // .add_systems(Update, animate_sprite)
        // .add_systems(Update, sprite_movement)

        // .add_systems(Startup, spawn_map)

        .insert_resource(AppState { listener })
        .run();
}

impl AppState {
    fn setup_system(mut commands: Commands) {
        // Additional setup if needed
    }
}

pub enum Parse{
    Map(i32, i32),
    RessourceCase(i32, i32, u8, u8, u8, u8, u8, u8, u8),
    ConnexionPlayer(u8, u8, u8, u8, u8, String),
    Donothing,
    // Movemement(i32, i32, i32)
}

pub fn lolo_fn(mut guizmo: Gizmos)
{
    guizmo.line_2d(Vec2::ZERO, Vec2::new(0.,150.), Color::RED);
}
   

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<Parse>);

#[derive(Event)]
struct StreamEvent(Parse);

fn copy_until_char(buffer: &[u8], char: u8) -> String
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
    println!("{:?}", iter);
    let vec : Vec<i32> =  iter.map(|x| x.parse::<i32>().ok().unwrap()).collect();
    vec
}


pub fn parse_ressource(content: String) -> crate::Parse
{
    let mut iter = content.split_ascii_whitespace();
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

pub fn parse_connexion_player(content: String) -> crate::Parse
{
    let mut vec_parsing_u8: Vec<u8> = vec![];
    let mut team: String = format!("");
    for i in content.split_ascii_whitespace().skip(1).enumerate()
    {
        if i.0 < 5
        {
            println!("{:?}", i.1);
            vec_parsing_u8.push(i.1.parse::<u8>().ok().unwrap());
        }
        else {
            team = i.1.to_string().clone();
        }
    }
    Parse::ConnexionPlayer(vec_parsing_u8[0], vec_parsing_u8[1], vec_parsing_u8[2], vec_parsing_u8[3], vec_parsing_u8[4], team)
}
// dispatch what you parse 
fn parser_server_packet(pkt_receive: String) -> Parse
{
    println!("{}", pkt_receive);
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
                    println!("bct");
                    parse = parse_ressource(pkt_receive);
                }
                "tna" => {
                    todo!();
                }
                "pnw" => {
                    parse = parse_connexion_player(pkt_receive);
                }
                "ppo" => {
                    todo!();
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
        println!("STRING MAP = {:?}", string);
        vec_map.push(string.parse::<i32>().ok().unwrap());
    }
    // let x = vec_map[0].parse::<u32>;
    Parse::Map {0:vec_map[0],1:vec_map[1]}
}

fn setup_handle_connections(state: Res<AppState>, mut command: Commands) {
    println!("toto1");
    for stream in state.listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // Spawn a new thread to handle each incoming connection
                println!("toto1.5");
                let (tx, rx) = bounded::<Parse>(1);
                thread::spawn(move || {
                    let mut buffer = [0; 32];
                    loop {
                        println!("{:?}", stream);
                        match stream.read(&mut buffer) {
                            Ok(n) if n == 0 => {
                            
                                // Connection closed by the client
                                break;
                            }
                            Ok(n) => {
                                // Process the received data
                                let received_data = &buffer[..n];
                                println!("Received data: {:?}", received_data);
                                let str = copy_until_char(received_data, b'\n');
                                println!("str {:?}", str);
                                let parse : Parse = parser_server_packet(str);
                                tx.send(parse).unwrap();
                                // Optionally, send a response back to the client
                            }
                            Err(e) => {
                                // Handle errors
                                eprintln!("Error reading from stream: {}", e);
                                break;
                            }
                        }
                    }
                });
                command.insert_resource(StreamReceiver(rx));
                break;
            }
            Err(e) => {
                // Handle errors
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}




// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());

//     let (tx, rx) = bounded::<u32>(10);
//     std::thread::spawn(move || {
//         let mut rng = StdRng::seed_from_u64(19878367467713);
//         loop {
//             // Everything here happens in another thread
//             // This is where you could connect to an external data source
//             let start_time = Instant::now();
//             let duration = Duration::from_secs_f32(rng.gen_range(0.0..0.2));
//             while start_time.elapsed() < duration {
//                 // Spinning for 'duration', simulating doing hard work!
//             }

//             tx.send(rng.gen_range(0..2000)).unwrap();
//         }
//     });

//     commands.insert_resource(StreamReceiver(rx));
// }

// // This system reads from the receiver and sends events to Bevy
// fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
//     for from_stream in receiver.try_iter() {
//         events.send(StreamEvent(from_stream));
//     }
// }

// fn spawn_text(mut commands: Commands, mut reader: EventReader<StreamEvent>) {
//     let text_style = TextStyle {
//         font_size: 20.0,
//         color: Color::WHITE,
//         ..default()
//     };

//     for (per_frame, event) in reader.read().enumerate() {
//         commands.spawn(Text2dBundle {
//             text: Text::from_section(event.0.to_string(), text_style.clone())
//                 .with_alignment(TextAlignment::Center),
//             transform: Transform::from_xyz(per_frame as f32 * 100.0, 300.0, 0.0),
//             ..default()
//         });
//     }
// }

// fn move_text(
//     mut commands: Commands,
//     mut texts: Query<(Entity, &mut Transform), With<Text>>,
//     time: Res<Time>,
// ) {
//     for (entity, mut position) in &mut texts {
//         position.translation -= Vec3::new(0.0, 100.0 * time.delta_seconds(), 0.0);
//         if position.translation.y < -300.0 {
//             commands.entity(entity).despawn();
//         }
//     }
// }