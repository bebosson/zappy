pub mod player
{
    use crate::ressources::ressources::Ressources;
    use crate::cell::cell::Point;
    use crate::action::action::Action;

    #[derive(Debug, Clone)]
    pub enum Orientation
    {
        N,
        E,
        S,
        O
    }

    #[derive(Debug, Clone)]
    pub struct Egg
    {
        count: u16,
        coord: Point,
    }

    #[derive(Debug, Clone)]
    pub struct Player
    {
        pub id: u32,
        pub port: u16,
        pub coor: Point,
        pub ivt: Ressources,
        pub life: u16,
        pub orient: Orientation,
        pub level: u8,
        pub actions: Vec<Action>,
    }

    
    impl Player
    {
        pub fn new_with_id(id_a: u32) -> Self
        {
            Player
            {
                id: id_a,
                port: 0,
                coor: Point::new(0,0),
                ivt: Ressources::new(),
                life: 1260,
                orient: Orientation::N,
                level: 1,
                actions: Vec::new(),
            }
        }
    }
    
}