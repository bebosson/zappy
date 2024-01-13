pub mod do_action{
    use std::collections::VecDeque;
    use crate::sprite_player::sprite_player::{Cell, AnimationIndices, Player};
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
        distance_restante: f32,
        orientation: u8,
        type_of_mvmt: TypeofMovement,
        cell_x_origin: u8,
        cell_y_origin: u8,
    }

    impl Movementinprogress
    {
        pub fn new() -> Self
        {
            Movementinprogress{
                distance_restante: 0.,
                orientation: 0,
                type_of_mvmt: TypeofMovement::Nothing,
                cell_x_origin: 0,
                cell_y_origin: 0,
            }
        }
        pub fn set_distance(& mut self, dist: f32, o: u8)
        {
            self.distance_restante = dist;
            self.orientation = o;
        }
    }

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
        



    pub fn player_translation(time: &Res<Time>, action_player: & mut ActionPlayer, movement : &mut Movementinprogress, transform: & mut Transform, cell: &mut Cell, asset_map:  &ResMut<RessCommandId>)
    {
        let distance_delta = TILES_WIDTH / 2. * time.delta_seconds();
        // println!("{:?}", 1./time.delta_seconds());
        // println!("{:?}", movement);
        
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
        if movement.distance_restante < 0.
        {
            action_player.state_action = StateAction::Idle;
            action_player.action_in_progress = TypeAction::Nothing;
            update_cell(movement, cell, asset_map);
            return ;
        }
    }



    pub fn add_action(mut query_action_player: & mut Query<& mut ActionPlayer>, id: &Entity, mut type_action: TypeAction)
    {
        // let type_action_ref: TypeAction = type_action;
        if let Ok(mut action_player) = query_action_player.get_mut(*id)
        {
            action_player.vecdeque.push_back(type_action);
            // println!("{:?}", action_player);
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
                            println!("{} {} {} {:?}", x_finish, y_finish, o, cell);
                            if (cell.0 != x_finish || cell.1 != y_finish) && cell.2 == o 
                            {
                                // let pixel_start = asset_map.center_map_new_system(cell.0 as f32, cell.1 as f32);
                                // let pixel_finish = asset_map.center_map_new_system(x_finish as f32, y_finish as f32);
                                println!("translation ");

                                *movement = Movementinprogress{ distance_restante: TILES_WIDTH, orientation: o, type_of_mvmt: TypeofMovement::Translation, cell_x_origin: cell.0, cell_y_origin: cell.1 }
                            }
                            else
                            {
                                println!("rotation");
                                *movement = Movementinprogress{ distance_restante: TILES_WIDTH, orientation: o, type_of_mvmt: TypeofMovement::Rotation, cell_x_origin: cell.0, cell_y_origin: cell.1}

                                
                            }
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
        mut query_action_player: Query<(& mut ActionPlayer, &mut Movementinprogress, & mut Transform, &mut Handle<TextureAtlas>, & mut TextureAtlasSprite, & mut AnimationIndices, &mut Cell, &Player)>,
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
                match action_player.action_in_progress{
                    TypeAction::Movement(_, _, _) => {
                        match typeofmvmt
                        {
                            TypeofMovement::Translation => {
                                // let ref_movement = *movement;
                                player_translation(&time, & mut action_player, & mut movement, & mut transform, &mut cell, &asset_map);
                                // new_coor = asset_map.get_my_coor(trans.x, translat)
                            }
                            TypeofMovement::Rotation => {
                                println!("team_num {:?}", player.0 as usize);
                                player_rotation(movement.orientation ,&asset_map, & mut handle_texture_atlas, & mut texture_atlas_sprite, & mut animation_indices, player.0 as usize);
                                cell.2 = movement.orientation;
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

}