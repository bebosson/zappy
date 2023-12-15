pub mod player
{
    use crate::ressources::ressources::Ressources;
    use crate::cell::cell::Point;
    use crate::action::action::{Action, ActionTemplate};
    use crate::{COMMAND_SLICE, get_obj_from_string};
    use crate::action::action::*;


    #[derive(Debug, Clone)]
    pub enum Orientation
    {
        N,
        E,
        S,
        O
    }

    #[derive(Debug, Clone)]
    pub struct Egg
    {
        pub count: u16,
        pub coord: Point,
    }

    #[derive(Debug)]
    pub struct Player
    {
        pub id: u32,
        //pub stream: TcpStream, // pas sur
        pub port: u16,
        pub coord: Point,
        pub ivt: Ressources,
        pub life: u16,
        pub orientation: Orientation,
        pub level: u8,
        pub actions: Vec<Action>,
    }

    
    impl Player
    {
        pub fn new(id_a: u32, port: u16) -> Self
        {

            Player
            {
                id: id_a,
                port: port,
                coord: Point::new(0, 0),
                ivt: Ressources::new(),
                life: 1260,
                orientation: Orientation::N,
                level: 1,
                actions: Vec::new(),
            }
        }

        pub fn action_push(& mut self, command_full: String)
        {
            let mut action: Action = Action::new(NO_ACTION);
            let object: Option<String> = get_obj_from_string(&command_full);

            for COMMAND in COMMANDS
            {
                if command_full.starts_with(COMMAND.action_name)
                {
                    action = Action::new(ActionTemplate{action_name: COMMAND.action_name, arg: object, count: COMMAND.count});
                    break ;
                }
            }
            self.actions.push(action.clone());
        }

        pub fn print_player_actions(&self)
        {
            println!("--------print_player_actions-----------\nplayer {} actions:", self.id);
            for action in &self.actions
            {
                println!("action : {} , arg : {:?} count : {}", action.action_name, action.arg, action.count);
            }

        }

        pub fn avance(& mut self, x_max: &u8, y_max: &u8) -> bool
        {
            match self.orientation
            {
                Orientation::N =>
                {
                    if self.coord.y == 0    { self.coord.y = *y_max; }
                    else                    { self.coord.y = self.coord.y - 1; }
                },
                Orientation::E =>
                {
                    if self.coord.x == 0    { self.coord.x = *x_max; }
                    else                    { self.coord.x = self.coord.x + 1; }
                },
                Orientation::S =>
                {
                    if self.coord.y == *y_max    { self.coord.y = 0; }
                    else                        { self.coord.y = self.coord.y + 1; }
                },
                Orientation::O =>
                {
                    if self.coord.x == *x_max    { self.coord.x = 0; }
                    else                        { self.coord.x = self.coord.x - 1; }
                },
            }
            true
        }

    }
    
}