
pub mod game{
    use std::collections::LinkedList;

    use crate::{teams::teams::Teams, cell::cell::Cell};

    pub struct Game{
        x: u8,
        y: u8,
        cases: LinkedList<Cell>,
        teams: LinkedList<Teams>,
        timestamp: u32,
        // recv_pkt: 
    }
}