pub mod map{
    use std::io::Empty;

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
            .add_systems(Update, (read_stream, spawn_map));
        }
    }

    fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
        for from_stream in receiver.try_iter() {
            events.send(StreamEvent(from_stream));
        }
    }
    
    pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>, mut reader: EventReader<StreamEvent>)
    {
        let texture_handle: Handle<Image> = asset_server.load("lawn.png");
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
        let mut x: i32;
        let mut y: i32;

        for (_, event) in reader.read().enumerate() {
            x = event.0.0;
            y = event.0.1;
        

            let start = - (x * TILES_WIDTH as i32) / 2 as i32;
            let end = x * TILES_WIDTH as i32 / 2;
            let orig_x = 0;
            let orig_y = 0;

            for i in (start..=end).step_by(TILES_WIDTH as usize)
            {
                for j in (start..=end).step_by(TILES_HEIGHT as usize)
                {
                    let i_f32 = i as f32;
                    let j_f32 = j as f32;
                    let vec = Vec3::new(orig_x as f32 + i_f32, orig_y as f32 + j_f32, 10.);
                    commands.spawn((
                            Tile(vec),
                            SpriteBundle {
                                texture: texture_handle.clone(),
                                transform: Transform::from_translation(vec),
                                ..default()
                            }
                        ) 
                    );

                }


            }
        }
    }
     
}
