pub mod map{
    use std::{io::Empty, vec};

    use bevy::{prelude::*, math::vec3};

    use crate::{MAP_WIDTH, TILES_WIDTH, TILES_HEIGHT, StreamEvent, StreamReceiver};

    #[derive(Component)]
    pub struct Tile(Vec3);
    
    #[derive(Component)]
    pub struct Map(Vec<Tile>);

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
        for from_stream in receiver.try_iter() {
            events.send(StreamEvent(from_stream));
        }
    }

    pub fn spawn_map(x: i32 , y: i32, commands: & mut Commands, asset_server: &Res<AssetServer>) -> Vec<Entity>
    {
        // let mut x: i32;
        // let mut y: i32;
            let mut vec_entity_map: Vec<Entity> = vec![];
            let texture_handle: Handle<Image> = asset_server.load("lawn.png");
            let start_x = - (x * TILES_WIDTH as i32) / 2 as i32;
            let end_x = x * TILES_WIDTH as i32 / 2 - 1;
            let start_y = - (y * TILES_WIDTH as i32) / 2 as i32;
            let end_y = y * TILES_WIDTH as i32 / 2 - 1;
            let orig_x = 0;
            let orig_y = 0;

            for i in (start_x..=end_x).step_by(TILES_WIDTH as usize)
            {
                for j in (start_y..=end_y).step_by(TILES_HEIGHT as usize)
                {
                    let i_f32 = i as f32;
                    let j_f32 = j as f32;
                    let vec = Vec3::new(orig_x as f32 + i_f32, orig_y as f32 + j_f32, 10.);
                    vec_entity_map.push(commands.spawn((
                            Tile(vec),
                            SpriteBundle {
                                texture: texture_handle.clone(),
                                transform: Transform::from_translation(vec),
                                ..default()
                            }
                        )
                    ).id());
                }
            }
            // let child = commands.spawn(
            //         SpriteBundle {
            //             texture: asset_server.load("Ressource.png"),
            //             transform: Transform::from_xyz(100., 0., 0.),
            //             ..default()
            //         }
                
            // ).id();
            // println!("{:?}", child);
            vec_entity_map
        }

    pub fn spawn_resources(commands: & mut Commands, vec_entity: &Vec<Entity>, asset_server: &Res<AssetServer>)
    {
        println!("{:?}", vec_entity);
        println!("WESHHHHHHHHHHHHHHHHHHHHHH");
        for i in vec_entity{
            commands.entity(*i).with_children(|parent|
                {
                    parent.spawn(SpriteBundle{
                        texture: asset_server.load("Ressource.png"),
                        transform: Transform::from_xyz(100., 0., 0.),
                        ..default()
                    });
                }

            );
        }
    }
    
    pub fn dispatch_event(mut commands: Commands, asset_server: Res<AssetServer>, mut reader: EventReader<StreamEvent>)
    {
        let mut vec_map_entity: Vec<Entity> = vec![];
        for (_, event) in reader.read().enumerate() {
            let x = &event.0;
            match x
            {
                crate::Parse::Map(x, y) => vec_map_entity = spawn_map(*x, *y, & mut commands, &asset_server),
                crate::Parse::Content_case(x, y, n, l, d, s,m , ph, th) => {
                    spawn_resources(& mut commands, &vec_map_entity, &asset_server);

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
