pub mod map{


    use bevy::{prelude::*, math::vec3, asset};
    use bevy_pancam::PanCam;

    use crate::env::env::RessCommandId;


    #[derive(Component, Debug)]
    pub struct Position(Vec3);

    #[derive(Component)]
    pub struct Tile;
    
    #[derive(Component)]
    pub struct Map(Vec<Tile>);

    
    
    

    pub fn spawn_map(x: u32 , y: u32, commands: & mut Commands, asset_server: &Res<AssetServer>,  asset_map: & mut ResMut<RessCommandId>)
    {
        // let mut x: i32;
        // let mut y: i32;
        let texture_handle: Handle<Image> = asset_server.load("lawn.png");
        let start_x = 0;
        let mut end_x = x;
        let start_y = 0;
        let mut end_y = y;
        let orig_x = 0;
        let orig_y = 0;
        
        asset_map.x = x;
        asset_map.y = y;
        if x > 0
        {
            end_x = x - 1;
        }
        if y > 0
        {
            end_y = y - 1;
        }
        commands.spawn((Camera2dBundle::default(), PanCam::default()));
        for y_iter in start_y..=end_y
        {
            for x_iter in start_x..=end_x
            {
                let (x_rel, y_rel) = asset_map.center_map_new_system(x_iter as f32, y_iter as f32);
                let vec = Vec3 { x: x_rel, y: y_rel, z: 12. };
                commands.spawn((
                        Tile,
                        Position(vec),
                        SpriteBundle {
                            texture: texture_handle.clone(),
                            transform: transform_for_tiles(x_rel, y_rel),
                            ..default()
                        }
                    )
                );
            }
        }
    }
    pub fn transform_for_ressource(x: f32, y: f32) -> Transform
    {
        let my_transform = Transform::from_translation(vec3(x, y, 13.));
        my_transform.with_scale(vec3(0.5, 0.5, 1.0))
    }

    pub fn transform_for_tiles(x: f32, y: f32) -> Transform
    {
        let my_transform = Transform::from_translation(vec3(x, y, 12.));
        my_transform.with_scale(vec3(2., 2., 1.0))
    }

    
    
    
    
     
}
