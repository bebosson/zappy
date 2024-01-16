pub mod dispatch{
    use std::collections::HashMap;

    use bevy::{ecs::{system::{Resource, Commands, Res, ResMut, Query}, entity::Entity, event::{EventWriter, EventReader}}, math::Vec2, app::{Plugin, App, Startup, Update}, asset::{AssetServer, Assets, self}, sprite::{TextureAtlas, SpriteSheetBundle}, transform::components::Transform, prelude::default};

    use crate::{TILES_WIDTH, StreamEvent, StreamReceiver, map::map::spawn_map, Ressource::Ressource::{Ressource, spawn_resources, ContentCase}, sprite_player::{sprite_player::{setup_sprite, SpriteAnimation, SpriteComponent, set_sprite_animation, Cell}, self}, do_action::do_action::{ActionPlayer, TypeAction, add_action}};

    // const for teams folder name 
    pub const SIZE_VECSPRITE: usize = 4;
    pub const SIZE_VECTEAM: usize = 2;
    pub const VECSPRITE: [&'static str; SIZE_VECSPRITE] = ["zelda_up2.png", "zelda_east.png", "zelda_down.png", "zelda_west.png"];
    pub const VECTEAM: [&'static str; SIZE_VECTEAM] = ["zelda_1", "zelda_2"];
    
    #[derive(Debug)]
    enum Playable{
        Player(Player),
        Egg(Egg)
    }
    #[derive(Debug)]
    pub struct Egg{
        pub egg_entity: Entity,
    }
    #[derive(Debug)]
    pub struct Player{
        pub num_team : u8,
        pub name_team: String,
        pub level: u8,
        pub inventory: Option<Ressource>,
        pub player_entity: Entity,
    }

    impl Player{
        pub fn new(level: &u8, team: String, entity: Entity) -> Self
        {
            Player{
                num_team: 0,
                name_team: team,
                level: *level,
                inventory: None,
                player_entity: entity,
            }
        }
    }


    #[derive(Resource)]
    pub struct RessCommandId{
        pub x: u32,
        pub y: u32,
        pub pixel_x_max: f32,
        pub pixel_y_max: f32,
        pub pixel_x_min: f32,
        pub pixel_y_min: f32,
        pub time: u32,
        pub nbr_equipe: u8,
        pub name_equipe: Vec<String>, 
        pub id_Ressource: Vec<Vec<HashMap<usize, ContentCase>>>, //vec<vec<hashmap<id, vec<entity>>>>
        // pub player_id: Vec<Entity>,
        pub player_id: HashMap<u8, Playable>, // hashmap<id, playable>
        pub vec_sprite_player: Vec<Vec<SpriteAnimation>>, // vec<vec<sprite>>> ?
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

        pub fn set_x_y_pixel(&mut self, x_dim: u32, y_dim: u32)
        {
            let (x_pixel, y_pixel) = self.center_map_new_system((x_dim - 1) as f32 , 0.);
            self.pixel_y_max = y_pixel + TILES_WIDTH / 2.; 
            self.pixel_x_max = x_pixel + TILES_WIDTH / 2.; 
            let (x_pixel, y_pixel) = self.center_map_new_system(0. , (y_dim - 1) as f32);
            self.pixel_x_min = x_pixel - TILES_WIDTH / 2.;
            self.pixel_y_min = y_pixel - TILES_WIDTH / 2.;
        }

        pub fn set_hashmap_ressource(& mut self, x_dim: u32, y_dim: u32)
        {
            let x_size: usize;
            let y_size: usize;
            // let vec_new: Vec<Vec<HashMap<u32,ContentCase>>>;
            if x_dim > 0
            {
                x_size = (x_dim - 1) as usize;
            }
            else
            {
                x_size = x_dim as usize;
            }
            if y_dim > 0
            {
                y_size = (y_dim - 1) as usize;
            }
            else
            {
                y_size = y_dim as usize;
            }
            // self.id_Ressource = Vec::with_capacity(y_size + 1);
            
            for y in 0..y_size + 1
            {
                self.id_Ressource.push(vec![]); 
                for _ in 0..x_size + 1 
                {
                    self.id_Ressource[y].push(HashMap::new());
                }
            }
        
        }

        pub fn get_player_id(&self, id_game: &u8) -> Entity
        {
            let playable_opt = self.player_id.get(&id_game);
            println!("{:?}", playable_opt);
            let entity;
            if let Some(playable) = playable_opt{
                entity = match playable{
                    Playable::Player(player) => player.player_entity,
                    Playable::Egg(_) => panic!(), // for now
                }
            }
            else{
                panic!();
            }
            entity
        }

        pub fn get_num_team(&self, name_team: &String) -> Option<u8>
        {
            let iter = self.name_equipe.iter().enumerate(); 
            for (nbr_iter, name) in iter{
                println!("name_team {} name {} nbr_iter {} ", name_team, name, nbr_iter);
                if name.eq(name_team){
                    return Some(nbr_iter as u8)
                }
            }
            None
            
            // for (name, i) in self.name_equipe.enumerate(){
                // if (name_team.eq(i)){
// 
                // }
            // }
        }

        pub fn set_sprites_team(& mut self, mut texture_atlases: & mut ResMut<Assets<TextureAtlas>>, asset_server: &Res<AssetServer>, name_team: &String) //(nbr_teams)
        {
            let num_team = self.get_num_team(name_team).unwrap() as usize;
            for i in 0..SIZE_VECSPRITE
            {
                self.vec_sprite_player[num_team].push(set_sprite_animation(num_team, i as u8, texture_atlases, asset_server))
            }
            println!("{:?}", self.vec_sprite_player[num_team].len());
        }

        pub fn get_sprite(&self, indice: usize, num_team: usize) -> SpriteAnimation// (team, orientation)
        {
            println!("num_team {} indice {}", num_team, indice); //num_team 1 
            self.vec_sprite_player[num_team][indice].clone()
        }

        pub fn set_new_entry_hashmap_player(&mut self, id: &u8, level: &u8, team: String, entity: Entity)
        {
            let player = Player::new(level, team, entity);
            let playable = Playable::Player(player);
            self.player_id.insert(*id, playable);
            // let player = Playable::Player(())
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
        commands.insert_resource(
            RessCommandId{
            x: 0,
            y: 0,
            pixel_x_max: 0.,
            pixel_y_max: 0.,
            pixel_x_min: 0., 
            pixel_y_min: 0., 
            time: 0,
            id_Ressource: vec![], 
            player_id: HashMap::new(), 
            vec_sprite_player: vec![], 
            nbr_equipe: 0, 
            name_equipe: vec![] 
        });
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
                spawn_map(*x, *y, & mut commands, &asset_server, & mut asset_map);
                asset_map.set_x_y_pixel(*x, *y);
                asset_map.set_hashmap_ressource(*x, *y);
            }
            crate::Parse::Time(t) => {
                asset_map.time = *t;
            }
            crate::Parse::RessourceCase(x, y, n, l, d, s,m , ph, th) => {
                let (x_rel, y_rel) = asset_map.center_map_new_system(*x as f32, *y as f32);
                let ressource = Ressource{ x_rel: x_rel, x: *x, y: *y, y_rel: y_rel, n: *n, l: *l, d: *d, s: *s, m: *m, ph: *ph, th: *th};
                spawn_resources(& mut commands, &asset_server, ressource, & mut asset_map.id_Ressource);
            }
            crate::Parse::NomEquipe(n) => {
                asset_map.name_equipe.push((*n.clone()).to_string());
                asset_map.nbr_equipe += 1;
                asset_map.vec_sprite_player.push(vec![]);
                asset_map.set_sprites_team(&mut texture_atlases, &asset_server, n); // doit dependre de la team 
            }
            crate::Parse::ConnexionPlayer(id, x, y, o, l, n) => {
                // std::process::exit(1);
                let (x_rel, y_rel) = asset_map.center_map_new_system(*x as f32, *y as f32);
                let team_name = n.to_string();
                let team_num = asset_map.get_num_team(&team_name).unwrap() as usize;
                let player_animation = asset_map.get_sprite((*o - 1) as usize, team_num);
                // let player_component = animation_to_sprite_component(, x, y)
                let entity = setup_sprite(& mut commands, &asset_server, (x_rel, y_rel),(*x, *y, *o), & mut asset_map, player_animation, team_num as u8);
                asset_map.set_new_entry_hashmap_player(id, l, team_name, entity);
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
                    // let id_back = *id - 1; //method to get the id is wrong because if a player died the index of vector won't be reliable anymore (like arsenal_id [1, 2] chelsea_id [3, 4] => arsenal_id [1, 2] chelsea_id [3], => arsenal_id [1, 2, 5(egg)] chelsea_id [3])
                    let mut mov = TypeAction::Movement{0: *x, 1: *y, 2: *o};

                    add_action(& mut query_action_player, &asset_map.get_player_id(id), mov);
                }
                _ => ()
            }
        }
    }
}