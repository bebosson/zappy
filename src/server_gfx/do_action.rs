pub mod do_action{
    use std::{collections::VecDeque, fmt::Debug};
    use crate::sprite_player::sprite_player::{Cell, AnimationIndices, Player, SpriteComponent, SpriteAnimation};
    use bevy::{ecs::{component::Component, system::{Res, Query, ResMut}, entity::Entity}, time::Time, asset::Handle, sprite::{TextureAtlas, TextureAtlasSprite}, transform::components::Transform};

    use crate::{TILES_WIDTH, dispatch::dispatch::RessCommandId};


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
        pub distance_restante: f32,
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

        #[derive(Clone)]
        pub enum TypeAction{
            Movement(u8,u8,u8),
            Expulsion(u8, u8, u8, SpriteAnimation, SpriteAnimation),
            Nothing,
        }
        impl Debug for TypeAction
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Movement(arg0, arg1, arg2) => f.debug_tuple("Movement").field(arg0).field(arg1).field(arg2).finish(),
            Self::Expulsion(arg0, arg1, arg2, arg3, arg4) => f.debug_tuple("Expulsion").field(arg0).field(arg1).field(arg2).finish(),
            Self::Nothing => write!(f, "Nothing"),
        }
    }
        }
        #[derive(Debug, PartialEq)]
        pub enum StateAction
        {
            InAction,
            Idle
        }

        #[derive(Component, Debug)]
        pub struct ActionPlayer
        {
            pub vecdeque: VecDeque<TypeAction>,
            pub state_action: StateAction,
            pub action_in_progress: TypeAction,
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
    


    pub fn out_of_bound(transform: & mut Transform, asset_map:  &ResMut<RessCommandId>) {
        if transform.translation.y > asset_map.pixel_y_max {transform.translation.y = asset_map.pixel_y_min}
        if transform.translation.x > asset_map.pixel_x_max {transform.translation.x = asset_map.pixel_x_min}
        if transform.translation.x < asset_map.pixel_x_min {transform.translation.x = asset_map.pixel_x_max}
        if transform.translation.y < asset_map.pixel_y_min {transform.translation.y = asset_map.pixel_y_max}
    }

    pub fn update_cell(movement : &Movementinprogress, cell: &mut Cell, asset_map:  &ResMut<RessCommandId>)
    {
        match movement.orientation
        {
            1 => {
                if cell.1 == 0
                {
                    cell.1 = (asset_map.y - 1) as u8
                }
                else {cell.1 -= 1;} 
            } // Nord 
            2 => {
                cell.0 += 1;
                if cell.0 == asset_map.x as u8
                {
                    cell.0 = 0;
                } 
            } // Est
            3 => {
                cell.1 += 1;
                if cell.1 == asset_map.y as u8
                {
                    cell.1 = 0;
                }
            } // Sud
            4 => { 
                if cell.0 == 0
                {
                    cell.0 = (asset_map.x - 1) as u8
                }
                else {cell.0 -= 1;}
            }// Ouest
            _ => { panic!()}
        }
    }
        



    pub fn player_translation(time: &Res<Time>,
        action_player: & mut ActionPlayer,
        movement : &mut Movementinprogress,
        transform: & mut Transform,
        cell: &mut Cell,
        asset_map:  &ResMut<RessCommandId>,
        handle_texture_atlas: &mut Handle<TextureAtlas>,
        texture_atlas_sprite: &mut TextureAtlasSprite,
        animation_indice: &mut AnimationIndices,
        opt_change_sprite: Option<SpriteAnimation>)
    {
        
        let t_prime: f32 = asset_map.time  as f32 / 7.0;
        // println!("asset_map.time = {:?}", asset_map.time);

        let mut distance_delta = TILES_WIDTH * time.delta_seconds() * t_prime as f32; // time
        // println!("{:?}", time.delta_seconds());
        // println!("distance_delta = {:?}", distance_delta);
        if distance_delta > movement.distance_restante {distance_delta = movement.distance_restante;}
        
        match movement.orientation
        {
            1 => { transform.translation.y += distance_delta } // Nord 
            2 => { transform.translation.x += distance_delta } // Est
            3 => { transform.translation.y -= distance_delta } // Sud
            4 => { transform.translation.x -= distance_delta } // Ouest
            _ => { panic!() }
        }
        // println!("transform.x = {} transform.y = {}", transform.translation.x, transform.translation.y);
        // println!("asset_map.x_max {} asset_map.y_max {}", asset_map.pixel_x_max, asset_map.pixel_y_max);
        // println!("asset_map.x_min {} asset_map.y_min {}", asset_map.pixel_x_min, asset_map.pixel_y_min);
        out_of_bound(transform, asset_map);
        movement.distance_restante -= distance_delta;
        if movement.distance_restante == 0.
        {
            println!("{:?}", movement.distance_restante);
            if let Some(sprite_animation) = opt_change_sprite
            {
                change_sprite(handle_texture_atlas, texture_atlas_sprite, animation_indice, sprite_animation);
            }
            action_player.state_action = StateAction::Idle;
            action_player.action_in_progress = TypeAction::Nothing;
            update_cell(movement, cell, asset_map);
            // let pos = asset_map.center_map_new_system(cell.0 as f32, cell.1 as f32);
            // transform.translation.x = pos.0;
            // transform.translation.y = pos.1;            
            return ;
        }
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

    pub fn change_sprite(
        handle_texture_atlas: &mut Handle<TextureAtlas>,
        texture_atlas_sprite: &mut TextureAtlasSprite,
        animation_indice: &mut AnimationIndices,
        sprite_animation: SpriteAnimation,
    ) 
    {
        

        *handle_texture_atlas = sprite_animation.texture_atlas_handle;
        *texture_atlas_sprite = sprite_animation.texture_atlas_sprite;
        *animation_indice = sprite_animation.animation_indices;

        // set_texture_atlas_animation_indice(texture_handle, o);
        // set_sprite_animation(0, o, texture_atlases, asset_server)

    }

    pub fn set_exec_action(
        mut query_action_player: Query<(&mut ActionPlayer,
                                        &mut Movementinprogress,
                                        &mut Cell,
                                        &mut Handle<TextureAtlas>, 
                                        &mut TextureAtlasSprite, 
                                        &mut AnimationIndices,
                                        &Player)>,
        asset_map: ResMut<RessCommandId>)
    {
        for (mut action_player,
            mut movement,
            mut cell,
            mut handle,
            mut texture, 
            mut animation, 
            player) in query_action_player.iter_mut()
        {
            if let StateAction::Idle = action_player.state_action{
                if let Some(current_action) = action_player.vecdeque.pop_front()
                {
                    action_player.state_action = StateAction::InAction;
                    action_player.action_in_progress = current_action;
                    match &action_player.action_in_progress{
                        TypeAction::Movement(x_finish, y_finish, o) => {
                            println!("{} {} {} {:?}", x_finish, y_finish, o, cell);
                            if (cell.0 != *x_finish || cell.1 != *y_finish) && cell.2 == *o 
                            {
                                // let pixel_start = asset_map.center_map_new_system(cell.0 as f32, cell.1 as f32);
                                // let pixel_finish = asset_map.center_map_new_system(x_finish as f32, y_finish as f32);
                                println!("translation ");

                                *movement = Movementinprogress{ distance_restante: TILES_WIDTH, orientation: *o, type_of_mvmt: TypeofMovement::Translation}
                            }
                            else if cell.2 != *o
                            {
                                println!("rotation");
                                *movement = Movementinprogress{ distance_restante: TILES_WIDTH, orientation: *o, type_of_mvmt: TypeofMovement::Rotation}
                            }
                            else {
                                println!("Pb de Movement_Rotation ici (set exec action)\n")
                            }
                        }
                        TypeAction::Expulsion(x_finish, y_finish, o, _, sprite_expulsion) =>
                        {
                            if (cell.0 != *x_finish || cell.1 != *y_finish) && cell.2 == *o 
                            {
                                // let pixel_start = asset_map.center_map_new_system(cell.0 as f32, cell.1 as f32);
                                // let pixel_finish = asset_map.center_map_new_system(x_finish as f32, y_finish as f32);
                                println!("expulsion ");
                                change_sprite(& mut handle, & mut texture, & mut animation, sprite_expulsion.clone());
                                *movement = Movementinprogress{ distance_restante: TILES_WIDTH, orientation: *o, type_of_mvmt: TypeofMovement::Translation}
                            }
                            else {
                                println!("Pb d Expulsion ici (set exec action)\n")
                            }
                            // let sprite_animation = asset_map.get_sprite_expulsion(indice, num_team)
                        }
                        TypeAction::Nothing => ()
                    }
                }
            }
        }
    }

    pub fn player_rotation(
        rotation: u8,
        asset_map: &ResMut<RessCommandId>,
        handle_texture_atlas: &mut Handle<TextureAtlas>,
        texture_atlas_sprite: &mut TextureAtlasSprite,
        animation_indice: &mut AnimationIndices,
        team_num: usize,
    ) 
    {
        
        let sprite_animation = asset_map.get_sprite((rotation - 1) as usize, team_num);

        *handle_texture_atlas = sprite_animation.texture_atlas_handle;
        *texture_atlas_sprite = sprite_animation.texture_atlas_sprite;
        *animation_indice = sprite_animation.animation_indices;

        // set_texture_atlas_animation_indice(texture_handle, o);
        // set_sprite_animation(0, o, texture_atlases, asset_server)

    }

    
    

    pub fn exec_action(
        time: Res<Time>, 
        mut query_action_player: Query<(&mut ActionPlayer,
                                        &mut Movementinprogress,
                                        &mut Transform,
                                        &mut Handle<TextureAtlas>, 
                                        &mut TextureAtlasSprite, 
                                        &mut AnimationIndices,
                                        &mut Cell,
                                        &Player)>,
        asset_map: ResMut<RessCommandId>)
    {
        for (mut action_player,
            mut movement,
            mut transform,
            mut handle_texture_atlas,
            mut texture_atlas_sprite,
            mut animation_indices,
            mut cell,
            player,
        ) 
            in query_action_player.iter_mut()
        {
            let typeofmvmt = &movement.type_of_mvmt;
            if let StateAction::InAction = action_player.state_action{
                let actioninprogress = action_player.action_in_progress.clone();
                match actioninprogress{
                    TypeAction::Movement(_, _, _) => {
                        match typeofmvmt
                        {
                            TypeofMovement::Translation => {
                                // let ref_movement = *movement;
                                player_translation(&time, & mut action_player, & mut movement, & mut transform, &mut cell, &asset_map,& mut handle_texture_atlas, & mut texture_atlas_sprite, & mut animation_indices, None);
                                // new_coor = asset_map.get_my_coor(trans.x, translat)
                            }
                            TypeofMovement::Rotation => {
                                println!("team_num {:?}", player.0 as usize);
                                player_rotation(movement.orientation ,&asset_map, & mut handle_texture_atlas, & mut texture_atlas_sprite, & mut animation_indices, player.0 as usize);
                                cell.2 = movement.orientation;
                                action_player.state_action = StateAction::Idle;
                                action_player.action_in_progress = TypeAction::Nothing;
                            },
                            TypeofMovement::Nothing => {
                                action_player.state_action = StateAction::Idle;
                                action_player.action_in_progress = TypeAction::Nothing;
                            },
                        }
                        
                    },
                    TypeAction::Expulsion(_, _, _, sprite_anim_mvmt, _) => {
                        if let TypeofMovement::Translation = typeofmvmt
                        {
                            player_translation(&time, & mut action_player, & mut movement, & mut transform, &mut cell, &asset_map, & mut handle_texture_atlas, & mut texture_atlas_sprite, & mut animation_indices,Some(sprite_anim_mvmt.clone()));
                        }
                        else {
                            action_player.state_action = StateAction::Idle;
                            action_player.action_in_progress = TypeAction::Nothing;
                        }
                    }
                    TypeAction::Nothing => todo!(),
                }
            }
        }
    }

    pub fn get_nbr_player_cell(mut query_player_cell: & mut Query<(Entity, &Cell)>, player_entity: Entity) -> u8
    {
        let mut nbr_player: u8 = 0;
        let mut cell_ref = Cell(0,0,0);
        if let Ok((entity, cell)) = query_player_cell.get_mut(player_entity) {
            cell_ref = Cell(cell.0, cell.1, cell.2);
        }
        for (entity, cell) in query_player_cell.iter(){
            if (cell.0 == cell_ref.0) && (cell.1 == cell_ref.1) && entity != player_entity
            {
                nbr_player += 1;
            }
        }
        nbr_player
    }

}