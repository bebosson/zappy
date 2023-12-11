
pub mod game{
    use std::collections::LinkedList;

    use crate::{teams::team::Team, args::args::Args};

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
    }
}