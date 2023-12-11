pub mod team{
    use std::collections::LinkedList;

    use crate::{egg::egg::Egg, player::player::Player, args::args::Args};

    pub struct Team{
        pub name: String,
        pub port_start_index: u16,
        pub players: Vec<Player>,
        pub eggs: Vec<Egg>,
        // pub nb_players: u8,
    }
    

    impl Team{
        fn new(name: &String) -> Self
        {
            Team{
                name: name.clone(),
                port_start_index: 0,
                players: vec![Player::new()],
                eggs: vec![],
            }
        } 
    }
}