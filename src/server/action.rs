pub mod action
{
    use crate::gamecontrol::game::GameController;

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
        action_name : &'static str,
        arg         : Option<String>,
        count       : u16,
    }

    const NO_ACTION: ActionTemplate     = ActionTemplate{ action_name: "",            arg: None,                  count: 0};
    const AVANCE: ActionTemplate        = ActionTemplate{ action_name: "avance",      arg: None,                  count: 7};
    const DROITE: ActionTemplate        = ActionTemplate{ action_name: "droite",      arg: None,                  count: 7};
    const GAUCHE: ActionTemplate        = ActionTemplate{ action_name: "gauche",      arg: None,                  count: 7};
    const VOIR: ActionTemplate          = ActionTemplate{ action_name: "voir",        arg: None,                  count: 7};
    const INVENTAIRE: ActionTemplate    = ActionTemplate{ action_name: "inventaire",  arg: None,                  count: 1};
    const PREND: ActionTemplate         = ActionTemplate{ action_name: "prend",       arg: Some(String::new()),   count: 7};
    const POSE: ActionTemplate          = ActionTemplate{ action_name: "pose",        arg: Some(String::new()),   count: 7};
    const EXPULSE: ActionTemplate       = ActionTemplate{ action_name: "expulse",     arg: None,                  count: 7};
    const BROADCAST: ActionTemplate     = ActionTemplate{ action_name: "broasdcast",  arg: Some(String::new()),   count: 7};
    const INCANTATION: ActionTemplate   = ActionTemplate{ action_name: "incantation", arg: None,                  count: 300};
    const FORK: ActionTemplate          = ActionTemplate{ action_name: "fork",        arg: None,                  count: 42};
    const CONNECT_NBR: ActionTemplate   = ActionTemplate{ action_name: "connect_nbr", arg: None,                  count: 0};

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
        pub timestamp: u64,
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
                timestamp: 0, // get time now
                action_name: action.action_name.to_string(),
                count: action.count,
                arg: action.arg,
            }
        }

        // a refaire
        pub fn avance(_game: GameController, _id: u32) -> bool
        {
            /*
            for team in game.teams
            {

            }
            */

            true
        }
    }

}