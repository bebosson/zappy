pub mod action
{
    use std::collections::HashMap;

    use crate::player;
    //use crate::gamecontrol::game::GameController;
    use crate::player::player::{Player, Orientation};
    use crate::gamecontrol::game::GameController;
    use crate::cell::cell::{Point, Cell};
    use crate::teams::team::Team;


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

    #[derive(Debug)]
    pub struct ReadyAction
    {
        pub id: u32, // player id
        pub action: Action,
    }

    #[derive(Debug)]
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

        pub fn avance(&self, height: &u8, width: &u8, player: &mut Player) -> bool
        {
            match player.orientation {
                Orientation::N => player.coord.y += 1 % height,
                Orientation::E => player.coord.x += 1 % width,
                Orientation::S => player.coord.y -= 1 % height,
                Orientation::O => player.coord.y -= 1 % width,
            }
            true
        }

        pub fn droite(&self, player: &mut Player) -> bool
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

        pub fn gauche(&self, player: &mut Player) -> bool
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

        pub fn voir(&self, player: &Player, cells: &Vec<Vec<Cell>>, teams: Vec<Team>) -> Vec<HashMap<String, u8>>
        {
            let mut cases_content: Vec<HashMap<String, u8>> = Vec::new();
            let cases_coord = get_cases_coord_from_player_pov(player, cells[0].len(), cells.len());
            for case_coord in cases_coord
            {
                cases_content.push(get_case_content_from_position(case_coord, cells, &teams));
            }
            cases_content
        }

        pub fn inventaire(&self, player: &Player) -> HashMap<String, u8>
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

        pub fn prend(&self, cell: & mut Cell, player: &mut Player, obj: String) -> bool
        {
            println!("player {} want to 'prend' {} on the cell -> {:?}", player.id, obj, cell);
            if (check_obj_is_present_on_cell(obj.to_string(), cell) == false)
            {
                return false;
            }
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
                _ => {()},
            }
            println!("after 'prend', cell content -> {:?}", cell);
            true
        }

        pub fn pose(&self, cell: & mut Cell, player: &mut Player, obj: String) -> bool
        {
            println!("player {} want to 'pose' {} on the cell -> {:?}", player.id, obj, cell);
            if (check_obj_is_present_on_player(obj.to_string(), player) == false)
            {
                return false;
            }

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
            println!("after 'prend', cell content -> {:?}", cell);
            true
        }

        pub fn expulse(&self, teams: & mut Vec<Team>, player: & mut Player, width: &u8, height: &u8) -> bool
        {

            true
        }


    }


///////////////////////////////////////////////////////////////////////////////////////////////////////

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
                if x < 0
                {
                    x = width as i8 + x;
                }
                else if x > width as i8 - 1
                {
                    x = x % width as i8;
                }
                let mut y = match player.orientation
                {
                    Orientation::E => player.coord.y as i8 + (-1) * (i * 2 + 1) / 2 + j,
                    Orientation::O => player.coord.y as i8 +        (i * 2 + 1) / 2 - j,
                    Orientation::N => player.coord.y as i8 -1 * i,
                    Orientation::S => player.coord.y as i8 + i,
                };
                //println!("y -----> {}", y);
                if y < 0
                {
                    y = height as i8 + y;
                }
                else if y > height as i8 - 1
                {
                    y = y % height as i8;
                }
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

}