pub mod player
{
    pub enum Orientation
    {
        N,
        E,
        S,
        O
    }

    pub struct Player
    {
        level: u8,
        life: u16,
        map: Vec<Vec<Cell>>,
        coord: Point,
        orientation: Orientation,
        actions:Vec<String>,
    }
}