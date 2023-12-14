pub mod cell
{
    use crate::ressources::ressources::Ressources;
    

/**********************************************************************
 * Struct Point
***********************************************************************/
    #[derive(Debug, Clone)]
    pub struct Point
    {
        x: u32,
        y: u32,
    }

    impl Point
    {
        pub fn new(x: u32, y: u32) -> Self
        {
            Point {x, y}
        }
    }


/**********************************************************************
 * Struct Cell
***********************************************************************/
    #[derive(Debug)]
    pub struct Cell
    {
        pub ressources: Ressources,
        //players: linkedList<Player>, // not sure to implement here
        //egg: LinkedList<Egg>, // not sure to implement here
    }

}