pub mod player
{
    use std::net::TcpStream;
    use std::ops::Deref;

    use crate::ressources::ressources::Ressources;
    use crate::cell::cell::Point;
    use crate::action::action::{Action, ActionTemplate};
    use crate::get_obj_from_string;
    use crate::action::action::*;

    use rand::{thread_rng, Rng};


    #[derive(Debug, Clone, Copy)]
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

    impl Egg
    {
        pub fn update(&mut self)
        {
            self.count = self.count - 1;
        }
    }

    pub struct SimplePlayer
    {
        pub id: u32,
        pub coord: Point,
        pub ivt: Ressources,
        pub life: u16,
        pub orientation: Orientation,
        pub level: u8,
    }

    #[derive(Debug)]
    pub struct Player
    {
        pub stream: TcpStream, // pas sur
        pub id: u32,
        // pub port: u16,
        pub coord: Point,
        pub ivt: Ressources,
        pub life: u16,
        pub orientation: Orientation,
        pub level: u8,
        pub actions: Vec<Action>,
    }
    
    // impl Clone for Player
    // {
        
        // }
        
        impl Player
        {
            pub fn new(stream: TcpStream, id: u32, port: u16, width: u8, height: u8) -> Self
        {
            let mut rng = thread_rng();

            Player
            {
                stream,
                id,
                // port: port,
                //coord: Point::new(rng.gen_range(0..width - 1), rng.gen_range(0..height - 1)),
                coord: Point::new(0, 0), // to remove for random
                ivt: Ressources::new(),
                life: 1260,
                //orientation: get_random_orientation(),
                orientation: Orientation::S, // to remove for random
                level: 1,
                actions: Vec::new(),
            }
        }
        
        // pub fn new_from_egg(id: u32, coord: Point) -> Self
        // {
        //     Player
        //     {
        //         stream: Box::new(),
        //         id,
        //         // port: 42,
        //         coord: Point::new(coord.x, coord.y),
        //         ivt: Ressources::new(),
        //         life: 1260,
        //         orientation: get_random_orientation(),
        //         level: 1,
        //         actions: Vec::new(),
        //     }
        // }
        
        fn clone(&self, stream: TcpStream) -> Self {
            Player {
                stream,
                id: self.id,
                coord: self.coord,
                ivt: self.ivt,
                life: self.life,
                orientation: self.orientation,
                level: self.level,
                actions: self.actions.clone(),// a modifier
            }
        }
        
        pub fn update(& mut self)
        {
            self.life -= 1;
            if self.actions.len() > 0
            {
                self.actions[0].count = self.actions[0].count - 1;
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

    }

    pub fn get_random_orientation() -> Orientation
    {
        let mut rng = thread_rng();

        match rng.gen_range(0..4)
        {
            0 => Orientation::N,
            1 => Orientation::E,
            2 => Orientation::S,
            3 => Orientation::O,
            _ => Orientation::N,
        }
    }
    
}