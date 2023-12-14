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
    pub struct Action_template
    {
        action_name : &'static str,
        arg         : Option<String>,
        count       : u16,
    }

    pub const NO_ACTION: Action_template     = Action_template{ action_name: "",            arg: None,                  count: 0};
    const AVANCE: Action_template        = Action_template{ action_name: "avance",      arg: None,                  count: 7};
    const DROITE: Action_template        = Action_template{ action_name: "droite",      arg: None,                  count: 7};
    const GAUCHE: Action_template        = Action_template{ action_name: "gauche",      arg: None,                  count: 7};
    const VOIR: Action_template          = Action_template{ action_name: "voir",        arg: None,                  count: 7};
    const INVENTAIRE: Action_template    = Action_template{ action_name: "inventaire",  arg: None,                  count: 1};
    const PREND: Action_template         = Action_template{ action_name: "prend",       arg: Some(String::new()),   count: 7};
    const POSE: Action_template          = Action_template{ action_name: "pose",        arg: Some(String::new()),   count: 7};
    const EXPULSE: Action_template       = Action_template{ action_name: "expulse",     arg: None,                  count: 7};
    const BROADCAST: Action_template     = Action_template{ action_name: "broasdcast",  arg: Some(String::new()),   count: 7};
    const INCANTATION: Action_template   = Action_template{ action_name: "incantation", arg: None,                  count: 300};
    const FORK: Action_template          = Action_template{ action_name: "fork",        arg: None,                  count: 42};
    const CONNECT_NBR: Action_template   = Action_template{ action_name: "connect_nbr", arg: None,                  count: 0};

    #[derive(Debug, Clone)]
    pub struct Action
    {
        state: State,
        timestamp: u64,
        action_name: String,
        count: u16,
        arg: Option<String>
    }

    impl Action
    {
        pub fn new(action: Action_template) -> Self
        {
            Action
            {
                state: State::Wait,
                timestamp: 0, // get time now
                action_name: action.action_name.to_string(),
                count: action.count,
                arg: action.arg,
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

}