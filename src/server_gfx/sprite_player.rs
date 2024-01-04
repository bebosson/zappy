

//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

pub mod sprite_player{
    // mod map;

    use bevy::prelude::*;
    // use map::map::spawn_map;
    
    const MAP_WIDTH: f32 = 50. * 10.0;
    const MAP_HEIGHT: f32 = 50.0;
    const TILES_WIDTH: f32 = 50.0;
    const TILES_HEIGHT: f32 = 50.0;
    
    
    
    // fn main() {
    //     App::new()
    //         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
            // .add_systems(Startup, setup)
    //         .add_systems(Startup, spawn_map)
    //         .add_systems(Update, animate_sprite)
    //         .add_systems(Update, sprite_movement)
    //         .run();
    // }
    
    
    #[derive(Component)]
    pub enum Direction {
        Left,
        Right,
    }
    
    #[derive(Component)]
    pub struct AnimationIndices {
        first: usize,
        last: usize,
    }
    
    #[derive(Component, Deref, DerefMut)]
    pub struct AnimationTimer(Timer);
    
    pub fn animate_sprite(
        time: Res<Time>,
        mut query: Query<(
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        )>,
    ) {
        for (indices, mut timer, mut sprite) in &mut query {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            }
        }
    }
    
    pub fn setup_sprite(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        commands.spawn(Camera2dBundle::default());
        let texture_handle = asset_server.load("yoshi_walking3.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(27.1, 32.0), 10, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices = AnimationIndices { first: 1, last: 9 };
        // commands.spawn(Camera2dBundle::default());
        commands.spawn((
            Direction::Left,
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(40.,0.,15.),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }

    
    // pub fn sprite_orient(mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    //     for (mut logo, mut transform) in &mut sprite_position {
    //         match *logo {
    //             Direction::Right => transform.translation.x += 30. * time.delta_seconds(),
    //             Direction::Left => transform.translation.x -= 30. * time.delta_seconds(),
    //         }
    
    //         if transform.translation.x > 200. {
    //             *logo = Direction::Left;
    //             // transform.rotate_z(30.);
    //             transform.scale.x *= -1.;
    //         } else if transform.translation.x < -200. {
    //             *logo = Direction::Right;
    //             transform.scale.x *= -1.;
    
    //         }
    //     }
    // }
    
    
    pub fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
        for (mut logo, mut transform) in &mut sprite_position {
            match *logo {
                Direction::Right => transform.translation.x += 30. * time.delta_seconds(),
                Direction::Left => transform.translation.x -= 30. * time.delta_seconds(),
            }
    
            if transform.translation.x > 200. {
                *logo = Direction::Left;
                // transform.rotate_z(30.);
                transform.scale.x *= -1.;
            } else if transform.translation.x < -200. {
                *logo = Direction::Right;
                transform.scale.x *= -1.;
    
            }
        }
    }
    
}

