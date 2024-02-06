pub mod env{
    use std::collections::HashMap;

    use bevy::{ecs::{system::{Resource, ResMut, Res}, entity::Entity}, math::Vec2, sprite::TextureAtlas, asset::{Assets, AssetServer}};

    use crate::{Ressource::Ressource::ContentCase, sprite_player::sprite_player::{SpriteAnimation, set_sprite_animation}, TILES_WIDTH, dispatch::dispatch::{SIZE_VECSPRITE, VECSPRITE, VECEXPULSION, Player, Playable}};

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
        pub vec_sprite_player_mvmt: Vec<Vec<SpriteAnimation>>, // vec<vec<sprite>>> ?
        pub vec_sprite_player_expulsion: Vec<Vec<SpriteAnimation>>, // vec<vec<sprite>>> ?
        pub last_event_id_visited: usize,
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
            // //println!("{:?}", playable_opt);
            let entity;
            if let Some(playable) = playable_opt{
                entity = match playable{
                    Playable::Player(player) => player.player_entity,
                    Playable::Egg(_) => panic!(), // for now
                }
            }
            else{
                panic!(); // explicit panic si chelsea est premier player et arsenal second ?? 
            }
            entity
        }

        pub fn get_player_num_team(&self, id_game: &u8) -> u8
        {
            let playable_opt = self.player_id.get(&id_game);
            //println!("{:?}", playable_opt);
            let num_team;
            if let Some(playable) = playable_opt{
                num_team = match playable{
                    Playable::Player(player) => player.num_team,
                    Playable::Egg(_) => panic!(), // for now
                }
            }
            else{
                panic!();
            }
            num_team
        }

        pub fn get_num_team(&self, name_team: &String) -> Option<u8>
        {
            let iter = self.name_equipe.iter().enumerate(); 
            for (nbr_iter, name) in iter{
                //println!("name_team {} name {} nbr_iter {} ", name_team, name, nbr_iter);
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

//        

        pub fn set_sprites_mvmt(& mut self, mut texture_atlases: & mut ResMut<Assets<TextureAtlas>>, asset_server: &Res<AssetServer>, name_team: &String) //(nbr_teams)
        {
            let num_team = self.get_num_team(name_team).unwrap() as usize;
            for i in 0..SIZE_VECSPRITE
            {
                self.vec_sprite_player_mvmt[num_team].push(set_sprite_animation(num_team, i as u8, texture_atlases, asset_server, VECSPRITE))
            }
            for i in 0..SIZE_VECSPRITE
            {
                self.vec_sprite_player_expulsion[num_team].push(set_sprite_animation(num_team, i as u8, texture_atlases, asset_server, VECEXPULSION))
            }
            // //println!("{:?}", self.vec_sprite_player_mvmt[num_team].len());
        }

        pub fn get_sprite(&self, indice: usize, num_team: usize) -> SpriteAnimation// (team, orientation)
        {
            println!("sprite num_team {} indice {}", num_team, indice); //num_team 1 
            self.vec_sprite_player_mvmt[num_team][indice].clone()
        }

        pub fn get_sprite_expulsion(&self, indice: usize, num_team: usize) -> SpriteAnimation// (team, orientation)
        {
            // println!("num_team {} indice {}", num_team, indice); //num_team 1 
            self.vec_sprite_player_expulsion[num_team][indice].clone()
        }

        pub fn set_new_entry_hashmap_player(&mut self, id: &u8, level: &u8, team: String, num_team: u8, entity: Entity)
        {
            let player = Player::new(num_team, level, team, entity);
            let playable = Playable::Player(player);
            println!("add Player {:?}", playable);
            self.player_id.insert(*id, playable);
            // let player = Playable::Player(())
        }
        // get all the component with id and orientation 
    }

}