
pub mod game
{
    use std::net::TcpStream;
    use std::time::SystemTime;

    use crate::action::action::Die;
    use crate::teams::team::Team;
    use crate::args::args::Args;
    use crate::player::player::{Player, Egg};
    use crate::cell::cell::Cell;
    use crate::init::init::init_map_cells;

/**********************************************************************
 * Struct GameController, this is the main structure of the program
***********************************************************************/
    #[derive(Debug)]
    pub struct GameController
    {
        pub x: u8,
        pub y: u8,
        pub cells: Vec<Vec<Cell>>,
        pub teams: Vec<Team>,
        pub timestamp: u32,
        pub stream_gfx: Option<TcpStream>,
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
                stream_gfx: None
            }
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
                    team.nb_total_players += 1;
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

        /*
        **  update life, action counter and egg counter
        **/
        pub fn update_game_datas(& mut self)
        {
            self.teams.iter_mut().for_each(|t| t.update());
            
            // TODO : il faut récupérer les datas modifiées pour créer 
            // les packets gfx résultant de l'update --> les mort
        }

        pub fn packet_gfx_ressources_map(&self) -> Vec<String>
        {
            let mut vec_packet: Vec<String> = vec![];
            let mut x = 0;
            let mut y = 0; 
        
            for line in &self.cells{
                x = 0;
                for cell in line{
                    vec_packet.push(format!("bct {} {} {}\n", x, y, cell.ressources));
                    x += 1;
                }
                y += 1;
            }
            vec_packet
        }

        pub fn packet_gfx_timestamp(&self) -> String
        {
            format!("sgt {}\n", self.timestamp)
        }

        pub fn packet_gfx_map_size(&self) -> String
        {
            format!("msz {} {}\n", self.x, self.y)
        }

        pub fn packet_gfx_all_teams(&self) -> Vec<Vec<String>>
        {
            let mut vec_gfx_packets_teams = vec![];
            for team in &self.teams
            {
                vec_gfx_packets_teams.push(team.packet_gfx_add_teams());
            }
            vec_gfx_packets_teams
        }


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////// Utils ////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

        pub fn get_players_eggs_id(&self) -> Vec<(u32, Die)>
        {
            let mut players_id = Vec::new();

            for team in self.teams.clone()
            {
                let mut tmp: Vec<(u32, Die)> = team
                    .eggs
                    .iter()
                    .map(|e| (e.id as u32, Die::EggDie))
                    .collect();
                players_id.append(&mut tmp);
                let mut tmp: Vec<(u32, Die)> = team
                    .players
                    .iter()
                    .map(|p| (p.id, Die::PlayerDie))
                    .collect();
                players_id.append(&mut tmp);
            }
            println!("-----------------> players id2 {:?}", players_id);
            players_id
        }

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////// Print for debug //////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

    }

}