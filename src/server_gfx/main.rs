mod map;
pub mod sprite_player;
mod Ressource;
mod parser;
pub mod dispatch;

use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{WindowTheme, PresentMode};
use bevy_pancam::PanCamPlugin;
use crossbeam_channel::bounded;
use dispatch::dispatch::Dispatch;
use rand::SeedableRng;
use rand::rngs::StdRng;
use sprite_player::sprite_player::{setup_sprite, animate_sprite, DoAction};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::SplitAsciiWhitespace;
use std::thread;

use bevy::prelude::*;
// Using crossbeam_channel instead of std as std `Receiver` is `!Sync`
use crossbeam_channel::{Receiver};
use rand::{Rng};
use std::time::{Duration, Instant};

use crate::parser::parser::{copy_until_char, Parse, parser_server_packet};

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
        .insert_resource(AppState { listener })
        .add_systems(Startup, setup_handle_connections)
        .add_plugins(Dispatch)
        .add_plugins(DoAction)
        .add_systems(Update, lolo_fn)
        // .add_systems(Startup, setup_sprite)
        .add_systems(Update, animate_sprite)
        // .add_systems(Update, sprite_movement)

        // .add_systems(Startup, spawn_map)

        .run();
}

impl AppState {
    fn setup_system(mut commands: Commands) {
        // Additional setup if needed
    }
}



pub fn lolo_fn(mut guizmo: Gizmos)
{
    guizmo.line_2d(Vec2::ZERO, Vec2::new(0.,150.), Color::RED);
}
   

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<Parse>);

#[derive(Event)]
pub struct StreamEvent(Parse);



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