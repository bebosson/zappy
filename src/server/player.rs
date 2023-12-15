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

    #[derive(Debug)]
    pub struct Egg
    {
        count: u16,
        coord: Point,
    }

    #[derive(Debug)]
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

        pub fn avance(&mut self, height: u8, width: u8)
        {
            match self.orient {
                Orientation::N => self.coor.y += 1 % height as u32,
                Orientation::E => self.coor.x += 1 % width as u32,
                Orientation::S => self.coor.y -= 1 % height as u32,
                Orientation::O => self.coor.y -= 1 % width as u32,
            }
        }

        pub fn droite(&mut self)
        {
            match self.orient
            {
                Orientation::N => self.orient = Orientation::E,
                Orientation::E => self.orient = Orientation::S,
                Orientation::S => self.orient = Orientation::O,
                Orientation::O => self.orient = Orientation::N,
            }
        }

        pub fn gauche(&mut self)
        {
            match self.orient
            {
                Orientation::N => self.orient = Orientation::O,
                Orientation::E => self.orient = Orientation::N,
                Orientation::S => self.orient = Orientation::E,
                Orientation::O => self.orient = Orientation::S,
            }
        }

        // pub fn voir

        pub fn inventaire(&self) -> String
        {
            let str = format!("food: {}, sibur: {}, mediane: {}, linemate: {}, deraumere: {}, phiras: {}, thystate: {}",
                                    self.ivt.food, self.ivt.sibur, self.ivt.mendiane, self.ivt.linemate, self.ivt.deraumere, self.ivt.phiras, self.ivt.thystate);
            str
        }

        // pub fn prend
        // pub fn prend
        // pub fn expulse
        
    }
    
}

//