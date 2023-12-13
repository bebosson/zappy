pub mod action
{
    use std::collections::HashMap;

    use crate::gamecontrol::game::GameController;

    #[derive(Debug, Copy, Clone)]
    enum State
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
    enum ActionType
    {
        NoAction(),
        Avance(&'static str, Option<String>, u16),
        Droite(&'static str, Option<String>, u16),
        Gauche(&'static str, Option<String>, u16),
        Voir(&'static str, Option<String>, u16),
        Inventaire(&'static str, Option<String>, u16),
        Prend(&'static str, Option<String>, u16),
        Pose(&'static str, Option<String>, u16),
        Expulse(&'static str, Option<String>, u16),
        Broadcast(&'static str, Option<String>, u16),
        Incantation(&'static str, Option<String>, u16),
        Fork(&'static str, Option<String>, u16),
        ConnectNbr(&'static str, Option<String>, u16),
    }

    const NO_ACTION: ActionType     = ActionType::NoAction();
    const AVANCE: ActionType        = ActionType::Avance(       "avance",       None,                   7);
    const DROITE: ActionType        = ActionType::Droite(       "droite",       None,                   7);
    const GAUCHE: ActionType        = ActionType::Gauche(       "gauche",       None,                   7);
    const VOIR: ActionType          = ActionType::Voir(         "voir",         None,                   7);
    const INVENTAIRE: ActionType    = ActionType::Inventaire(   "inventaire",   None,                   1);
    const PREND: ActionType         = ActionType::Prend(        "prend",        Some(String::new()),    7);
    const POSE: ActionType          = ActionType::Pose(         "pose",         Some(String::new()),    7);
    const EXPULSE: ActionType       = ActionType::Expulse(      "expulse",      None,                   7);
    const BROADCAST: ActionType     = ActionType::Broadcast(    "broasdcast",   Some(String::new()),    7);
    const INCANTATION: ActionType   = ActionType::Incantation(  "incantation",  None,                   300);
    const FORK: ActionType          = ActionType::Fork(         "fork",         None,                   42);
    const CONNECT_NBR: ActionType   = ActionType::ConnectNbr(   "connect_nbr",  None,                   0);

    #[derive(Debug, Clone)]
    pub struct Action
    {
        state: State,
        count: u16,  
        timestamp: u64,
        action: ActionType,
    }

    impl Action
    {
        pub fn new() -> Self
        {
            Action
            {
                state: State::Idle,
                count: 0,
                timestamp: 0,
                action: NO_ACTION.clone(),
            }
        }

        pub fn action_with_const(action_const : ActionType) -> Self
        {
            // let toto = action_const
            Action
            {
                state: State::Idle,
                count: action_const.2,
                timestamp: 0,
                action: action_const.0,
            }
        }
        
        

        // a refaire
        pub fn avance(game: GameController, id: u32) -> bool
        {
            for team in game.teams
            {

            }

            true
        }
    }


    fn is_valid_obj(object: &str)
    {
        match object
        {
            "food" => println!("it is food"),
            "linemate" => println!("it is linemate"),
            "deraumere" => println!("it is deraumere"),
            "sibur" => println!("it is sibur"),
            "mendiane" => println!("it is mendiane"),
            "phiras" => println!("it is phiras"),
            "thystame" => println!("it is thystame"),
            _ => println!("it is ta mere")
        }
    } 

    fn is_valid_cmd(buf: &str)
    {
        match buf
        {
            "avance" => return AVANCE,
            "droite" => println!("droite ok"),
            "gauche" => println!("gauche ok"),
            "voir" => println!("voir ok"),
            "inventaire" => println!("inventaire ok"),
            txt if txt.starts_with("prend ") => { print!("prend ok => "); is_valid_obj(&buf[6..]);},
            txt if txt.starts_with("pose ") => { print!("pose ok => "); is_valid_obj(&buf[5..]);},
            "expulse" => println!("expulse ok"),
            txt if txt.starts_with("broadcast ") => println!("broadcast ok"),
            "incantation" => println!("incantation ok"),
            "fork" => println!("fork ok"),
            "connect_nbr" => println!("connect_nbr ok"),
            _ => println!("###PAS OK"),
        }
    }
}