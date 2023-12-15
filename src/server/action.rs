pub mod action
{
    //use crate::gamecontrol::game::GameController;
    use std::time::SystemTime;

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
        pub id: u32,
        pub action: Action,
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

    }

}