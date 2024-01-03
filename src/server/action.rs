pub mod action
{
    //use crate::gamecontrol::game::GameController;
    use crate::player::player::{Player, Orientation};

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

    pub enum ActionResult
    {
        ActionBool(bool),
        ActionString(String),
        ActionInt(u8),
        ActionError
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

        // pub fn voir

        pub fn inventaire(&self, player: &Player) -> String
        {
            let str = format!("food: {}, sibur: {}, mediane: {}, linemate: {}, deraumere: {}, phiras: {}, thystate: {}",
                                    player.ivt.food, player.ivt.sibur, player.ivt.mendiane, player.ivt.linemate, player.ivt.deraumere, player.ivt.phiras, player.ivt.thystate);
            str
        }

        // pub fn prend
        // pub fn pose
        // pub fn expulse


    }

}