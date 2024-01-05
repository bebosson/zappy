pub mod map{
    use std::{io::Empty, vec};

    use bevy::{prelude::*, math::vec3};

    use crate::{MAP_WIDTH, TILES_WIDTH, TILES_HEIGHT, StreamEvent, StreamReceiver};

    #[derive(Component, Debug)]
    pub struct Position(Vec3);

    #[derive(Component)]
    pub struct Tile;
    
    #[derive(Component)]
    pub struct Map(Vec<Tile>);

    #[derive(Resource)]
    pub struct CollectId {
        pub vec_id: Vec<Vec<Entity>>,
    }

    pub struct Ressource
    {
        x: i32,
        y: i32,
        n: u8, 
        l: u8, 
        d: u8, 
        s: u8,
        m: u8,
        ph: u8,
        th: u8,
    }
    // struct MapId{
    //     Tile_Id: Entity,
    // }

    // #[derive(Bundle)]
    // struct TilesBundle {
    //     a: Tiles,
    //     b: SpriteBundle,
    // }

    // pub struct Map

    pub struct TilesPlugin;

    impl Plugin for TilesPlugin{
        fn build(&self, app: &mut App) {
            app
            .add_event::<StreamEvent>()
            .add_systems(Update, (read_stream, dispatch_event));
        }
    }

    fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
        for parse in receiver.try_iter() {
            events.send(StreamEvent(parse));
        }
    }

    pub fn spawn_map(x: i32 , y: i32, commands: & mut Commands, asset_server: &Res<AssetServer>)
    {
        // let mut x: i32;
        // let mut y: i32;
            let texture_handle: Handle<Image> = asset_server.load("lawn.png");
            let start_x = - (x * TILES_WIDTH as i32);
            let end_x = x * TILES_WIDTH as i32;
            let start_y = - (y * TILES_WIDTH as i32);
            let end_y = y * TILES_WIDTH as i32;
            let orig_x = 0;
            let orig_y = 0;
            let mut vec_entity_2d: Vec<Vec<Entity>> = vec![];
            for i in (start_y..=end_y).step_by(TILES_WIDTH as usize)
            {
                let mut vec_entity: Vec<Entity> = vec![];
                for j in (start_x..=end_x).step_by(TILES_HEIGHT as usize)
                {
                    let i_f32 = i as f32;
                    let j_f32 = j as f32;
                    let vec = Vec3::new(orig_x as f32 + i_f32, orig_y as f32 + j_f32, 10.);
                    vec_entity.push(commands.spawn((
                            Tile,
                            Position(vec),
                            SpriteBundle {
                                texture: texture_handle.clone(),
                                transform: Transform::from_translation(vec),
                                ..default()
                            }
                        )
                    ).id());
                }
                vec_entity_2d.push(vec_entity);
            }
            // commands.insert_resource(CollectId{
            //     vec_id: vec_entity_2d,
            // })
            // let child = commands.spawn(
            //         SpriteBundle {
            //             texture: asset_server.load("Ressource.png"),
            //             transform: Transform::from_xyz(100., 0., 0.),
            //             ..default()
            //         }
                
            // ).id();
            // println!("{:?}", child);
        }
    pub fn transform_for_ressource(x: f32, y: f32) -> Transform
    {
        let my_transform = Transform::from_translation(vec3(x, y, 12.));
        my_transform.with_scale(vec3(0.5, 0.5, 1.0))
    }

    pub fn spawn_resources(commands: & mut Commands, vec_entity: &Vec<Entity>, asset_server: &Res<AssetServer>, query : &Query<(&Position, With<Tile>)>, res: Ressource)
    {
        // let vec_entity = collect_id;
        // let vec_id = &vec_entity.vec_id;
        // let random_id = vec_id[0][0];
        // let entity_first_case = commands.entity(random_id);
        
        for (position) in query.iter() {
            if position.0.0.x == (res.x * TILES_WIDTH as i32) as f32 && position.0.0.y == (res.y * TILES_WIDTH as i32) as f32
            {
                commands.spawn((
                    SpriteBundle{
                        texture: asset_server.load("Ressource.png"),
                        transform: transform_for_ressource(position.0.0.x, position.0.0.y),
                        ..Default::default()
                    })
                );
            }
            println!("position: {:?}", position);
        }
    

    }
    
    pub fn dispatch_event(mut commands: Commands, asset_server: Res<AssetServer>, mut reader: EventReader<StreamEvent>, query : Query<(&Position, With<Tile>)>)
    {
        let mut vec_map_entity: Vec<Entity> = vec![];
        for (_, event) in reader.read().enumerate() {
            let x = &event.0;
            match x
            {
                crate::Parse::Map(x, y) => {
                    spawn_map(*x, *y, & mut commands, &asset_server);
                    for entity in &vec_map_entity{
                        println!("{:?}", entity);
                    }
                }
                crate::Parse::Content_case(x, y, n, l, d, s,m , ph, th) => {
                    let ressource = Ressource{ x: *x, y: *y, n: *n, l: *l, d: *d, s: *s, m: *m, ph: *ph, th: *th };
                    spawn_resources(& mut commands, &vec_map_entity, &asset_server, &query, ressource);

                }
                // crate::Parse::Movemement(_, _, _) => todo!(),
            }
            // let y = event.0;
        }
        // let s: SpriteBundle = SpriteBundle{
            //     texture: texture_handle.clone(),
            //             transform: Transform::from_xyz(orig_x as f32 + i_f32, 0., 10.),
            //             ..default()
            // }
        // let start = - MAP_WIDTH as i32 / 2 as i32;
        // let end = MAP_WIDTH as i32 / 2;
        // let orig_x = 0;
        // let orig_y = 0;
        // let mut tiles = Vec::new();
        // let ex = spawn_ascii_sprite()
        
    }
     
}
