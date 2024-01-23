pub mod dispatch{
    use std::{collections::HashMap, default};

    use bevy::{ecs::{system::{Resource, Commands, Res, ResMut, Query}, entity::Entity, event::{EventWriter, EventReader, Events}, schedule::{States, IntoSystemConfigs, common_conditions::in_state, NextState, State}}, math::Vec2, app::{Plugin, App, Startup, Update}, asset::{AssetServer, Assets, self}, sprite::{TextureAtlas, SpriteSheetBundle}, transform::components::Transform, prelude::default, utils::tracing::Event};

    use crate::{TILES_WIDTH, StreamEvent, StreamReceiver, map::map::spawn_map, Ressource::Ressource::{anim_take_ressource_res, get_ressource_entity, spawn_resources, ContentCase, Ressource}, sprite_player::{sprite_player::{setup_sprite, SpriteAnimation, SpriteComponent, set_sprite_animation, Cell}, self}, do_action::do_action::{ActionPlayer, TypeAction, add_action, get_nbr_player_cell}, parser::parser::Parse, env::env::RessCommandId};

    // const for teams folder name 
    pub const SIZE_VECSPRITE: usize = 4;
    pub const SIZE_VECTEAM: usize = 2;
    pub const VECSPRITE: [&'static str; SIZE_VECSPRITE] = ["zelda_up2.png", "zelda_east.png", "zelda_down.png", "zelda_west.png"];
    pub const VECEXPULSION: [&'static str; SIZE_VECSPRITE] = ["expulse_up.png", "expulse_east.png", "expulse_down.png", "zelda_west.png"]; // miss expulse west
    pub const VECTEAM: [&'static str; SIZE_VECTEAM] = ["zelda_1", "zelda_2"];
    
    #[derive(Debug)]
    pub enum Playable{
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



    pub struct Dispatch;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
    pub enum StateCommand {
        #[default]
        Simple_Command,
        Stacking_Command,
        Complexe_Command,
    }
    #[derive(Resource)]
    pub struct Complexcommand{
        pub nbr_command: u8,
        pub vec_id_command: Vec<usize>,
        pub first_command: Parse,
        pub vec_command: Vec<Parse>,
        // pub vec_command: Vec<(Parse, u32)>,
    }

    impl Plugin for Dispatch{
        fn build(&self, app: &mut App) {
            app
            .add_state::<StateCommand>()
            .add_event::<StreamEvent>()
            .insert_resource(Complexcommand { nbr_command: 1,vec_id_command: vec![],  first_command: Parse::Donothing, vec_command: vec![] })

            .add_systems(Startup, init)
            .add_systems(Update, read_stream)
            // .add_systems(Update, dispatch_init_complex_event.run_if(in_state(StateCommand::Simple_Command)))
            .add_systems(Update, dispatch_setup_event.run_if(in_state(StateCommand::Simple_Command)))
            .add_systems(Update, dispatch_action_event.run_if(in_state(StateCommand::Simple_Command)))
            .add_systems(Update,  dispatch_stacking_command.run_if(in_state(StateCommand::Stacking_Command)))
            .add_systems(Update, dispatch_handle_complex_command.run_if(in_state(StateCommand::Complexe_Command)));
            // .add_systems(Update, dispatch_action_event.run_if(in_state(StateCommand::Simple_Command)));
            /*.init_state::<AppState>()
        ) */
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
            vec_sprite_player_mvmt: vec![], 
            vec_sprite_player_expulsion: vec![], 
            nbr_equipe: 0, 
            name_equipe: vec![],
            last_event_id_visited: 0,
        });
        // let toto = world.query(Query<)
    }

    fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>, state: Res<State<StateCommand>>) {
        for parse in receiver.try_iter() {
            println!("state {:?}", state.get());
            // println!("{:?}", events.g);
            
            events.send(StreamEvent(parse));
        }
        // events.send(StreamEvent(receiver.iter().next().unwrap()));
    }

    


    pub fn dispatch_setup_event(
        mut commands: Commands, //spawn des entity 
        asset_server: Res<AssetServer>, // ptr sur env 
        mut reader: EventReader<StreamEvent>, // event (interrupts)
        mut asset_map: ResMut<RessCommandId>, // ptr sur env 
        mut texture_atlases: ResMut<Assets<TextureAtlas>> // ressource 
    )
{
    for event in reader.read_with_id()
    {
        let parse = &event.0.0;
        let streamevent = event.1;
        // let x = &event.0;
        match parse
        {
            crate::Parse::Map(x, y) => {
                spawn_map(*x, *y, & mut commands, &asset_server, & mut asset_map);
                asset_map.set_x_y_pixel(*x, *y);
                asset_map.set_hashmap_ressource(*x, *y);
                asset_map.last_event_id_visited = streamevent.id;
            }
            crate::Parse::Time(t) => {
                asset_map.time = *t;
                asset_map.last_event_id_visited = streamevent.id;
            }
            crate::Parse::RessourceCase(x, y, n, l, d, s,m , ph, th) => {
                let (x_rel, y_rel) = asset_map.center_map_new_system(*x as f32, *y as f32);
                let ressource = Ressource{ x_rel: x_rel, x: *x, y: *y, y_rel: y_rel, n: *n, l: *l, d: *d, s: *s, m: *m, ph: *ph, th: *th};
                spawn_resources(& mut commands, &asset_server, ressource, & mut asset_map.id_Ressource);
                asset_map.last_event_id_visited = streamevent.id;
            }
            crate::Parse::NomEquipe(n) => {
                asset_map.name_equipe.push((*n.clone()).to_string());
                asset_map.nbr_equipe += 1;
                asset_map.vec_sprite_player_mvmt.push(vec![]);
                asset_map.vec_sprite_player_expulsion.push(vec![]);
                asset_map.set_sprites_mvmt(&mut texture_atlases, &asset_server, n);
                asset_map.last_event_id_visited = streamevent.id;
                 // doit dependre de la team 
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
                asset_map.last_event_id_visited = streamevent.id;
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
        mut complexcommand: ResMut<Complexcommand>,
        mut query_player_cell: Query<(Entity, &Cell)>,
        mut statecommand: ResMut<NextState<StateCommand>>,
        mut state: Res<State<StateCommand>>,
        
    )
    {
        
        // println!("reader {:?}", reader.read_with_id());
        for event in reader.read_with_id() {
            let parse = &event.0.0;
            let streamevent = event.1;
            println!("PARSE {:?}", parse);
            // let last_id = reader.read_with_id().map(|x | )
            // let event = event;
            if let StateCommand::Simple_Command = state.get() 
            {
                if streamevent.id > asset_map.last_event_id_visited
                {
                    match parse
                    {
                        Parse::Expulse(id) => {
                            // cherche le nb de joueurs concerne par expulse equivalant au nombre de commande a attendre 
                            let nbr_mov_command_waited = get_nbr_player_cell(& mut query_player_cell, asset_map.get_player_id(&id));  
                            println!("EXPULSE {:?}", statecommand);
                            complexcommand.first_command = (event.0.0).clone();
                            complexcommand.nbr_command = nbr_mov_command_waited;
                            println!("nbr_command_wanted {} streamevent_id {:?}", nbr_mov_command_waited, streamevent.id);
                            let first_id = streamevent.id + 1;
                            println!("first_id {:?}", first_id);
                            let last_id = streamevent.id + nbr_mov_command_waited as usize + 1;
                            println!("last_id {:?}", last_id);
                            for i in first_id..last_id
                            {
                                complexcommand.vec_id_command.push(i); 
                            }
                            statecommand.set(StateCommand::Stacking_Command);
                            println!("EXPULSE {:?}", statecommand);
                            // println!("reader {:?}", reader.read().map(||));
                            println!("streamevent {:?}", streamevent);
                            asset_map.last_event_id_visited = streamevent.id;
                            
                            return ;
                        }

                        Parse::Prend(_, _) => {
                            let nbr_mov_command_waited = 2;
                            complexcommand.first_command = (event.0.0).clone();
                            complexcommand.nbr_command = nbr_mov_command_waited;
                            let first_id = streamevent.id + 1;
                            let last_id = streamevent.id + nbr_mov_command_waited as usize + 1;
                            for i in first_id..last_id
                            {
                                complexcommand.vec_id_command.push(i);
                                
                            }
                            println!("nbr_command_wanted {} streamevent_id {:?}", nbr_mov_command_waited, streamevent.id);
                            statecommand.set(StateCommand::Stacking_Command);
                            asset_map.last_event_id_visited = streamevent.id;
                            return ;

                        }
        
                        Parse::MovementPlayer(id, x, y, o) =>{
                            println!("MOVEMENT ? STATE {:?}, INDEX {}, Parse {:?}", state.get(), streamevent.id, parse);
                            // let id_back = *id - 1; //method to get the id is wrong because if a player died the index of vector won't be reliable anymore (like arsenal_id [1, 2] chelsea_id [3, 4] => arsenal_id [1, 2] chelsea_id [3], => arsenal_id [1, 2, 5(egg)] chelsea_id [3])
                            let mut mov = TypeAction::Movement{0: *x, 1: *y, 2: *o};
        
                            add_action(& mut query_action_player, &asset_map.get_player_id(&id), mov);
                            asset_map.last_event_id_visited = streamevent.id;

                        }
                        _ => ()
                    }
                }

            }
        }
    }

    pub fn dispatch_stacking_command(
        mut reader: EventReader<StreamEvent>,
        mut asset_map: ResMut<RessCommandId>,
        mut query_player_cell: Query<(Entity, &Cell)>,
        mut complexcommand: ResMut<Complexcommand>,
        mut statecommand: ResMut<NextState<StateCommand>>,
    )
    {
        // pour l instant generic aux 3 complexes events: Expulse(nb: nb de joueurs concerne, type: Movement), Prend et Pose (nb: 2, type: inventaire et contenu de la case)
        
        
        // if complexcommand.nbr_command as usize > complexcommand.vec_command.len()
        // {
            let mut x = &Parse::Donothing;
            let events = reader.read_with_id();
            for event in events{
                let parse = &event.0.0;
                complexcommand.vec_command.push(parse.clone());
                // let streamevent = event.1;
                // for id in complexcommand.vec_id_command.clone(){
                //     println!("stacking event_id {:?}", streamevent);
                //     println!("stacking parse {:?}", parse);
                //     if streamevent.id == id{
                //         complexcommand.vec_command.push(parse.clone());
                //         asset_map.last_event_id_visited = streamevent.id;
                //     }
                //     // if id > streamevent.id{
                //     //     panic!("ah bah cho un peu")
                //     //     // println!("{:?}", &events);
                //     // }
                // }
            }
            // for event in reader.read(){
            //     x = &event.0; //va jusqu a la fin parce que pour une raison que j ignore le reader.read() stack les fucking events .... 
            // }
            // if let Parse::MovementPlayer(_, _, _, _) = x{
                // complexcommand.vec_command.push(x.clone());
            // }
            statecommand.set(StateCommand::Stacking_Command);
        
        if reader.is_empty() 
        {
            
            println!("pb stacking command ? {:?}", complexcommand.vec_command);
            println!(" asset_map.last_event_id  {:?}", asset_map.last_event_id_visited);
            // asset_map.last_event_id_visited = streamevent.id;
        
             println!("stacking {:?}", complexcommand.vec_id_command);
            println!("complexcommand.nbr_command {:?}", complexcommand.nbr_command);
            println!("complexcommand.vec_command.len() {:?}", complexcommand.vec_command.len());
        
            println!("tab {:?} len {}", complexcommand.vec_command, complexcommand.nbr_command);
            statecommand.set(StateCommand::Complexe_Command);
        }
        // CECI EST LA VERSION FINAL, COMME LA FONCTION DEVRAIT S IMPLEMENTER A LA FIN
        // AU DESSUS C EST JUSTE UNE VERSION POUR TESTER LE CHANGEMENT DE SPRITE
        // MAIS COMME EXPULSE EST MAL IMPLEMENTE DANS SERVER ON S ADAPTE
        // else if complexcommand.nbr_command as usize == complexcommand.vec_command.len()
        // {
        //     statecommand.set(StateCommand::Complexe_Command);
        // }
        // else {
        //     panic!("ERROR in changing state command: we should not be here")
        // } 
    }

    pub fn dispatch_handle_complex_command(
        mut commands: Commands,
        mut asset_map: ResMut<RessCommandId>,
        mut complexcommand: ResMut<Complexcommand>,
        mut statecommand: ResMut<NextState<StateCommand>>,
        mut query_action_player: Query<& mut ActionPlayer>,
        mut query_transform_res: Query<&mut Transform>,
    )
    {
        
        match complexcommand.first_command
        {
            // expulse est un evenement dont le nb de commandes n'est pas connu a l'avance 
            Parse::Expulse(id_1) => {
                // on iter donc sur tous les movements precedemments sauvegarde
                for mov_expulse in &complexcommand.vec_command
                {
                    // un if let semblerai plus approprie 
                    match mov_expulse {
                        Parse::MovementPlayer(id, x, y, o) => {
                            // PAREIL CETTE CONDITION EST JUSTE LA PARCE QUE EXPULSE EST MAL IMPLEMENTE DANS SERVER
                            if id_1 != *id { 
                                // on recupere 
                                let num_team = asset_map.get_player_num_team(id);
                                let sprite_anim_mvmt = asset_map.get_sprite((*o - 1 ) as usize, num_team as usize);
                                let sprite_anim_expulse = asset_map.get_sprite_expulsion((*o - 1 ) as usize, num_team as usize);
                                let expulsion = TypeAction::Expulsion{0: *x, 1: *y, 2: *o, 3: sprite_anim_mvmt, 4: sprite_anim_expulse};
                                println!("do_action {:?}", mov_expulse);
                                add_action(& mut query_action_player, &asset_map.get_player_id(id), expulsion);
                                
                            }
                        }
                        _ => panic!("on ne devrai qu'avoir des movement"),
                    }
                }
                
            }
            Parse::Prend(num_player, num_res) => {
                if let Parse::Inventaire(_, _, _, _, _, _, _, _, _, _) = complexcommand.vec_command[0] 
                {
                    if let Parse::RessourceCase(x, y, n, l, d, s,m , ph, th) = complexcommand.vec_command[1]{
                        println!("Ressource?");
                        let (x_rel, y_rel) = asset_map.center_map_new_system(x as f32, y as f32);
                        let res = Ressource{ x_rel: x_rel, x: x, y: y, y_rel: y_rel, n: n, l: l, d: d, s: s, m: m, ph: ph, th: th};
                        let res_entity = get_ressource_entity(& mut commands,  &res, num_res, & mut asset_map.id_Ressource);
                        anim_take_ressource_res(&mut query_transform_res, &res_entity, res);
                    }
                }
            }
            
            _ => ()
        }
        statecommand.set(StateCommand::Simple_Command);
        //need to set complexcommand back to normal 
        // miss the reset method for complexcommand
    }

   
}