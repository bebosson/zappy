pub mod player{
    use crate::{ressources::ressources::Ressources, cell::cell::Point, action::action::Action};

    #[derive(Debug)]
    pub enum Orientation{
        N,
        E,
        S,
        O
    }

    #[derive(Debug)]
    pub struct Player{
        pub id: u128,
        pub port: u16,
        pub coor: Point,
        pub ivt: Ressources,
        pub life: u16,
        pub orient: Orientation,
        pub level: u8,
        pub state: Action,
    }

    
    impl Player{
        pub fn new() -> Self
        {
            Player{
                id: 0,
                port: 0,
                coor: Point::new(0,0),
                ivt: Ressources::new(),
                life: 1260,
                orient: Orientation::N,
                level: 1,
                state: Action::new(),
            }
        }

        pub fn new_with_id(id_a: u128) -> Self
        {
            Player{
                id: id_a,
                port: 0,
                coor: Point::new(0,0),
                ivt: Ressources::new(),
                life: 1260,
                orient: Orientation::N,
                level: 1,
                state: Action::new(),
            }
        }   
    }
    
}