

//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

pub mod sprite_player{
    // mod map;

    use std::collections::VecDeque;

    use bevy::prelude::*;
    // use map::map::spawn_map;
    use crate::{dispatch::dispatch::{RessCommandId, VECSPRITE, VECTEAM, SIZE_VECSPRITE}, do_action::do_action::{set_exec_action, exec_action, ActionPlayer, Movementinprogress, StateAction}};
    
    
    
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
    pub struct Cell(pub u8, pub u8, pub u8);


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
        // println!("o {}", o);
        match o
        {

            2 | 4 => { 
                text_at = TextureAtlas::from_grid(texture_handle, Vec2::new(27., 32.0), 9, 1, None, None);
                anim_i = AnimationIndices { first: 1, last: 8 };
                (text_at, anim_i)
            },
            3 => {
                text_at = TextureAtlas::from_grid(texture_handle, Vec2::new(27., 32.0), 9, 1, None, None);
                anim_i = AnimationIndices { first: 1, last: 8 };
                (text_at, anim_i)
            },
            _ => {
                text_at = TextureAtlas::from_grid(texture_handle, Vec2::new(27., 32.0), 9, 1, None, None);
                anim_i = AnimationIndices { first: 1, last: 8 };
                (text_at, anim_i)
            },
        }
    }

    pub fn set_sprite_animation(team: usize, o: u8, mut texture_atlases: & mut ResMut<Assets<TextureAtlas>>, asset_server: &Res<AssetServer>, name_sprite: [&'static str; SIZE_VECSPRITE]) -> SpriteAnimation
    {
        let path = format!("{}/{}", VECTEAM[team], name_sprite[o as usize]);
        println!("{}", path);
        let texture_handle = asset_server.load(path);
        let texture_atlas_and_anim = set_texture_atlas_animation_indice(texture_handle, o + 1); // on doit regler cette confusion entre orientation et indice 
            // TextureAtlas::from_grid(texture_handle, Vec2::new(27.1, 32.0), 10, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas_and_anim.0);
        // Use only the subset of sprites in the sheet that make up the run animation
        let texture_atlas_sprite = TextureAtlasSprite::new(texture_atlas_and_anim.1.first);
   
        SpriteAnimation { texture_atlas_sprite, texture_atlas_handle, animation_indices: texture_atlas_and_anim.1 }
    }

    #[derive(Component)]
    pub struct Player(pub u8);
    
    
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
            &ActionPlayer,
        )>,
    ) {
        for (indices, mut timer, mut sprite, action) in &mut query {
            if action.state_action == StateAction::InAction{
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
    }
    #[derive(Component)]
    pub enum StateSprite{
        NoAnimation(SpriteBundle),
        Animation(SpriteComponent)
    }

    pub fn animation_to_sprite_component(animation: SpriteAnimation, x: &f32, y: &f32) -> SpriteComponent
    {
        //transform.scale *= -1 if o == 2
        let sprite_sheet_bundle = SpriteSheetBundle{
            texture_atlas: animation.texture_atlas_handle,
            sprite: animation.texture_atlas_sprite,
            transform: Transform::from_xyz(*x as f32,*y as f32,15.),
            visibility: Visibility::Visible,
            ..default()
        };
        SpriteComponent{ sprite_sheet_bundle,  animation_indices: animation.animation_indices }
    }
    
    pub fn setup_sprite(
        mut commands: & mut Commands,
        asset_server: &Res<AssetServer>,
        coord_pixel: (f32, f32),
        coord_cell: (u8, u8, u8),
        asset_map: &mut ResMut<RessCommandId>,
        sprite_animation: SpriteAnimation,
        team_num: u8,
    ) -> Entity {
        let sprite_component = animation_to_sprite_component(sprite_animation, &coord_pixel.0, &coord_pixel.1);
        
        // on devrait rajouter un tmp sprite_component et un sprite_bundle 
        commands.spawn((
            sprite_component,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ActionPlayer::new(),
            Player(team_num),
            Cell(coord_cell.0, coord_cell.1, coord_cell.2), // manque orientation 
            Movementinprogress::new(),
        )).id()
    }

    
  
    
}

