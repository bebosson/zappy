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

    fn create_players(size: u8) -> Vec<Player>
    {
        let mut vec_player: Vec<Player> = Vec::with_capacity(size as usize);

        /*for _ in 0..size
        {
            vec_player.push(Player::new());
        }
        */
        vec_player = (0..size)
                        .map(|_| Player::new())
                        .collect();
        vec_player
        
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

        pub fn new_with_size(name: &String, size: u8) -> Self
        {
            Team
            {
                name: name.clone(),
                port_start_index: 0,
                players: create_players(size),
                eggs: vec![],
            }
        }
        // pub fn update_player 
    }

    
}