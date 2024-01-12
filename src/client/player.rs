pub mod player
{
    use crate::cell::cell::{Point, Cell};

    pub enum Orientation
    {
        N,
        E,
        S,
        O
    }

    pub struct Player
    {
        level           : u8,
        life            : u16,
        map             : Vec<Vec<Cell>>,
        coord           : Point,
        orientation     : Orientation,
        actions         :Vec<String>,
    }

    impl Player
    {
        pub fn avance(& mut self, height: u8, width: u8) -> bool
        {
            // let mut x: i8 = player.coord.x as i8;
            // let mut y: i8 = player.coord.y as i8;

            //println!("player coord X {}", player.coord.x);
            //println!("player coord Y {}", player.coord.y);
            match self.orientation
            {
                Orientation::N =>
                {
                    if self.coord.y == 0 { self.coord.y = height as i8 }
                    self.coord.y -= 1 % height as i8;
                },
                Orientation::E => 
                {
                    if self.coord.x == width as i8 - 1 { self.coord.x = -1 }
                    self.coord.x += 1 % width as i8
                },
                Orientation::S =>
                {
                    if self.coord.y == height as i8 - 1 { self.coord.y = -1 }
                    self.coord.y += 1 % height as i8
                },
                Orientation::O =>
                {
                    if self.coord.x == 0 { self.coord.x = width as i8 }
                    self.coord.x -= 1 % width as i8
                },
            }
            // player.coord.x = x as u8;
            // player.coord.y = y as u8;
            true
        }

        pub fn voir(&self, map: &Vec<Vec<Cell>>, teams: Vec<Team>) -> Vec<HashMap<String, u8>>
        {
            let mut cases_content: Vec<HashMap<String, u8>> = Vec::new();
            let cases_coord = get_cases_coord_from_player_pov(player, cells[0].len(), cells.len());
            for case_coord in cases_coord
            {
                let mut x = HashMap::new();
                let mut y = HashMap::new();
                x.insert("x".to_string(), case_coord.x);
                y.insert("y".to_string(), case_coord.y);
                cases_content.push(x);
                cases_content.push(y);
                cases_content.push(get_case_content_from_position(case_coord, cells, &teams));
            }
            cases_content
        }
    }
}