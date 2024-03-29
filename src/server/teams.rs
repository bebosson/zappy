pub mod team
{
    use crate::player::player::{Player, Egg};
    use crate::paket_crafter::paquet_crafter::packet_gfx_player_connexion;
    
    #[derive(Debug, Clone)]
    pub struct Team
    {
        pub name: String,
        pub connect_nbr: u8,
        pub port_start_index: u16,
        pub players: Vec<Player>,
        pub eggs: Vec<Egg>,
        // pub nb_players: u8,
    }


    impl Team
    {
        pub fn new(name: &String, connect_nbr: u8) -> Self
        {
            Team
            {
                name: name.clone(),
                connect_nbr: connect_nbr - 1,
                port_start_index: 0,
                players: Vec::new(),
                eggs: Vec::new(),
            }
        }

        pub fn print_players_from_team(&self)
        {
            for player in &self.players
            {
                println!("player #{} :\n\
                coord: {:?}\n\
                orient: {:?}\n\
                level/life: {} / {}\n\
                ivt: {:?}\n\
                actions: {:?}\n",
                player.id, 
                player.coord,
                player.orientation,
                player.level,
                player.life,
                player.ivt,
                player.actions,
            );
            }
        }

        pub fn print_eggs_from_team(&self)
        {
            let mut i = 0;
            for egg in &self.eggs
            {
                println!("egg #{} :\n\
                coord: {:?}\n\
                count: {}\n", 
                i, 
                egg.coord,
                egg.count);
                i += 1;
            }
        }

        pub fn packet_gfx_add_teams(&self) -> Vec<String>
        {
            let mut vec_gfx_packet = vec![];
            for player in &self.players
            {
                vec_gfx_packet.push(format!("{} {}\n", packet_gfx_player_connexion(player), self.name));
            }
            vec_gfx_packet
        }
    }

    
}