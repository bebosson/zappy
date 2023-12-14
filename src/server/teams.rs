pub mod team
{
    use crate::egg::egg::Egg;
    use crate::player::player::Player;
    use crate::args::args::Args;
    
    #[derive(Debug)]
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
                players: vec![],
                eggs: vec![],
            }
        }

        // pub fn update_player 
    }

    
}