pub mod player{
    use crate::{ressources::ressources::Ressources, cell::cell::Point, action::action::Action};

    pub enum Orientation{
        N,
        E,
        S,
        O
    }


    pub struct Player{
        pub id: u16,
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
    }
}