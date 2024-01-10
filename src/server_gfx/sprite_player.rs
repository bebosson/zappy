

//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

pub mod sprite_player{
    // mod map;

    use std::collections::VecDeque;

    use bevy::prelude::*;
    // use map::map::spawn_map;
    use crate::{TILES_WIDTH, dispatch::dispatch::{RessCommandId, VECSPRITE}};

    
    
    pub struct DoAction;

    impl Plugin for DoAction{
        fn build(&self, app: &mut App) {
            app
            .add_systems(Update, set_exec_action)
            .add_systems(Update, exec_action);
            // .add_systems(Update, print_resources);
        }
    }

    

    #[derive(Component, Debug)]
    pub enum TypeofMovement
    {
        Translation,
        Rotation,
        Nothing,
        // distance_total: f32,
        // distance_restante: f32,
        // orientation: u8,
    }
    #[derive(Component, Debug)]
    pub struct Movementinprogress
    {
        distance_restante: f32,
        orientation: u8,
        type_of_mvmt: TypeofMovement,
    }

    impl Movementinprogress
    {
        pub fn new() -> Self
        {
            Movementinprogress{
                distance_restante: 0.,
                orientation: 0,
                type_of_mvmt: TypeofMovement::Nothing,
            }
        }
        pub fn set_distance(& mut self, dist: f32, o: u8)
        {
            self.distance_restante = dist;
            self.orientation = o;
        }
    }

    
    #[derive(Component)]
    pub struct Cell(u8, u8, u8);

    #[derive(Debug)]
    pub enum TypeAction{
        Movement(u8,u8,u8),
        Nothing,
    }
    #[derive(Debug)]
    enum StateAction
    {
        InAction,
        Idle
    }

    

    #[derive(Component, Debug)]
    pub struct ActionPlayer
    {
        vecdeque: VecDeque<TypeAction>,
        state_action: StateAction,
        action_in_progress: TypeAction,
    }

    impl ActionPlayer
    {
        pub fn new() -> Self
        {
            ActionPlayer{
                vecdeque: VecDeque::new(),
                state_action: StateAction::Idle,
                action_in_progress: TypeAction::Nothing,
            }
        }
    }

    pub fn player_translation(time: &Res<Time>, action_player: & mut ActionPlayer, movement : &mut Movementinprogress, transform: & mut Transform)
    {
        let distance_delta = TILES_WIDTH / 2. * time.delta_seconds();
        // println!("{:?}", 1./time.delta_seconds());
        if movement.distance_restante < 0.
        {
            action_player.state_action = StateAction::Idle;
            action_player.action_in_progress = TypeAction::Nothing;
            return ;
        }
        // println!("{:?}", movement);

        match movement.orientation
        {
            1 => { transform.translation.y += distance_delta } // Nord 
            2 => { transform.translation.x += distance_delta } // Est
            3 => { transform.translation.y -= distance_delta } // Sud
            4 => { transform.translation.x -= distance_delta } // Ouest
            _ => { panic!() }
        }
        movement.distance_restante -= distance_delta;
    }

    

    pub fn add_action(mut query_action_player: & mut Query<& mut ActionPlayer>, id: &Entity, mut type_action: TypeAction)
    {
        // let type_action_ref: TypeAction = type_action;
        if let Ok(mut action_player) = query_action_player.get_mut(*id)
        {
            action_player.vecdeque.push_back(type_action);
            println!("{:?}", action_player);
        }
    }

    pub fn dist_abs(start: f32, finish: f32) -> f32
    {
        if start < finish
        {
            (start - finish) * -1.
        }
        else {
            start - finish
        }
    }


    pub fn set_exec_action(mut query_action_player: Query<(& mut ActionPlayer, &mut Movementinprogress, &Cell)>, asset_map: ResMut<RessCommandId>)
    {
        for (mut action_player, mut movement, cell) in query_action_player.iter_mut()
        {
            if let StateAction::Idle = action_player.state_action{
                if let Some(current_action) = action_player.vecdeque.pop_front()
                {
                    action_player.state_action = StateAction::InAction;
                    action_player.action_in_progress = current_action;
                    match action_player.action_in_progress{
                        TypeAction::Movement(x_finish, y_finish, o) => {
                            if (cell.0 != x_finish || cell.1 != y_finish) && cell.2 == o 
                            {
                                // let pixel_start = asset_map.center_map_new_system(cell.0 as f32, cell.1 as f32);
                                // let pixel_finish = asset_map.center_map_new_system(x_finish as f32, y_finish as f32);
                                *movement = Movementinprogress{ distance_restante: TILES_WIDTH, orientation: o, type_of_mvmt: TypeofMovement::Translation }
                            }
                            else
                            {
                                *movement = Movementinprogress{ distance_restante: TILES_WIDTH, orientation: o, type_of_mvmt: TypeofMovement::Rotation }

                                
                            }
                        }
                        TypeAction::Nothing => ()
                    }
                }
            }
        }
    }

    pub fn player_rotation(rotation: u8, asset_map: &ResMut<RessCommandId>,  handle_texture_atlas: &mut Handle<TextureAtlas>, texture_atlas_sprite: &mut TextureAtlasSprite) 
    {
        println!("rotation?");
        let sprite_animation = asset_map.get_sprite(rotation as usize);
        *handle_texture_atlas = sprite_animation.texture_atlas_handle;
        *texture_atlas_sprite = sprite_animation.texture_atlas_sprite;
        // set_texture_atlas_animation_indice(texture_handle, o);
        // set_sprite_animation(0, o, texture_atlases, asset_server)

    }
    

    pub fn exec_action(
        time: Res<Time>, 
        mut query_action_player: Query<(& mut ActionPlayer, &mut Movementinprogress, & mut Transform, &mut Handle<TextureAtlas>, & mut TextureAtlasSprite)>,
        asset_map: ResMut<RessCommandId>)
    {
        for (mut action_player,
            mut movement,
            mut transform,
            mut handle_texture_atlas,
            mut texture_atlas_sprite
        ) 
            in query_action_player.iter_mut()
        {
            let typeofmvmt = &movement.type_of_mvmt;
            if let StateAction::InAction = action_player.state_action{
                match action_player.action_in_progress{
                    TypeAction::Movement(_, _, _) => {
                        match typeofmvmt
                        {
                            TypeofMovement::Translation => {
                                // let ref_movement = *movement;
                                println!("translation ");
                                player_translation(&time, & mut action_player, & mut movement, & mut transform);
                            }
                            TypeofMovement::Rotation => {
                                player_rotation(movement.orientation ,&asset_map, & mut handle_texture_atlas, & mut texture_atlas_sprite);
                                action_player.state_action = StateAction::Idle;
                                action_player.action_in_progress = TypeAction::Nothing;
                            },
                            TypeofMovement::Nothing => panic!(),
                        }
                        
                    },
                    TypeAction::Nothing => todo!(),
                }
            }
        }
    }

    #[derive(Bundle, Clone)]
    pub struct SpriteAnimation{
        // pub sprite_sheet_bundle: SpriteSheetBundle,
        pub texture_atlas_sprite : TextureAtlasSprite,
        pub texture_atlas_handle: Handle<TextureAtlas>,
        pub animation_indices: AnimationIndices,
    }

    #[derive(Bundle, Clone)]
    pub struct SpriteComponent{
        pub sprite_sheet_bundle: SpriteSheetBundle,
        // pub texture_atlas_sprite : TextureAtlasSprite,
        // pub texture_atlas_handle: Handle<TextureAtlas>,
        pub animation_indices: AnimationIndices,
    }

    pub fn set_texture_atlas_animation_indice(texture_handle: Handle<Image>, o: u8) -> (TextureAtlas, AnimationIndices)
    {
        let text_at;
        let anim_i;
        match o
        {

            2 | 4 => { 
                text_at = TextureAtlas::from_grid(texture_handle, Vec2::new(27.1, 32.0), 10, 1, None, None);
                anim_i = AnimationIndices { first: 1, last: 9 };
                (text_at, anim_i)
            },
            _ => {
                text_at = TextureAtlas::from_grid(texture_handle, Vec2::new(21.9, 32.0), 5, 1, None, None);
                anim_i = AnimationIndices { first: 1, last: 4 };
                (text_at, anim_i)
            },
        }
    }

    pub fn set_sprite_animation(team: usize, o: u8, mut texture_atlases: & mut ResMut<Assets<TextureAtlas>>, asset_server: &Res<AssetServer>) -> SpriteAnimation
    {
        let texture_handle = asset_server.load(VECSPRITE[o as usize]);
        let texture_atlas_and_anim = set_texture_atlas_animation_indice(texture_handle, o);
            // TextureAtlas::from_grid(texture_handle, Vec2::new(27.1, 32.0), 10, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas_and_anim.0);
        // Use only the subset of sprites in the sheet that make up the run animation
        let texture_atlas_sprite = TextureAtlasSprite::new(texture_atlas_and_anim.1.first);
    // commands.spawn(Camera2dBundle::default());
        // let sprite_sheet_bundle = SpriteSheetBundle {
        //     texture_atlas: texture_atlas_handle,
        //     sprite: TextureAtlasSprite::new(animation_indices.first),
        //     transform: Transform::from_xyz(0.,0.,15.),
        //     ..default()
        // };
        SpriteAnimation { texture_atlas_sprite, texture_atlas_handle, animation_indices: texture_atlas_and_anim.1 }
    }

    #[derive(Component)]
    pub struct Player;
    
    
    #[derive(Component, Clone)]
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

    pub fn animation_to_sprite_component(animation: SpriteAnimation, x: &f32, y: &f32) -> SpriteComponent
    {
        let sprite_sheet_bundle = SpriteSheetBundle{
            texture_atlas: animation.texture_atlas_handle,
            sprite: animation.texture_atlas_sprite,
            transform: Transform::from_xyz(*x as f32,*y as f32,15.),
            ..default()
        };
        SpriteComponent{ sprite_sheet_bundle,  animation_indices: animation.animation_indices }
    }
    
    pub fn setup_sprite(
        mut commands: & mut Commands,
        asset_server: &Res<AssetServer>,
        mut texture_atlases: & mut ResMut<Assets<TextureAtlas>>, // ????
        coord_pixel: (f32, f32),
        coord_cell: (u8, u8, u8),
        asset_map: &mut ResMut<RessCommandId>,
        sprite_animation: SpriteAnimation,
    ) {
        let sprite_component = animation_to_sprite_component(sprite_animation, &coord_pixel.0, &coord_pixel.1);
        // commands.spawn(Camera2dBundle::default(),);
        // let texture_handle = asset_server.load("yoshi_walking3.png");
        // let texture_atlas =
        //     TextureAtlas::from_grid(texture_handle, Vec2::new(27.1, 32.0), 10, 1, None, None);
        // let texture_atlas_handle = texture_atlases.add(texture_atlas);
        // // Use only the subset of sprites in the sheet that make up the run animation
        // let animation_indices = AnimationIndices { first: 1, last: 9 };
        // commands.spawn(Camera2dBundle::default());
        asset_map.player_id.push(commands.spawn((
            // SpriteSheetBundle {
            //     texture_atlas: texture_atlas_handle,
            //     sprite: TextureAtlasSprite::new(animation_indices.first),
            //     transform: Transform::from_xyz(coord_pixel.0,coord_pixel.1,15.),
            //     ..default()
            // },
            // animation_indices,
            sprite_component,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ActionPlayer::new(),
            Player,
            Cell(coord_cell.0, coord_cell.1, coord_cell.2), // manque orientation 
            Movementinprogress::new(),
        )).id());
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
    
    
    // pub fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    //     for (mut logo, mut transform) in &mut sprite_position {
    //         match *logo {
    //             Direction::Right => transform.translation.x += 50. * time.delta_seconds(),
    //             Direction::Left => transform.translation.x -= 50. * time.delta_seconds(),
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
    
}

