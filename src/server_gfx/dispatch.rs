pub mod dispatch{
    use bevy::{ecs::{system::{Resource, Commands, Res, ResMut, Query}, entity::Entity, event::{EventWriter, EventReader}}, math::Vec2, app::{Plugin, App, Startup, Update}, asset::{AssetServer, Assets, self}, sprite::{TextureAtlas, SpriteSheetBundle}, transform::components::Transform, prelude::default};

    use crate::{TILES_WIDTH, StreamEvent, StreamReceiver, map::map::spawn_map, Ressource::Ressource::{Ressource, spawn_resources}, sprite_player::{sprite_player::{setup_sprite, SpriteAnimation, SpriteComponent, set_sprite_animation, Cell}, self}, do_action::do_action::{ActionPlayer, TypeAction, add_action}};

    // const for teams folder name 
    pub const SIZE_VECSPRITE: usize = 3;
    pub const VECSPRITE: [&'static str; SIZE_VECSPRITE] = ["zelda_up2.png", "zelda_east.png", "zelda_down.png"];

    #[derive(Resource)]
    pub struct RessCommandId{
        pub x: u32,
        pub y: u32,
        pub pixel_x_max: f32,
        pub pixel_y_max: f32,
        pub pixel_x_min: f32,
        pub pixel_y_min: f32,
        pub id_Ressource: Vec<Entity>,
        pub player_id: Vec<Entity>,
        pub vec_sprite_player: Vec<SpriteAnimation> // vec<vec<sprite>>> ?
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
            (x_new, y_new)
            // let x_rel: x_abs as i32
            
        }
        pub fn get_player_id(&self, id_game: &u8) -> Entity
        {
            self.player_id[*id_game as usize]
        }

        pub fn set_all_sprites(& mut self, mut texture_atlases: & mut ResMut<Assets<TextureAtlas>>, asset_server: &Res<AssetServer>) //(nbr_teams)
        {
            for i in 0..SIZE_VECSPRITE
            {
                self.vec_sprite_player.push(set_sprite_animation(0, i as u8, texture_atlases, asset_server))

            }
        }

        pub fn get_sprite(&self, indice: usize) -> SpriteAnimation// (team, orientation)
        {
            self.vec_sprite_player[indice].clone()
        }
        // get all the component with id and orientation 
    }

    pub struct Dispatch;

    impl Plugin for Dispatch{
        fn build(&self, app: &mut App) {
            app
            .add_event::<StreamEvent>()
            .add_systems(Startup, init)
            .add_systems(Update, read_stream)
            .add_systems(Update, dispatch_setup_event)
            .add_systems(Update, dispatch_action_event);
            // .add_systems(Update, print_resources);
        }
    }
    
    fn init(mut commands: Commands)
    {
        commands.insert_resource(RessCommandId{x: 0, y: 0, pixel_x_max: 0., pixel_y_max: 0., pixel_x_min: 0., pixel_y_min: 0., id_Ressource: vec![], player_id: vec![], vec_sprite_player: vec![]});
        // let toto = world.query(Query<)
    }

    fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
        for parse in receiver.try_iter() {
            events.send(StreamEvent(parse));
        }
    }

    

    pub fn dispatch_setup_event(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut reader: EventReader<StreamEvent>,
        mut asset_map: ResMut<RessCommandId>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>
    )
{
    for (_, event) in reader.read().enumerate()
    {
        let x = &event.0;
        match x
        {
            crate::Parse::Map(x, y) => {
                spawn_map(*x as u32, *y as u32, & mut commands, &asset_server, & mut asset_map);
                let (x_pixel, y_pixel) = asset_map.center_map_new_system((*x - 1) as f32 , 0.);
                asset_map.pixel_y_max = y_pixel + TILES_WIDTH / 2.; 
                asset_map.pixel_x_max = x_pixel + TILES_WIDTH / 2.; 
                let (x_pixel, y_pixel) = asset_map.center_map_new_system(0. , (*y - 1) as f32);
                asset_map.pixel_x_min = x_pixel - TILES_WIDTH / 2.;
                asset_map.pixel_y_min = y_pixel - TILES_WIDTH / 2.;
            }
            crate::Parse::RessourceCase(x, y, n, l, d, s,m , ph, th) => {
                let (x_rel, y_rel) = asset_map.center_map_new_system(*x as f32, *y as f32);
                let ressource = Ressource{ x: x_rel as i32, y: y_rel as i32, n: *n, l: *l, d: *d, s: *s, m: *m, ph: *ph, th: *th };
                spawn_resources(& mut commands, &asset_server, ressource);
            }
            crate::Parse::ConnexionPlayer(id, x, y, o, L, N) => {
                
                asset_map.set_all_sprites(&mut texture_atlases, &asset_server);
                let (x_rel, y_rel) = asset_map.center_map_new_system(*x as f32, *y as f32);
                let player_animation = asset_map.get_sprite((*o - 1) as usize);
                // let player_component = animation_to_sprite_component(, x, y)
                setup_sprite(& mut commands, &asset_server, & mut texture_atlases, (x_rel, y_rel),(*x, *y, *o), & mut asset_map, player_animation);
            }
            _ => ()
        }
    }
}



    pub fn dispatch_action_event(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut reader: EventReader<StreamEvent>,
        mut asset_map: ResMut<RessCommandId>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut query_action_player: Query<& mut ActionPlayer>,
    )
    {
        let mut vec_map_entity: Vec<Entity> = vec![];
        for (_, event) in reader.read().enumerate() {
            let x = &event.0;
            match x
            {
                crate::Parse::MovementPlayer(id, x, y, o) =>{
                    // println!("{:?}", asset_map);
                    let id_back = *id - 1;
                    let mut mov = TypeAction::Movement{0: *x, 1: *y, 2: *o};

                    add_action(& mut query_action_player, &asset_map.get_player_id(&id_back), mov);
                }
                _ => ()
            }
        }
    }
}