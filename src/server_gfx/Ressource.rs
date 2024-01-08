pub mod Ressource
{
    use bevy::{ecs::system::{Commands, Res}, asset::AssetServer, sprite::SpriteBundle, a11y::accesskit::Vec2, math::Vec3, transform::components::Transform};

    use crate::{map::map::{transform_for_ressource}, TILES_WIDTH};



    #[derive(Debug)]
    pub struct Ressource
    {
        pub x: i32,
        pub y: i32,
        pub n: u8, 
        pub l: u8, 
        pub d: u8, 
        pub s: u8,
        pub m: u8,
        pub ph: u8,
        pub th: u8,
    }

    const OFFSET: f32 = 0.6;

    pub fn spawn_food(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32)
    {
        
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x - OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y + OFFSET * TILES_WIDTH / 2., 13.);
            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("food.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            ));
        }
    }

    pub fn spawn_linemate(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32)
    {
        
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x + (nbr_res * 2) as f32, pos_y + OFFSET * TILES_WIDTH / 2., 13.);
            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("Linemate.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            ));
        }
    }

    pub fn spawn_deraumere(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32)
    {
        
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x + OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y + OFFSET * TILES_WIDTH / 2., 13.);
            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("deraumere.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            ));
        }
    }
    pub fn spawn_sibur(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32)
    {
        
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x - OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y, 13.);
            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("Sibur.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            ));
        }
    }

    pub fn spawn_mendiane(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32)
    {
        
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x + OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y, 13.);
            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("Mendiane.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            ));
        }
    }

    pub fn spawn_phiras(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32)
    {
        
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x - OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y - OFFSET * TILES_WIDTH / 2., 13.);
            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("Phiras.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            ));
        }
    }

    pub fn spawn_thystame(commands: & mut Commands, asset_server: &Res<AssetServer> , nbr_ressource: u8, pos_x: f32, pos_y: f32)
    {
        
        for nbr_res in 0..nbr_ressource{
            let pos_res = Vec3::new(pos_x + OFFSET * TILES_WIDTH / 2. + (nbr_res * 2) as f32, pos_y - OFFSET * TILES_WIDTH / 2., 13.);
            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("Thystame.png"),
                    transform: Transform::from_translation(pos_res),
                    ..Default::default()
                }
            ));
        }
    }
    

    pub fn spawn_resources(commands: & mut Commands, asset_server: &Res<AssetServer> , res: Ressource)
    {
        if res.n > 0
        {
            spawn_food(commands, asset_server, res.n, res.x as f32, res.y as f32);
        }
        if res.l > 0
        {
            spawn_linemate(commands, asset_server, res.l, res.x as f32, res.y as f32);
        }
        if res.d > 0
        {
            spawn_deraumere(commands, asset_server, res.d, res.x as f32, res.y as f32);
        }
        if res.s > 0
        {
            spawn_sibur(commands, asset_server, res.s, res.x as f32, res.y as f32);
        }
        if res.m > 0
        {
            spawn_mendiane(commands, asset_server, res.m, res.x as f32, res.y as f32);
        }
        if res.ph > 0
        {
            spawn_phiras(commands, asset_server, res.ph, res.x as f32, res.y as f32);
        }
        if res.th > 0
        {
            spawn_thystame(commands, asset_server, res.th, res.x as f32, res.y as f32);
        }
        
        // commands.spawn((
        //     SpriteBundle{
        //         texture: asset_server.load("Ressource.png"),
        //         transform: transform_for_ressource(res.x as f32, res.y as f32),
        //         ..Default::default()
        //     }
        // ));
        // commands.get_entity(entity)

    }
}