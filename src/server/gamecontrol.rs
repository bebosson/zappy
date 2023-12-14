
pub mod game
{
    use crate::teams::team::Team;
    use crate::args::args::Args;
    use crate::player::player::Player;
    use crate::cell::cell::Cell;
    use crate::init::init::init_map_cells;

/**********************************************************************
 * Struct GameController, this is the main structure of the program
***********************************************************************/
    #[derive(Debug)]
    pub struct GameController
    {
        pub x: u8,
        pub y: u8,
        pub cells: Vec<Vec<Cell>>,
        pub teams: Vec<Team>,
        pub timestamp: u32,
    }

    impl GameController
    {
        pub fn new(args: &Args) -> Self
        {
            let mut vec_teams: Vec<Team> = vec![];

            args.n
                .iter()
                .map(|x| vec_teams.push(Team::new(&x.clone())))
                .for_each(drop);

            GameController
            {
                x: args.x,
                y: args.y,
                cells : init_map_cells(args.x, args.y),
                teams: vec_teams,
                timestamp: 0,
            }
        }

        pub fn get_team_and_push(& mut self, teamname: &String, id: u32)
        {
            for i in & mut self.teams
            {
                if i.name.eq(teamname) == true
                {
                    i.players.push(Player::new_with_id(id))
                }
            }
        }
    }


/**********************************************************************
 * Struct ToSend
***********************************************************************/
    struct ToSend
    {
        gfx_pkt: Vec<String>,
        client: String,
    }
}