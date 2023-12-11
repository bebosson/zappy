pub mod cell{
    use crate::ressources::ressources::Ressources;
    
    #[derive(Debug)]
    pub struct Point{
        x: u32,
        y: u32,
    }
    pub struct Cell{
        ressources: Ressources,
        //players: linkedList<Player>,
        //egg: LinkedList<Egg>,
    }

    impl Point{
        pub fn new(x: u32, y: u32) -> Self
        {
            Point {x, y}
        }
    }
}