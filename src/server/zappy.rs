pub mod zappy{

    use crate::{args::args::Args, gamecontrol::game::GameController};

    struct Zappy{
        game: Option<GameController>,
        args: Option<Args>,
        sockfd: Option<u16>,
    }

    impl Zappy{
        // pub fn partial_new() -> Self{

        // }
    }
}