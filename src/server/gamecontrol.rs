
pub mod game
{
    use std::net::TcpStream;
    use std::time::SystemTime;
    use rand::{thread_rng, Rng};

    use crate::ressources::ressources::Ressources;
    use crate::teams::team::Team;
    use crate::args::args::Args;
    use crate::player::player::{Player, Orientation};
    use crate::cell::cell::Cell;
    use crate::init::init::init_map_cells;

/**********************************************************************
 * Struct GameController, this is the main structure of the program
***********************************************************************/
    #[derive(Debug, Clone)]
    pub struct GameController
    {
        pub x: u8,
        pub y: u8,
        pub cells: Vec<Vec<Cell>>,
        pub teams: Vec<Team>,
        pub timestamp: u32,
    }

    impl GameController
    {
        pub fn new(args: &Args) -> Self
        {
            let mut vec_teams: Vec<Team> = vec![];

            args.n
                .iter()
                .map(|x| vec_teams.push(Team::new(&x.clone(), args.c)))
                .for_each(drop);

            GameController
            {
                x: args.x,
                y: args.y,
                cells : init_map_cells(args.x, args.y),
                teams: vec_teams,
                timestamp: 0,
            }
        }

        pub fn print_all_players(&self)
        {
            for team in & self.teams
            {
                println!("- - - - - - - team {} - - - - - - - - -", team.name);
                team.print_players_from_team();
                println!("-------------------------------");
                team.print_eggs_from_team();
                println!(" - - - - - - - - - - - - - - - - - - - - - - - -");
            }
            println!("\n\n");
        }

        pub fn get_team_and_push(& mut self, teamname: &String, id: u32, stream: &TcpStream, width: u8, height: u8)
        {
            let port = stream
                                .peer_addr()
                                .unwrap()
                                .port();

            for team in & mut self.teams
            {
                if team.name.eq(teamname) == true
                {
                    team.players.push(Player::new(id, port, width, height));
                }
            }
        }

        pub fn update_timestamp(& mut self, start_time: &SystemTime, t: u16) -> bool
        {
            let now = start_time.elapsed();
            let millis = now.unwrap().as_millis();

            let theorical_time: f64 = 1000.0 / (t as f64);
            let theorical_time: f64 = theorical_time * self.timestamp as f64;
            if theorical_time <= millis as f64
            {
                self.timestamp = self.timestamp + 1;
                return true;
            }
            false
        }

        pub fn update_game_datas(& mut self)
        {
            let tmp_teams = self.teams.clone();
            for mut team in & mut self.teams
            {
                let tmp = &mut team.clone();
                for player in & mut team.players
                {
                    player.life -= 1;
                    if player.life == 0
                    {
                        // remove player from team
                        tmp.players.retain(|p| p.id != player.id);
                    }
                    if player.actions.len() > 0
                    {
                        player.actions[0].count = player.actions[0].count - 1;
                    }                    
                }
                let mut total_players = tmp_teams.iter().map(|team| team.players.len() as u16).sum::<u16>() + 1;
                total_players += tmp_teams.iter().map(|team| team.eggs.len() as u16).sum::<u16>() + 1;
                for egg in & mut team.eggs
                {
                    egg.count = egg.count - 1;
                    if egg.count == 0
                    {
                        let mut rng = thread_rng();
                        team.players.push(Player
                            {
                                id: total_players as u32,
                                port: 42, // fill with stream port
                                coord: egg.coord.clone(),
                                ivt: Ressources::new(),
                                life: 1260,
                                orientation: match rng.gen_range(0..4)
                                {
                                    0 => Orientation::N,
                                    1 => Orientation::E,
                                    2 => Orientation::S,
                                    3 => Orientation::O,
                                    _ => Orientation::N,
                                },
                                level: 1,
                                actions: Vec::new(),
                            }
                        );
                        //remove egg
                        //println!("tmp -----> {:?}", tmp);
                        tmp.eggs.retain(|e| e.id != egg.id);
                        //println!("tmp -----> {:?}", tmp);
                        //tmp.eggs.retain(|e| e.id != egg.id);
                    }
                }
                team = &mut tmp.clone();
            }
        }
    }


/**********************************************************************
 * Struct ToSend
***********************************************************************/
    struct ToSend
    {
        gfx_pkt: Vec<String>,
        client: String,
    }
}