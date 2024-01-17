pub mod action
{
    use std::arch::x86_64::_MM_FROUND_TO_NEAREST_INT;
    use std::collections::HashMap;

    use crate::player::player::{Player, Orientation, Egg};
    use crate::cell::cell::{Point, Cell};
    use crate::teams::team::Team;
    use crate::get_obj_from_string;


    #[derive(Debug, Copy, Clone)]
    pub enum State
    {
        Idle,
        Wait,
        Action,
    }

    /***********************************************************************
     * the 3 params of this struct is :
     *      1st --> name of the command (ex: avance)
     *      2nd --> argument of the command (for broadcast, prend & pose)
     *      3rd --> number of cycle to execute the command
    ***********************************************************************/
    #[derive(Debug, Clone)]
    pub struct ActionTemplate
    {
        pub action_name : &'static str,
        pub arg         : Option<String>,
        pub count       : u16,
    }

    pub const NO_ACTION: ActionTemplate     = ActionTemplate{ action_name: "",            arg: None,                  count: 0};
    pub const AVANCE: ActionTemplate        = ActionTemplate{ action_name: "avance",      arg: None,                  count: 7};
    pub const DROITE: ActionTemplate        = ActionTemplate{ action_name: "droite",      arg: None,                  count: 7};
    pub const GAUCHE: ActionTemplate        = ActionTemplate{ action_name: "gauche",      arg: None,                  count: 7};
    pub const VOIR: ActionTemplate          = ActionTemplate{ action_name: "voir",        arg: None,                  count: 7};
    pub const INVENTAIRE: ActionTemplate    = ActionTemplate{ action_name: "inventaire",  arg: None,                  count: 1};
    pub const PREND: ActionTemplate         = ActionTemplate{ action_name: "prend",       arg: Some(String::new()),   count: 7};
    pub const POSE: ActionTemplate          = ActionTemplate{ action_name: "pose",        arg: Some(String::new()),   count: 7};
    pub const EXPULSE: ActionTemplate       = ActionTemplate{ action_name: "expulse",     arg: None,                  count: 7};
    pub const BROADCAST: ActionTemplate     = ActionTemplate{ action_name: "broasdcast",  arg: Some(String::new()),   count: 7};
    pub const INCANTATION: ActionTemplate   = ActionTemplate{ action_name: "incantation", arg: None,                  count: 300};
    pub const FORK: ActionTemplate          = ActionTemplate{ action_name: "fork",        arg: None,                  count: 42};
    pub const CONNECT_NBR: ActionTemplate   = ActionTemplate{ action_name: "connect_nbr", arg: None,                  count: 0};

    pub const COMMANDS: [ActionTemplate; 13] = [AVANCE, DROITE, GAUCHE, VOIR, INVENTAIRE, PREND, POSE, EXPULSE, BROADCAST, INCANTATION, FORK, CONNECT_NBR, NO_ACTION];

    #[derive(Debug, Clone)]
    pub struct ReadyAction
    {
        pub id: u32, // player id
        pub action: Action,
    }

    #[derive(Debug, PartialEq)]
    pub enum ActionResult
    {
        ActionBool(bool),
        ActionString(String),
        ActionInt(u8),
        ActionHashMap(HashMap<String, u8>),
        ActionVecHashMap(Vec<HashMap<String, u8>>),
    }

    #[derive(Debug, Clone)]
    pub struct Action
    {
        pub state: State,
        //pub timestamp: SystemTime,
        pub action_name: String,
        pub count: u16,
        pub arg: Option<String>
    }

    impl Action
    {
        pub fn new(action: ActionTemplate) -> Self
        {
            Action
            {
                state: State::Wait,
                //timestamp: SystemTime::now(),
                action_name: action.action_name.to_string(),
                count: action.count,
                arg: action.arg,
            }
        }

        pub fn new_from_string(command_full: String) -> Self
        {
            let mut action: Action = Action::new(NO_ACTION);
            let object: Option<String> = get_obj_from_string(&command_full);

            for tmp_command in COMMANDS
            {
                if command_full.starts_with(tmp_command.action_name)
                {
                    action = Action::new(ActionTemplate{action_name: tmp_command.action_name, arg: object, count: tmp_command.count});
                    break ;
                }
            }
            action
        }


    }


    pub fn avance(height: u8, width: u8, player: &mut Player) -> bool
    {
        match player.orientation
        {
            Orientation::N =>
            {
                if player.coord.y == 0 { player.coord.y = height - 1; }
                else { player.coord.y -= 1; }
            },
            Orientation::E => 
            {
                if player.coord.x == width - 1 { player.coord.x = 0; }
                else { player.coord.x += 1; }
            },
            Orientation::S => {
                if player.coord.y == height - 1 { player.coord.y = 0; }
                else { player.coord.y += 1; }
            },
            Orientation::O =>
            {
                if player.coord.x == 0 { player.coord.x = width - 1; }
                else { player.coord.x -= 1; }
            }
        }
        true
    }

    pub fn droite(player: &mut Player) -> bool
    {
        match player.orientation
        {
            Orientation::N => player.orientation = Orientation::E,
            Orientation::E => player.orientation = Orientation::S,
            Orientation::S => player.orientation = Orientation::O,
            Orientation::O => player.orientation = Orientation::N,
        }
        true
    }

    pub fn gauche(player: &mut Player) -> bool
    {
        match player.orientation
        {
            Orientation::N => player.orientation = Orientation::O,
            Orientation::E => player.orientation = Orientation::N,
            Orientation::S => player.orientation = Orientation::E,
            Orientation::O => player.orientation = Orientation::S,
        }
        true
    }

    pub fn voir(player: &Player, cells: &Vec<Vec<Cell>>, teams: &Vec<Team>) -> Vec<HashMap<String, u8>>
    {
        let mut cases_content: Vec<HashMap<String, u8>> = Vec::new();
        let cases_coord = get_cases_coord_from_player_pov(player, cells[0].len(), cells.len());
        for case_coord in cases_coord
        {
            let mut x = HashMap::new();
            let mut y = HashMap::new();
            x.insert("x".to_string(), case_coord.x);
            y.insert("y".to_string(), case_coord.y);
            cases_content.push(x);
            cases_content.push(y);
            cases_content.push(get_case_content_from_position(case_coord, cells, &teams));
        }
        cases_content
    }

    pub fn inventaire(player: &Player) -> HashMap<String, u8>
    {
        let mut hashmap = HashMap::new();
        
        hashmap.insert("food".to_string(), player.ivt.food);
        hashmap.insert("sibur".to_string(), player.ivt.sibur);
        hashmap.insert("mendiane".to_string(), player.ivt.mendiane);
        hashmap.insert("linemate".to_string(), player.ivt.linemate);
        hashmap.insert("deraumere".to_string(), player.ivt.deraumere);
        hashmap.insert("phiras".to_string(), player.ivt.phiras);
        hashmap.insert("thystate".to_string(), player.ivt.thystate);
        hashmap
    }

    pub fn prend(cell: & mut Cell, player: &mut Player, obj: &String) -> bool
    {
        //println!("player {} want to 'prend' {} on the cell -> {:?}", player.id, obj, cell);
        if check_obj_is_present_on_cell(obj.to_string(), cell) == false { return false; }

        match obj.as_str()
        {
            "food" => {
                if player.life > 1260 - 126 { player.ivt.food += 1; }
                else { player.life += 126; }
                cell.ressources.food -= 1;
            },
            "sibur" => {player.ivt.sibur += 1; cell.ressources.sibur -= 1;},
            "mendiane" => {player.ivt.mendiane += 1; cell.ressources.mendiane -= 1;},
            "linemate" => {player.ivt.linemate += 1; cell.ressources.linemate -= 1;},
            "deraumere" => {player.ivt.deraumere += 1; cell.ressources.deraumere -= 1;},
            "phiras" => {player.ivt.phiras += 1; cell.ressources.phiras -= 1;},
            "thystate" => {player.ivt.thystate += 1; cell.ressources.thystate -= 1;},
            _ => {()},
        }
        //println!("after 'prend', cell content -> {:?}", cell);
        true
    }

    pub fn pose(cell: & mut Cell, player: &mut Player, obj: &String) -> bool
    {
        //println!("player {} want to 'pose' {} on the cell -> {:?}", player.id, obj, cell);
        if check_obj_is_present_on_player(obj.to_string(), player) == false { return false; }

        match obj.as_str()
        {
            "food" => {player.ivt.food -= 1; cell.ressources.food += 1;},
            "sibur" => {player.ivt.sibur -= 1; cell.ressources.sibur += 1;},
            "mendiane" => {player.ivt.mendiane -= 1; cell.ressources.mendiane += 1;},
            "linemate" => {player.ivt.linemate -= 1; cell.ressources.linemate += 1;},
            "deraumere" => {player.ivt.deraumere -= 1; cell.ressources.deraumere += 1;},
            "phiras" => {player.ivt.phiras -= 1; cell.ressources.phiras += 1;},
            "thystate" => {player.ivt.thystate -= 1; cell.ressources.thystate += 1;},
            _ => {()},
        }
        //println!("after 'prend', cell content -> {:?}", cell);
        true
    }

    pub fn expulse(teams: & mut Vec<Team>, player: & Player, width: u8, height: u8) -> bool
    {
        let mut nb_kick_player = 0;
        let target_cell = find_target_cell_from_coord(&player.orientation, &player.coord, width as usize, height as usize);

        println!("target cell for player {} --> {:?}", player.id, target_cell);
        for team in teams
        {
            for tmp_player in &mut team.players
            {
                if player.coord.x == tmp_player.coord.x
                {
                    tmp_player.coord.x = target_cell.x;
                    tmp_player.coord.y = target_cell.y;
                    nb_kick_player += 1;
                }
            }
        }
        if nb_kick_player == 0
        {
            return false;
        }
        true
    }

    pub fn fork(player: &Player, teams: &mut Vec<Team>) -> bool
    {
        for i in 0..teams.len()
        {
            for tmp_player in teams[i].players.clone()
            {
                if tmp_player.id == player.id
                {
                    //let mut total_players = tmp_team.iter().map(|team| team.players.len() as u16).sum::<u16>();
                    //total_players += tmp_team.iter().map(|team| team.eggs.len() as u16).sum::<u16>();
                    teams[i].nb_total_players += 1;
                    println!("team {} -> nb total players: {}", teams[i].name, teams[i].nb_total_players);
                    let tmp = teams.clone();
                    teams[i].eggs.push(Egg { id: get_nb_total_players(&tmp), count: 600, coord: player.coord.clone() });
                }
            }
        }
        true
    }

    pub fn connect_nbr(player: &Player, teams: &Vec<Team>) -> u8
    {
        for team in teams
        {
            for tmp_player in &team.players
            {
                if tmp_player.id == player.id
                {
                    return team.connect_nbr;
                }
            }
        }
        0
    }

    pub fn incantation(player: &Player, teams: &Vec<Team>) -> String
    {
        let mut nb_players = 0;
        let mut is_enough_players_on_coord = false;
        let mut is_enough_ressources_for_player = true;
        let level_requirement = get_level_requirement();

        let elems = vec!
        [
            (level_requirement[player.level as usize].get(&"linemate".to_string()), player.ivt.linemate),
            (level_requirement[player.level as usize].get(&"deraumere".to_string()), player.ivt.deraumere),
            (level_requirement[player.level as usize].get(&"sibur".to_string()), player.ivt.sibur),
            (level_requirement[player.level as usize].get(&"mendiane".to_string()), player.ivt.mendiane),
            (level_requirement[player.level as usize].get(&"phiras".to_string()), player.ivt.phiras),
            (level_requirement[player.level as usize].get(&"thystate".to_string()), player.ivt.thystate),
        ];

        for team in teams
        {
            for tmp_player in &team.players
            {
                if tmp_player.coord.x == player.coord.x && tmp_player.coord.y == player.coord.y
                {
                    nb_players += 1;
                }
            }
        }
        if nb_players == *level_requirement[player.level as usize].get(&"nb_players".to_string()).unwrap()
        {
            is_enough_players_on_coord = true;
        }
        for elem in elems
        {
            if elem.0 != Some(&elem.1)
            {
                is_enough_ressources_for_player = false;
            }
        }
        if is_enough_players_on_coord == true && is_enough_ressources_for_player == true
        {
            return "Elevation en cours".to_string();
        }
        // qu'est ce qu'on envoie en cas d'erreur ???
        "".to_string()
    }

    pub fn broadcast(player: &Player, teams: &Vec<Team>) -> bool
    {
        for team in teams
        {
            for tmp_player in &team.players
            {
                let x1 = tmp_player.coord.x;
                let y1 = tmp_player.coord.y;
                let x2 = player.coord.x;
                let y2 = player.coord.y;
                let coeff: f32 = (y2 - y1) as f32 / (x2 - x1) as f32;
                
            }
        }
        true
    }


///////////////////////////////////////////////////////////////////////////////////////////////////////


    pub fn get_nb_total_players(teams: &Vec<Team>) -> u16
    {
        let mut ret = 0;

        for team in teams
        {
            ret += team.nb_total_players;
        }
        ret
    }

    fn find_target_cell_from_coord(orientation: &Orientation, coord: &Point, width: usize, height: usize) -> Point
    {
        let mut x = coord.x as i8;
        let mut y = coord.y as i8;
        match orientation
        {
            Orientation::E => {
                if x as usize == width - 1 { x = -1;}
                Point{x: x as u8 + 1, y: y as u8}
            },
            Orientation::N => {
                if y as u8 == 0 { y = height as i8;}
                Point{x: x as u8, y: y as u8 - 1}
            },
            Orientation::O => {
                if x == 0 as i8 { x = width as i8;}
                Point{x: x as u8 - 1, y: y as u8}
            },
            Orientation::S => {
                if y as usize == height - 1 { y = - 1;}
                Point{x: x as u8, y: y as u8 + 1}
            },
            _ => Point{x: 0, y: 0},
        }
        
    }

    fn check_obj_is_present_on_cell(obj: String, cell: &Cell) -> bool
    {
        match obj.as_str()
        {
            "food" => {if cell.ressources.food > 0 { return true; } false},
            "sibur" => {if cell.ressources.sibur > 0 { return true; } false},
            "mendiane" => {if cell.ressources.mendiane > 0 { return true; } false},
            "linemate" => {if cell.ressources.linemate > 0 { return true; } false},
            "deraumere" => {if cell.ressources.deraumere > 0 { return true; } false},
            "phiras" => {if cell.ressources.phiras > 0 { return true; } false},
            "thystate" => {if cell.ressources.thystate > 0 { return true; } false},
            _ => false,
        }
    }

    fn check_obj_is_present_on_player(obj: String, player: &Player) -> bool
    {
        match obj.as_str()
        {
            "food" => {if player.ivt.food > 0 { return true; } false},
            "sibur" => {if player.ivt.sibur > 0 { return true; } false},
            "mendiane" => {if player.ivt.mendiane > 0 { return true; } false},
            "linemate" => {if player.ivt.linemate > 0 { return true; } false},
            "deraumere" => {if player.ivt.deraumere > 0 { return true; } false},
            "phiras" => {if player.ivt.phiras > 0 { return true; } false},
            "thystate" => {if player.ivt.thystate > 0 { return true; } false},
            _ => false,
        }
    }

    fn get_cases_coord_from_player_pov(player: &Player, width: usize, height: usize) -> Vec<Point>
    {
        let mut cases_coord : Vec<Point> = Vec::new();

        cases_coord.push(Point{x: player.coord.x, y: player.coord.y});
        for i in 1..=player.level as i8
        {
            for j in 0..(i * 2) + 1 as i8
            {
                let mut x = match player.orientation
                {
                    Orientation::N => player.coord.x as i8 + (-1) * (i * 2 + 1) / 2 + j,
                    Orientation::S => player.coord.x as i8 +        (i * 2 + 1) / 2 - j,
                    Orientation::O => player.coord.x as i8 -1 * i,
                    Orientation::E => player.coord.x as i8 + i,
                };
                //println!("x -----> {}", x);
                if x < 0 { x = width as i8 + x; }
                else if x > width as i8 - 1 { x = x % width as i8; }
                let mut y = match player.orientation
                {
                    Orientation::E => player.coord.y as i8 + (-1) * (i * 2 + 1) / 2 + j,
                    Orientation::O => player.coord.y as i8 +        (i * 2 + 1) / 2 - j,
                    Orientation::N => player.coord.y as i8 -1 * i,
                    Orientation::S => player.coord.y as i8 + i,
                };
                //println!("y -----> {}", y);
                if y < 0 { y = height as i8 + y; }
                else if y > height as i8 - 1 { y = y % height as i8; }
                cases_coord.push(Point{x: x as u8, y: y as u8});
                println!("\n");
            }
        }
        //println!("coord des cases a voir ---> {:?}", cases_coord);
        cases_coord
    }

    fn get_case_content_from_position(coord: Point, cells: &Vec<Vec<Cell>>, teams: &Vec<Team>) -> HashMap<String, u8>
    {
        let mut cell_content: HashMap<String, u8> = HashMap::new();

        //println!("cell ({},{}) -> {:?} {}", coord.x, coord.y, cells.len(), cells[0].len());
        println!("cell ({},{})", coord.x, coord.y);
        cell_content.insert("food".to_string(), cells[coord.y as usize][coord.x as usize].ressources.food);
        cell_content.insert("sibur".to_string(), cells[coord.y as usize][coord.x as usize].ressources.sibur);
        cell_content.insert("mendiane".to_string(), cells[coord.y as usize][coord.x as usize].ressources.mendiane);
        cell_content.insert("linemate".to_string(), cells[coord.y as usize][coord.x as usize].ressources.linemate);
        cell_content.insert("deraumere".to_string(), cells[coord.y as usize][coord.x as usize].ressources.deraumere);
        cell_content.insert("phiras".to_string(), cells[coord.y as usize][coord.x as usize].ressources.phiras);
        cell_content.insert("thystate".to_string(), cells[coord.y as usize][coord.x as usize].ressources.thystate);
        
        // maybe add players 
        let mut nb_players = 0;
        for team in teams
        {
            for player in &team.players
            {
                if player.coord.x == coord.x && player.coord.y == coord.y
                {
                    nb_players += 1;
                }
            }
        }
        cell_content.insert("player".to_string(), nb_players);

        
        //println!("cell ({},{}) --> {:?}", coord.x, coord.y, cell_content);
        
        cell_content
    }

    fn get_level_requirement() -> Vec<HashMap<String, u8>>
    {
        let mut hashmap: Vec<HashMap<String, u8>> = vec![
        { // level 1-2
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 1 as u8);
            map.insert("linemate".to_string(), 1 as u8);
            map.insert("deraumere".to_string(), 0 as u8);
            map.insert("sibur".to_string(), 0 as u8);
            map.insert("mendiane".to_string(), 0 as u8);
            map.insert("phiras".to_string(), 0 as u8);
            map.insert("thystate".to_string(), 0 as u8);
            map
        },
        { // level 2-3
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 2 as u8);
            map.insert("linemate".to_string(), 1 as u8);
            map.insert("deraumere".to_string(), 1 as u8);
            map.insert("sibur".to_string(), 1 as u8);
            map.insert("mendiane".to_string(), 0 as u8);
            map.insert("phiras".to_string(), 0 as u8);
            map.insert("thystate".to_string(), 0 as u8);
            map
        },
        { // level 3-4
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 2 as u8);
            map.insert("linemate".to_string(), 2 as u8);
            map.insert("deraumere".to_string(), 0 as u8);
            map.insert("sibur".to_string(), 1 as u8);
            map.insert("mendiane".to_string(), 0 as u8);
            map.insert("phiras".to_string(), 2 as u8);
            map.insert("thystate".to_string(), 0 as u8);
            map
        },
        { // level 4-5
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 4 as u8);
            map.insert("linemate".to_string(), 1 as u8);
            map.insert("deraumere".to_string(), 1 as u8);
            map.insert("sibur".to_string(), 2 as u8);
            map.insert("mendiane".to_string(), 0 as u8);
            map.insert("phiras".to_string(), 1 as u8);
            map.insert("thystate".to_string(), 0 as u8);
            map
        },
        { // level 5-6
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 4 as u8);
            map.insert("linemate".to_string(), 1 as u8);
            map.insert("deraumere".to_string(), 2 as u8);
            map.insert("sibur".to_string(), 1 as u8);
            map.insert("mendiane".to_string(), 3 as u8);
            map.insert("phiras".to_string(), 0 as u8);
            map.insert("thystate".to_string(), 0 as u8);
            map
        },
        { // level 6-7
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 6 as u8);
            map.insert("linemate".to_string(), 1 as u8);
            map.insert("deraumere".to_string(), 2 as u8);
            map.insert("sibur".to_string(), 3 as u8);
            map.insert("mendiane".to_string(), 0 as u8);
            map.insert("phiras".to_string(), 1 as u8);
            map.insert("thystate".to_string(), 0 as u8);
            map
        },
        { // level 7-8
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 6 as u8);
            map.insert("linemate".to_string(), 2 as u8);
            map.insert("deraumere".to_string(), 2 as u8);
            map.insert("sibur".to_string(), 2 as u8);
            map.insert("mendiane".to_string(), 2 as u8);
            map.insert("phiras".to_string(), 2 as u8);
            map.insert("thystate".to_string(), 1 as u8);
            map
        }];
        hashmap
    }

}