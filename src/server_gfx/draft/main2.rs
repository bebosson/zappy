
//! Renders a 2D scene containing pixelated bevy logo in a pixel perfect style

mod map;
use map::map::{spawn_map, TilesPlugin, Tile};

use bevy::{prelude::*, ecs::query};

const MAP_WIDTH: f32 = 50. * 10.0;
const MAP_HEIGHT: f32 = 50.0;
const TILES_WIDTH: f32 = 50.0;
const TILES_HEIGHT: f32 = 50.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        .add_plugins(TilesPlugin)
        .add_systems(Startup, spawn_map)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, info_mouvement)
        .run();
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
}
#[derive(Component)]
pub struct Pos(Transform);

#[derive(Component)]
pub struct Player;

fn dist_abs(x1: f32, x2: f32) -> f32
{
    if x1 - x2 < 0.
    {
        -1. * (x1 - x2)
    }
    else {
        x1 - x2
    }
}

fn info_mouvement(sprite_position: Query<&Transform, With<Player>>, tile_position: Query<&Transform, With<Tile>>)
{
    if sprite_position.is_empty()
    {
        return ;
    }
    let player = sprite_position.single();
    let x = player.translation.x;
    let y = player.translation.y;
    // //println!("{} , {}", x, y);



    // let player = 
    for transform in tile_position.iter()
    {
        let tuple = (transform.translation.x, transform.translation.y);
        if dist_abs(x, tuple.0) < TILES_WIDTH / 2.0
        {
            //println!("transform {:?} x =  {} , y = {}", transform, x, y);
        }
    }
    // //println!("{} {}", x, y);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let vec = Vec3::new(100., 0., 11.);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player,
        Direction::Right,
        Pos(Transform::from_translation(vec)),
        SpriteBundle {
            texture: asset_server.load("my_perso.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
    ));
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Right => transform.translation.x += 30. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 30. * time.delta_seconds(), // Pos_1 <=====> pos2
        }

        if transform.translation.x > 200. {
            *logo = Direction::Left;
        } else if transform.translation.x < -200. {
            *logo = Direction::Right;
        }
    }
}



