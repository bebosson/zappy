pub mod map{


    use bevy::{asset, math::vec3, prelude::*, ui::RelativeCursorPosition};
    use bevy_pancam::PanCam;

    use crate::env::env::RessCommandId;


    #[derive(Component, Debug, Clone)]
    pub struct Position(Vec3);

    #[derive(Component, Clone)]
    pub struct Tile; //vouer a disparaitre (a remplacer par Cell)
    
    #[derive(Component)]
    pub struct Map(Vec<Tile>);

    #[derive(Bundle, Clone)]
    pub struct TileBundle{
        // pub sprite_sheet_bundle: SpriteSheetBundle,
        pub tile: Tile,
        pub position: Position,
        pub sprite_bundle: SpriteBundle,
        pub interact: Interaction,
    }

    impl TileBundle
    {
        pub fn new(texture_handle: &Handle<Image>, vec: &Vec3) -> Self
        {
            TileBundle{
                tile: Tile,
                position: Position(*vec),
                sprite_bundle: sprite_bundle(texture_handle.clone(), vec),
                interact: Interaction::None,
            }
        }
    }
    
    

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
                let entity_tile = commands.spawn((
                    TileBundle::new(&texture_handle, &vec),
                    RelativeCursorPosition::default()
                )
                ).id();
                asset_map.vec_tile_id.push(entity_tile);
                // if let Some(mut my_entity) = commands.get_entity(entity_tile)
                // {
                    // my_entity.insert(RelativeCursorPosition::default());
                // }
            }
        }
    }
    pub fn transform_for_ressource(x: f32, y: f32) -> Transform
    {
        let my_transform = Transform::from_translation(vec3(x, y, 13.));
        my_transform.with_scale(vec3(0.5, 0.5, 1.0))
    }

    pub fn transform_for_tiles(vec: &Vec3) -> Transform
    {
        let my_transform = Transform::from_translation(vec3(vec.x, vec.y, 12.));
        my_transform.with_scale(vec3(2., 2., 1.0))
    }

    pub fn sprite_bundle(texture_handle: Handle<Image>, vec: &Vec3) -> SpriteBundle
    {
        SpriteBundle{
            texture: texture_handle.clone(),
            transform: transform_for_tiles(vec),
            ..default()
        }
    }

    
    
    
    
     
}
