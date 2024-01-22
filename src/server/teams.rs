pub mod team
{
    use crate::player::player::{Player, Egg};
    use crate::paket_crafter::paquet_crafter::packet_gfx_player_connexion;
    
    #[derive(Debug)]
    pub struct Team
    {
        pub name: String,
        pub connect_nbr: u8,
        pub port_start_index: u16,
        pub players: Vec<Player>,
        pub eggs: Vec<Egg>,
        pub nb_total_players: u16, // count dead and alive players and eggs
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
                nb_total_players: 0,
            }
        }

        pub fn update(& mut self)
        {
            self.players.iter_mut().for_each(|p| p.update());
            // remove dead players
            self.players.retain(|p| p.life != 0);
            self.eggs.iter_mut().for_each(|e|
            {
                e.update();
                if e.count == 0
                {
                    // add new player
                    // self.players.push(Player::new_from_egg(e.id as u32, e.coord.clone()));
                }
            });
            // remove dead eggs
            self.eggs.retain(|egg| egg.count != 0);
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
            for egg in &self.eggs
            {
                println!("egg #{} :\n\
                coord: {:?}\n\
                count: {}\n", 
                egg.id, 
                egg.coord,
                egg.count);
            }
        }

    }

    
}