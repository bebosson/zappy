pub mod teams{
    use std::collections::LinkedList;

    use crate::{egg::egg::Egg, player::player::Player};

    pub struct Teams{
        pub name: String,
        pub port_start_index: u16,
        pub players: LinkedList<Player>,
        pub eggs: LinkedList<Egg>,
        // pub nb_players: u8,
    }
}