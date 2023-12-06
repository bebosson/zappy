pub mod player{
    use crate::{ressources::ressources::Ressources, cell::cell::Point};

    enum State{
        Idle,
        Wait,
        Action,
    }

    pub enum Orientation{
        N,
        E,
        S,
        O
    }

    pub struct Action{
        state: State,
        count: u16,    
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
}