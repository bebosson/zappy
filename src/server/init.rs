
pub mod init
{
    use rand::{thread_rng, Rng};

    use crate::ressources::ressources::Ressources;
    use crate::cell::cell::Cell;

    /********************************************************************************
     * from a x and y size, we create a 2d map fill by ressources on each cell
     * params:
     *      x: map width
     *      y: map height
     * 
     * return:
     *      vec<vec<Cell>> which is the 2d vector containing all the cells of the map 
    *********************************************************************************/
    pub fn init_map_cells(x: u8, y: u8) -> Vec<Vec<Cell>>
    {
        let mut map : Vec<Vec<Cell>> = Vec::with_capacity(y as usize);

        for _i in 0..y // si y est nul on initialise jamais ? 
        {
            let mut line : Vec<Cell> = Vec::with_capacity(x as usize);
            for _ in 0..x // pareil pour x ...
            {
                line.push(fill_map_cell());
            }
            map.push(line);
        }
        map
    }

    /********************************************************************************
     * by using random, fill a cell with random quantity of ressources
     * 
     * return:
     *      Cell fill randomly 
    *********************************************************************************/
    pub fn fill_map_cell() -> Cell
    {
        let mut rng = thread_rng();

        let ressource : Ressources = Ressources
        {
            /*    
            food        : rng.gen_range(0..4),
            sibur       : rng.gen_range(0..2),
            mendiane    : rng.gen_range(0..2),
            linemate    : rng.gen_range(0..2),
            deraumere   : rng.gen_range(0..2),
            phiras      : rng.gen_range(0..2),
            thystate    : rng.gen_range(0..2),
            */
            food        : 1,
            sibur       : 1,
            mendiane    : 1,
            linemate    : 1,
            deraumere   : 1,
            phiras      : 1,
            thystate    : 1,
        };
        let cell : Cell = Cell {ressources : ressource};
        cell
    }

    impl std::fmt::Display for Ressources {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {} {} {} {} {} {}", self.food, self.sibur, self.mendiane, self.linemate, self.deraumere, self.phiras, self.thystate)
        }
    }
}