pub mod dispatch{
    use std::{collections::HashMap, default, process::ExitCode};

    use bevy::{ecs::{system::{Resource, Commands, Res, ResMut, Query}, entity::Entity, event::{EventId, EventReader, EventWriter, Events}, schedule::{States, IntoSystemConfigs, common_conditions::in_state, NextState, State}}, math::Vec2, app::{Plugin, App, Startup, Update}, asset::{AssetServer, Assets, self}, sprite::{TextureAtlas, SpriteSheetBundle}, transform::components::Transform, prelude::default, utils::tracing::Event};

    use crate::{do_action::do_action::{add_action, get_cell, get_nbr_player_cell, ActionPlayer, TypeAction}, env::env::RessCommandId, map::map::spawn_map, parser::parser::{simple_command, Parse}, sprite_player::{self, sprite_player::{set_sprite_animation, setup_sprite, Cell, SpriteAnimation, SpriteComponent}}, Ressource::Ressource::{anim_take_ressource_res, get_ressource_entity, spawn_resources, ContentCase, Ressource}, StreamEvent, StreamReceiver, TILES_WIDTH};

    // const for teams folder name 
    pub const SIZE_VECSPRITE: usize = 4;
    pub const SIZE_VECTEAM: usize = 2;
    pub const VECSPRITE: [&'static str; SIZE_VECSPRITE] = ["zelda_up2.png", "zelda_east.png", "zelda_down.png", "zelda_west.png"];
    pub const VECEXPULSION: [&'static str; SIZE_VECSPRITE] = ["expulse_up.png", "expulse_east.png", "expulse_down.png", "expulse_west.png"]; // miss expulse west
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
        pub fn new(num_team: u8, level: &u8, team: String, entity: Entity) -> Self
        {
            Player{
                num_team: num_team,
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
    }
    #[derive(Resource, Debug)]
    pub struct Complexcommand{
        // []
        pub nbr_command: u8,
        pub vec_id_command: Vec<usize>,
        pub first_command: Parse, //[expulse || pose || prend]
        pub vec_command: Vec<Parse>, // [Mov(1), Mov(2), ...] | [Inventaire, Ressource]
        // pub vec_command: Vec<(Parse, u32)>,
    }

    impl Complexcommand{
        pub fn fflush(& mut self)
        {
            self.nbr_command = 0;
            self.vec_id_command.clear();
            self.first_command = Parse::Donothing;
            self.vec_command.clear();
        }
    }

    impl Plugin for Dispatch{
        fn build(&self, app: &mut App) {
            app
            .add_state::<StateCommand>()
            .add_event::<StreamEvent>()
            .insert_resource(Complexcommand { nbr_command: 0,vec_id_command: vec![],  first_command: Parse::Donothing, vec_command: vec![] })

            .add_systems(Startup, init)
            .add_systems(Update, read_stream)
            .add_systems(Update, dispatch);
            // .add_systems(Update, dispatch_init_complex_event.run_if(in_state(StateCommand::Simple_Command)))
            // .add_systems(Update, dispatch_setup_event.run_if(in_state(StateCommand::Simple_Command)))
            // .add_systems(Update, dispatch_action_event.run_if(in_state(StateCommand::Simple_Command)))
            // .add_systems(Update,  dispatch_stacking_command.run_if(in_state(StateCommand::Stacking_Command)))
            // .add_systems(Update, dispatch_handle_complex_command.run_if(in_state(StateCommand::Complexe_Command)));
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
            // ////println!("{:?}", events.g);
            
            events.send(StreamEvent(parse));
        }
        // events.send(StreamEvent(receiver.iter().next().unwrap()));
    }
    

    pub fn dispatch(
        mut reader: EventReader<StreamEvent>,
        mut state: Res<State<StateCommand>>,
        mut next_state:  ResMut<'_, NextState<StateCommand>>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut asset_map: ResMut<RessCommandId>,
        mut complexcommand: ResMut<Complexcommand>,
        mut query_action_player: Query<& mut ActionPlayer>,
        mut query_player_cell: Query<(Entity, &Cell)>,
        mut query_transform_res: Query<&mut Transform>,
    )
    {
        for event in reader.read_with_id()
        {
            let state_enum = state.get();
            let parse = &event.0.0;
            let streamevent = event.1;
            ////println!("parse_dispatch {:?}", parse);
            ////println!("state_enum_dispatch {:?}", state_enum);
            match complexcommand.nbr_command // autant de complexe commande qu il y a de joueur ? (donc vec_complex command)
            {
                0 => {
                    if simple_command(&parse) {
                        //ici on execute la simple command sans changer d'etat (simple -> simple)
                        dispatch_setup_event(parse, & mut commands, &asset_server, & mut asset_map, & mut texture_atlases, & mut query_action_player);
                    }
                    else
                    {
                        dispatch_action_event(& mut asset_map, & mut complexcommand, & mut query_player_cell, & mut next_state, &parse, &streamevent);
                        //ici on set up la complexe command (expulse/prend/pose) et on change d'etat (simple -> stacking)
                    }
                },
                1.. => {
                    dispatch_stacking_command(&parse, &mut asset_map, &mut query_player_cell, &mut complexcommand, &mut next_state, &streamevent, &mut commands, &mut query_transform_res, &mut query_action_player);
                },
                // StateCommand::Complexe_Command => {
                //     dispatch_handle_complex_command(& mut commands, & mut asset_map, & mut complexcommand, & mut next_state, & mut query_action_player, & mut query_transform_res);
                // },
            }
        }
    }

    

    // [Map...enw, ...., ...., ..., ]
    pub fn dispatch_setup_event(
        parse: &Parse,
        commands: & mut Commands, //spawn des entity 
        asset_server: &Res<AssetServer>, // ptr sur env 
        asset_map: & mut ResMut<RessCommandId>, // ptr sur env 
        mut texture_atlases: & mut ResMut<Assets<TextureAtlas>>, // ressource
        mut query_action_player: & mut Query<& mut ActionPlayer>,
    )
{
        // let x = &event.0;
        match parse
        {
            crate::Parse::Map(x, y) => {
                spawn_map(*x, *y, commands, &asset_server, asset_map);
                asset_map.set_x_y_pixel(*x, *y);
                asset_map.set_hashmap_ressource(*x, *y);
                // asset_map.last_event_id_visited = streamevent.id;
            }
            crate::Parse::Time(t) => {
                asset_map.time = *t;
                // asset_map.last_event_id_visited = streamevent.id;
            }
            crate::Parse::RessourceCase(x, y, n, l, d, s,m , ph, th) => {
                let (x_rel, y_rel) = asset_map.center_map_new_system(*x as f32, *y as f32);
                let ressource = Ressource{ x_rel: x_rel, x: *x, y: *y, y_rel: y_rel, n: *n, l: *l, d: *d, s: *s, m: *m, ph: *ph, th: *th};
                spawn_resources(commands, &asset_server, ressource, & mut asset_map.id_Ressource);
                // asset_map.last_event_id_visited = streamevent.id;
            }
            crate::Parse::NomEquipe(n) => {
                asset_map.name_equipe.push((*n.clone()).to_string());
                asset_map.nbr_equipe += 1;
                asset_map.vec_sprite_player_mvmt.push(vec![]);
                asset_map.vec_sprite_player_expulsion.push(vec![]);
                asset_map.set_sprites_mvmt(&mut texture_atlases, &asset_server, n);
                // asset_map.last_event_id_visited = streamevent.id;
                 // doit dependre de la team 
            }
            crate::Parse::ConnexionPlayer(id, x, y, o, l, n) => {
                // std::process::exit(1);
                let (x_rel, y_rel) = asset_map.center_map_new_system(*x as f32, *y as f32);
                let team_name = n.to_string();
                let team_num = asset_map.get_num_team(&team_name).unwrap() as usize;
                let player_animation = asset_map.get_sprite((*o - 1) as usize, team_num);
                // let player_component = animation_to_sprite_component(, x, y)
                let entity = setup_sprite(commands, &asset_server, (x_rel, y_rel),(*x, *y, *o), asset_map, player_animation, team_num as u8);
                asset_map.set_new_entry_hashmap_player(id, l, team_name, team_num as u8,  entity);
                // asset_map.last_event_id_visited = streamevent.id;
            }
            Parse::MovementPlayer(id, x, y, o) =>{
                ////println!("MOVEMENT ? , Parse {:?}", parse);
                // let id_back = *id - 1; //method to get the id is wrong because if a player died the index of vector won't be reliable anymore (like arsenal_id [1, 2] chelsea_id [3, 4] => arsenal_id [1, 2] chelsea_id [3], => arsenal_id [1, 2, 5(egg)] chelsea_id [3])
                let mut mov = TypeAction::Movement{0: *x, 1: *y, 2: *o};

                add_action(query_action_player, &asset_map.get_player_id(&id), mov);
                // asset_map.last_event_id_visited = streamevent.id;

            }
            
            
            _ => ()
        }
    }



    // [Map , ... , Expulse/Prend/Incantation, ... ]
    pub fn dispatch_action_event(
        mut asset_map: & mut ResMut<RessCommandId>,
        mut complexcommand: & mut ResMut<Complexcommand>,
        mut query_player_cell: & mut Query<(Entity, &Cell)>,
        mut statecommand: & mut ResMut<NextState<StateCommand>>,
        parse: &Parse,
        streamevent: &EventId<StreamEvent>,
    )
    {
        
            // Complex command
            //first command => prend
            // nb_command = 2
            // 33
            // [34, 35]
            //
            match parse
            {
                Parse::Expulse(id) => {
                    // cherche le nb de joueurs concerne par expulse equivalant au nombre de commande a attendre 
                    let nbr_mov_command_waited = get_nbr_player_cell(& mut query_player_cell, asset_map.get_player_id(&id));  
                    ////println!("EXPULSE {:?}", statecommand);
                    ////println!("nbr_mov_command_waited {:?}", nbr_mov_command_waited);
                    complexcommand.first_command = parse.clone();
                    complexcommand.nbr_command = nbr_mov_command_waited;
                    let first_id = streamevent.id + 1;
                    let last_id = streamevent.id + nbr_mov_command_waited as usize + 1;
                    for i in first_id..last_id
                    {
                        complexcommand.vec_id_command.push(i); // [32, 33]
                    }
                    asset_map.last_event_id_visited = streamevent.id; //31 
                    statecommand.set(StateCommand::Stacking_Command);
                    
                    return ;
                }
                Parse::Prend(_, _) => {
                    let nbr_mov_command_waited = 2;
                    complexcommand.first_command = parse.clone();
                    complexcommand.nbr_command = nbr_mov_command_waited;
                    let first_id = streamevent.id + 1;
                    let last_id = streamevent.id + nbr_mov_command_waited as usize + 1;
                    for i in first_id..last_id
                    {
                        complexcommand.vec_id_command.push(i);
                        
                    }
                    // ////println!("nbr_command_wanted {} streamevent_id {:?}", nbr_mov_command_waited, streamevent.id);
                    asset_map.last_event_id_visited = streamevent.id;
                    statecommand.set(StateCommand::Stacking_Command);
                    return 
                }
                _ => ()
            }

    }

    pub fn dispatch_stacking_command(
        parse: &Parse,
        mut asset_map: & mut ResMut<RessCommandId>,
        query_player_cell: & mut Query<(Entity, &Cell)>,
        mut complexcommand: & mut ResMut<Complexcommand>,
        statecommand: & mut ResMut<NextState<StateCommand>>,
        streamevent: &EventId<StreamEvent>,
        commands: & mut Commands,
        query_transform_res: & mut Query<&mut Transform>,
        query_action_player: & mut Query<& mut ActionPlayer>,
    )
    {
        // ////println!("{:?}")
        ////println!("stacking event_id {:?}", streamevent);
        ////println!("stacking parse {:?}", parse);
        ////println!("Complexe Command {:?}", complexcommand);
        ////println!("nbr_command {:?}", complexcommand.nbr_command);
        ////println!("nb wait  {:?}",complexcommand.vec_id_command);
        ////println!("Last command visit = {}", asset_map.last_event_id_visited);
        // if nb wait == 0 => panic ? 
        if complexcommand.nbr_command == 0 {panic!("something wrong w/ complex command (expulse)")}
        ////println!("vec_command.len() {:?}", complexcommand.vec_command.len());
        for id in complexcommand.vec_id_command.clone() //34, 36
        {
            if streamevent.id == id // ajouter test command attendu (expulse => movement, pose => {Inventaire, Ressource})
            {
                complexcommand.vec_command.push(parse.clone()); // [Inventaire, Ressource]
                // ////println!("")
                asset_map.last_event_id_visited = streamevent.id;
            }
            if complexcommand.nbr_command as usize == complexcommand.vec_command.len()
            {
                // statecommand.set(StateCommand::Simple_Command); // ? multithreade ? 
                dispatch_handle_complex_command(commands, & mut asset_map, & mut complexcommand, statecommand, query_action_player, query_player_cell, query_transform_res);
                // }
            }
            // else {
            //     statecommand.set(StateCommand::Stacking_Command);
                
            // }
        }
    }

    pub fn dispatch_handle_complex_command(
        mut commands: & mut Commands,
        mut asset_map: & mut ResMut<RessCommandId>,
        mut complexcommand: & mut ResMut<Complexcommand>,
        mut statecommand: & mut ResMut<NextState<StateCommand>>,
        mut query_action_player: & mut Query<& mut ActionPlayer>,
        mut query_cell: &Query<(Entity, &Cell)>,
        mut query_transform_res: & mut Query<&mut Transform>,
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
                            
                                // on recupere 
                                println!("player id => {}", id_1);
                                let num_team = asset_map.get_player_num_team(id);
                                println!("-----EXPULSION----- ");
                                let sprite_anim_mvmt = asset_map.get_sprite((*o - 1 ) as usize, num_team as usize);
                                let tmp_cell = get_cell(query_cell, asset_map.get_player_id(&id_1)) ;
                                let sprite_anim_expulse = asset_map.get_sprite_expulsion((tmp_cell.2 - 1 ) as usize, num_team as usize);
                                let expulsion = TypeAction::Expulsion{0: *x, 1: *y, 2: tmp_cell.2, 3: sprite_anim_mvmt, 4: sprite_anim_expulse};
                                add_action(& mut query_action_player, &asset_map.get_player_id(id), expulsion);
                                
                        }
                        _ => panic!("on ne devrai qu'avoir des movement"),
                    }
                }
                
            }
            Parse::Prend(num_player, num_res) => {
                if let Parse::Inventaire(_, _, _, _, _, _, _, _, _, _) = complexcommand.vec_command[0] 
                {
                    if let Parse::RessourceCase(x, y, n, l, d, s,m , ph, th) = complexcommand.vec_command[1]{
                        ////println!("Ressource?");
                        let (x_rel, y_rel) = asset_map.center_map_new_system(x as f32, y as f32);
                        let res = Ressource{ x_rel: x_rel, x: x, y: y, y_rel: y_rel, n: n, l: l, d: d, s: s, m: m, ph: ph, th: th};
                        let res_entity = get_ressource_entity(& mut commands,  &res, num_res, & mut asset_map.id_Ressource);
                        anim_take_ressource_res(&mut query_transform_res, &res_entity, res);
                    }
                }
            }
            
            _ => (panic!("you should not be here"))
        }
        statecommand.set(StateCommand::Simple_Command); // plus besoin 
        complexcommand.fflush();
        //need to set complexcommand back to normal 
        // miss the reset method for complexcommand
    }

}   
