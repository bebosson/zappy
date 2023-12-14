pub mod team
{
    use crate::player::player::{Player, Egg};
    
    #[derive(Debug, Clone)]
    pub struct Team
    {
        pub name: String,
        pub port_start_index: u16,
        pub players: Vec<Player>,
        pub eggs: Vec<Egg>,
        // pub nb_players: u8,
    }


    impl Team
    {
        pub fn new(name: &String) -> Self
        {
            Team
            {
                name: name.clone(),
                port_start_index: 0,
                players: Vec::new(),
                eggs: Vec::new(),
            }
        }

        // pub fn update_player 
    }

    
}