
pub mod game{
    use std::collections::LinkedList;

    use crate::{teams::teams::Teams, cell::cell::Cell, args::args::Args};

    pub struct GameController{
        x: u8,
        y: u8,
        // cases: Vec<Vec<Cell>>,
        teams: Vec<Teams>,
        timestamp: u32,
        //  recv_pkt: Vec<Tcphdr(?)>
        //  send_pkt: Vec<ToSend>
    }

    struct ToSend{
        gfx_pkt: Vec<String>,
        client: String,
    }

    impl GameController{
        // pub fn new(args: &mut Args) -> Self
        // {
        //     GameController{
        //         x :  args.x,
        //         y: args.y,
        //         teams: args.n,
        //         t = args.t
        //     }
        // }
    }
}