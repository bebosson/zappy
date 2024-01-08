pub mod map{
    use std::{io::Empty, vec, process::exit};

    use bevy::{prelude::*, math::vec3, asset};
    use bevy_pancam::PanCam;

    use crate::{MAP_WIDTH, TILES_WIDTH, TILES_HEIGHT, StreamEvent, StreamReceiver, MAP_HEIGHT, Ressource::Ressource::{spawn_resources, Ressource}, sprite_player::sprite_player::setup_sprite};

    #[derive(Component, Debug)]
    pub struct Position(Vec3);

    #[derive(Component)]
    pub struct Tile;
    
    #[derive(Component)]
    pub struct Map(Vec<Tile>);

    
    
    #[derive(Resource)]
    pub struct RessCommandId{
        x: u32,
        y: u32,
        pub Id_Ressource: Vec<Entity>,
    }

    impl RessCommandId{
        pub fn get_my_coor(&self) -> (u32, u32){
            (self.x, self.y)
        }
        pub fn center_map_new_system(&self, x_old: f32, y_old: f32) -> (f32, f32)
        {
            let vec_trans = Vec2::new(-(self.x as f32 ) * TILES_WIDTH, (self.y as f32) * TILES_WIDTH);
            let x_new = x_old * TILES_WIDTH + vec_trans.x; 
            let y_new = -y_old * TILES_WIDTH + vec_trans.y;
            println!("[x : {}===> {}", x_old, x_new);
            println!("y : {}===> {}]", y_old, y_new);
            (x_new, y_new)
            // let x_rel: x_abs as i32
            
        }
    }

    pub struct TilesPlugin;

    impl Plugin for TilesPlugin{
        fn build(&self, app: &mut App) {
            app
            .add_event::<StreamEvent>()
            .add_systems(Startup, init)
            .add_systems(Update, read_stream)
            .add_systems(Update, dispatch_event);
            // .add_systems(Update, print_resources);
        }
    }
    
    fn init(mut commands: Commands)
    {
        commands.insert_resource(RessCommandId{x: 2, y: 0, Id_Ressource: vec![]});
        // let toto = world.query(Query<)
    }

    fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
        for parse in receiver.try_iter() {
            events.send(StreamEvent(parse));
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

    
    
    pub fn dispatch_event(mut commands: Commands, asset_server: Res<AssetServer>, mut reader: EventReader<StreamEvent>, query : Query<(&Position, With<Tile>)>, mut asset_map: ResMut<RessCommandId>,  mut texture_atlases: ResMut<Assets<TextureAtlas>>)
    {
        let mut vec_map_entity: Vec<Entity> = vec![];
        for (_, event) in reader.read().enumerate() {
            let x = &event.0;
            match x
            {
                crate::Parse::Map(x, y) => {
                    spawn_map(*x as u32, *y as u32, & mut commands, &asset_server, & mut asset_map);
                }
                crate::Parse::RessourceCase(x, y, n, l, d, s,m , ph, th) => {
                    let (x_rel, y_rel) = asset_map.center_map_new_system(*x as f32, *y as f32);
                    let ressource = Ressource{ x: x_rel as i32, y: y_rel as i32, n: *n, l: *l, d: *d, s: *s, m: *m, ph: *ph, th: *th };
                    println!("Ress {:?}", ressource);
                    spawn_resources(& mut commands, &asset_server, ressource);
                }
                crate::Parse::ConnexionPlayer(id, x, y, O, L, N) => {
                    println!("LOOOOL {} {}", x, y);
                    let (x_rel, y_rel) = asset_map.center_map_new_system(*x as f32, *y as f32);
                    setup_sprite(& mut commands, &asset_server, & mut texture_atlases, x_rel, y_rel);
                }
                _ => ()
            }
        }
       
        
    }
   
       
    
     
}
