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
                    self.players.push(Player::new_from_egg(e.id as u32, e.coord.clone()));
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
                coord     : ({}, {}) -> {:?}\n\
                level/life: {} / {}\n\
                ivt       : food {}, linemate {}, deraumere {}, sibur {}, phiras {}, mendiane {}, thystate {}",
                player.id, 
                player.coord.x,
                player.coord.y,
                player.orientation,
                player.level,
                player.life,
                player.ivt.food,
                player.ivt.linemate,
                player.ivt.deraumere,
                player.ivt.sibur,
                player.ivt.phiras,
                player.ivt.mendiane,
                player.ivt.thystate);
                println!("actions   :");
                for action in &player.actions
                {
                    println!("            (name: {}, count: {}, arg: {:?})",
                        action.action_name,
                        action.count,
                        action.arg);
                }
            }
        }

        pub fn print_eggs_from_team(&self)
        {
            for egg in &self.eggs
            {
                println!("egg #{}    :\n\
                coord     : ({}, {}) --- count: {}\n", 
                egg.id, 
                egg.coord.x,
                egg.coord.y,
                egg.count);
            }
        }

    }

    
}