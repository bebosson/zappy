pub mod args;
pub mod cell;
pub mod gamer;
pub mod player;
mod ressources;
mod teams;
pub mod egg;
pub mod zappy;
use args::args::Args;




fn main() {
    let t: u16 = 24;

    println!("{:?}", Args::new(&t));
}
