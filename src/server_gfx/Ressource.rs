pub mod Ressource
{
    use std::collections::HashMap;

    use bevy::{ecs::{system::{Commands, Query, Res}, entity::Entity}, asset::AssetServer, sprite::SpriteBundle, a11y::accesskit::Vec2, math::Vec3, transform::{self, components::Transform}};

    use crate::{map::map::{transform_for_ressource}, TILES_WIDTH};
    
    #[derive(Debug)]
    pub struct ContentCase{
        pub nbr_ressource: u8,
        pub all_entity: Vec<Entity>,   
    }

    #[derive(Debug)]
    pub struct Ressource
    {
        pub x_rel: f32,
        pub x: u32,
        pub y_rel: f32,
        pub y: u32,
        ///****** TRANSFORMER TOUT CA EN VECTOR */
        pub n: u8, 
        pub l: u8, 
        pub d: u8, 
        pub s: u8,
        pub m: u8,
        pub ph: u8,
        pub th: u8,
        /****                                    */
    }

    const OFFSET: f32 = 0.6;

    pub fn spawn_food(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32) -> ContentCase
    {
        let mut vec_entity: Vec<Entity> = vec![];
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x - OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y + OFFSET * TILES_WIDTH / 2., 13.);
            vec_entity.push(commands.spawn(
                SpriteBundle{
                    texture: asset_server.load("food.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            ).id());
        }
        ContentCase { nbr_ressource, all_entity: vec_entity }
    }

    pub fn spawn_linemate(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32) -> ContentCase
    {
        let mut vec_entity: Vec<Entity> = vec![];
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x + (nbr_res * 2) as f32, pos_y + OFFSET * TILES_WIDTH / 2., 13.);
            vec_entity.push(commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("Linemate.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            )).id());
        }
        ContentCase { nbr_ressource, all_entity: vec_entity }
    }

    pub fn spawn_deraumere(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32) -> ContentCase
    {
        let mut vec_entity: Vec<Entity> = vec![];
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x + OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y + OFFSET * TILES_WIDTH / 2., 13.);
            vec_entity.push(commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("deraumere.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            )).id());
        }
        ContentCase { nbr_ressource, all_entity: vec_entity }
    }
    pub fn spawn_sibur(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32) -> ContentCase
    {
        let mut vec_entity: Vec<Entity> = vec![];
        
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x - OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y, 13.);
            vec_entity.push(commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("Sibur.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            )).id());
            
        }
        ContentCase { nbr_ressource, all_entity: vec_entity }
    }

    pub fn spawn_mendiane(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32) -> ContentCase
    {
        let mut vec_entity: Vec<Entity> = vec![];
        
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x + OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y, 13.);
            vec_entity.push(commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("Mendiane.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            )).id());
        }
        ContentCase { nbr_ressource, all_entity: vec_entity }
    }

    pub fn spawn_phiras(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32) -> ContentCase
    {
        let mut vec_entity: Vec<Entity> = vec![];
        
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x - OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y - OFFSET * TILES_WIDTH / 2., 13.);
            vec_entity.push(commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("Phiras.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            )).id());
        }
        ContentCase { nbr_ressource, all_entity: vec_entity }
    }

    pub fn spawn_thystame(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32) -> ContentCase
    {
        let mut vec_entity: Vec<Entity> = vec![];
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x + OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y - OFFSET * TILES_WIDTH / 2., 13.);
            vec_entity.push(commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("Thystame.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            )).id());
        }
        ContentCase { nbr_ressource, all_entity: vec_entity }
    }

    pub fn compare_resources(commands: &mut Commands, res: Ressource, vec_hashress: &mut Vec<Vec<HashMap<u32, ContentCase>>>)
    {
        // bonne fonction mais il faut introduire les states d'abord
        let vec_ressource = vec![res.n, res.l, res.d, res.s, res.m, res.ph, res.th];
        vec_ressource.into_iter().enumerate().for_each(|(index, actual_res)| {
            if let Some(x) = vec_hashress[res.y as usize][res.x as usize].get_mut(&(index as u32)) {
                if x.nbr_ressource != actual_res
                {
                    if let Some(entity) = x.all_entity.last()
                    {
                        commands.get_entity(*entity);

                    }
                }
            }
        });
       
        
    }

    pub fn add_all_resources(commands: &mut Commands, asset_server: &Res<AssetServer>,  res: Ressource, vec_hashress: &mut Vec<Vec<HashMap<usize, ContentCase>>>)
    {
        if res.n > 0
        {
            vec_hashress[res.y as usize][res.x as usize].insert(0, spawn_food(commands, asset_server, res.n, res.x_rel, res.y_rel));
        }
        if res.l > 0
        {
            vec_hashress[res.y as usize][res.x as usize].insert(1,spawn_linemate(commands, asset_server, res.l, res.x_rel, res.y_rel));
        }
        if res.d > 0
        {
            vec_hashress[res.y as usize][res.x as usize].insert(2,spawn_deraumere(commands, asset_server, res.d, res.x_rel, res.y_rel));
        }
        if res.s > 0
        {
            vec_hashress[res.y as usize][res.x as usize].insert(3,spawn_sibur(commands, asset_server, res.s, res.x_rel, res.y_rel));
        }
        if res.m > 0
        {
            vec_hashress[res.y as usize][res.x as usize].insert(4,spawn_mendiane(commands, asset_server, res.m, res.x_rel, res.y_rel));
        }
        if res.ph > 0
        {
            vec_hashress[res.y as usize][res.x as usize].insert(5,spawn_phiras(commands, asset_server, res.ph, res.x_rel, res.y_rel));
        }
        if res.th > 0
        {
            vec_hashress[res.y as usize][res.x as usize].insert(6,spawn_thystame(commands, asset_server, res.th, res.x_rel, res.y_rel));
        }
    }


    
    pub fn spawn_resources(commands: & mut Commands, asset_server: &Res<AssetServer> , res: Ressource, vec_hashress: &mut Vec<Vec<HashMap<usize, ContentCase>>>)
    {
        //println!("{:?}", vec_hashress[res.y as usize][res.x as usize]);
        //verifier si hashmap[y][x].is_empty -> spawn ressources
        //sinon on modifie la ou les ressources concerne, donc spawn ou despawn
        if vec_hashress[res.y as usize][res.x as usize].is_empty()
        {
           add_all_resources(commands, asset_server, res, vec_hashress);
        }
        else
        {

        }
        
        
        // commands.spawn((
        //     SpriteBundle{
        //         texture: asset_server.load("Ressource.png"),
        //         transform: transform_for_ressource(res.x_rel as f32, res.y_rel as f32),
        //         ..Default::default()
        //     }
        // ));
        // commands.get_entity(entity)

    }

    pub fn anim_take_ressource_res(mut query_pos_res: & mut Query<& mut Transform>, id: &Entity, res: Ressource)
    {
        // let type_action_ref: TypeAction = type_action;
        if let Ok(mut transform) = query_pos_res.get_mut(*id)
        {
            let vec = Vec3::new(res.x_rel, res.y_rel, 13.);
            *transform = Transform::from_translation(vec);
            //change sprite
            //despawn resource 
        }
    }

    pub fn get_ressource_entity(command: &mut Commands, res: &Ressource, num_res: u8, vec_hashress: &mut Vec<Vec<HashMap<usize, ContentCase>>>) -> Entity
    {
        if let Some(content_case) = vec_hashress[res.y as usize][res.x as usize].get(&(num_res as usize)){
            if let Some(entity) = content_case.all_entity.last()
            {
                return *entity
            }
            else{
                panic!("at this point the entity must exist");
            }
        }
        else {
            panic!("at this point the content case must exist");
        }
    }
}