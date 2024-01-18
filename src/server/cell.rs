pub mod cell
{
    use crate::ressources::ressources::Ressources;
    

/**********************************************************************
 * Struct Point
***********************************************************************/
    #[derive(Debug, Clone, Copy)]
    pub struct Point
    {
        pub x: u8,
        pub y: u8,
    }

    impl Point
    {
        pub fn new(x: u8, y: u8) -> Self
        {
            Point {x, y}
        }
    }


/**********************************************************************
 * Struct Cell
***********************************************************************/
    #[derive(Debug, Clone)]
    pub struct Cell
    {
        pub ressources: Ressources,
        //players: linkedList<Player>, // not sure to implement here
        //egg: LinkedList<Egg>, // not sure to implement here
    }

}