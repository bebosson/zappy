pub mod zappy{
    use crate::{gamer::game::Game, args::args::Args};

    struct Zappy{
        game: Game,
        args: Args,
        sockfd: u16,
    }
}