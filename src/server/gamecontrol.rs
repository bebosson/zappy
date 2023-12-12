
pub mod game{
    use std::{collections::{LinkedList, HashMap}, fmt};

    use crate::{teams::team::Team, args::args::Args, player::player:: Player};

    #[derive(Debug)]
    pub struct GameController{
        pub x: u8,
        pub y: u8,
        // cases: Vec<Vec<Cell>>,
        pub teams: Vec<Team>,
        pub timestamp: u32,
        //  recv_pkt: Vec<Tcphdr(?)>
        //  send_pkt: Vec<ToSend>
    }

    struct ToSend{
        gfx_pkt: Vec<String>,
        client: String,
    }

    impl GameController{
        pub fn new(args: &Args) -> Self
        {
            let mut vec_teams: Vec<Team> = vec![];

            args.n
                .iter()
                .map(|x| vec_teams.push(Team::new(&x.clone())))
                .for_each(drop);

            GameController{
                x: args.x,
                y: args.y,
                teams: vec_teams,
                timestamp: 0,
            }
        }

        pub fn get_team_and_push(& mut self, teamname: &String, id: u32)
        {
            for i in & mut self.teams
            {
                if i.name.eq(teamname) == true
                {
                    i.players.push(Player::new_with_id(id))
                }
            }
        }

       
    }

   

    // impl fmt::Debug for GameController {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         writeln!(f, "GameController")?;
    //         writeln!(f, "x: {}", self.x)?;
    //         writeln!(f, "y: {}", self.y)?;
    //         writeln!(f, "teams: {:#?}", self.teams)
    //     }
    // }
}