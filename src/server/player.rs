pub mod player
{
    use crate::ressources::ressources::Ressources;
    use crate::cell::cell::Point;
    use crate::action::action::{Action, ActionTemplate};
    use crate::get_obj_from_string;
    use crate::action::action::*;

    use rand::{thread_rng, Rng};


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
        pub id: u16,
        pub count: u16,
        pub coord: Point,
    }

    #[derive(Debug, Clone)]
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
        pub fn new(id_a: u32, port: u16, width: u8, height: u8) -> Self
        {
            let mut rng = thread_rng();

            Player
            {
                id: id_a,
                port: port,
                //coord: Point::new(rng.gen_range(0..width - 1), rng.gen_range(0..height - 1)),
                coord: Point::new(0, 0),
                ivt: Ressources::new(),
                life: 1260,
                //orientation: match rng.gen_range(0..4)
                //{
                //    0 => Orientation::N,
                //    1 => Orientation::E,
                //    2 => Orientation::S,
                //    3 => Orientation::O,
                //    _ => Orientation::N,
                //},
                orientation: Orientation::S,
                level: 1,
                actions: Vec::new(),
            }
        }

        pub fn action_push(& mut self, command_full: String)
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
            // println!("Action {:?}", action);
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

        pub fn packet_gfx_player_connexion(&self) -> String
        {
            format!("pnw {} {} {} {} {}", self.id, self.coord.x, self.coord.y, self.format_orientation(), self.level)
        }

        pub fn packet_gfx_player_position(&self) -> String
        {
            //"ppo #n X Y O\n"
            format!("ppo {} {} {} {}\n", self.id, self.coord.x, self.coord.y, self.format_orientation())
        }

        pub fn format_orientation(&self) -> String
        {
            match self.orientation
            {
                Orientation::N => format!("{}", 1),
                Orientation::E => format!("{}", 2),
                Orientation::S => format!("{}", 3),
                Orientation::O => format!("{}", 4),
            }
        }

        pub fn avance(& mut self, x_max: &u8, y_max: &u8) -> String
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
            self.packet_gfx_player_position()
        }

    }
    
}