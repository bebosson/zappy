pub mod paquet_crafter
{
    use crate::action::action::{ActionResult, ReadyAction, Action};
    use crate::find_player_from_id;
    use crate::player::player::Orientation;
    use crate::teams::team::Team;

    pub fn craft_gfx_packet_post_action(ready_action: &ReadyAction, action_result: &Option<ActionResult>, teams: &Vec<Team>) -> Option<String>
    {
        let player = find_player_from_id(teams.clone(), &ready_action.id).unwrap();
        let orientation = match player.orientation
        {
            Orientation::N => 1,
            Orientation::E => 2,
            Orientation::S => 3,
            Orientation::O => 4,
        };
        let mut cmd: Option<String> = match ready_action.action.action_name.as_str()
        {
            "avance" | "droite" | "gauche" =>
            {
                Some(player.packet_gfx_player_position())
            },
            "voir" =>
            {
                //Some(format!("bct {} {} {} {} {} {} {} {}"))
                Some(format!("bct"))
            },
            "inventaire" =>
            {
                Some(format!("pin"))
            }
            "prend" =>
            {
                Some(format!("pgt"))
            },
            "pose" =>
            {
                Some(format!("pr"))
            },
            "expulse" =>
            {
                Some(format!("pex"))
            },
            "broadcast" =>
            {
                Some(format!("pbc"))
            },
            "incantation" =>
            {
                Some(format!("pic"))
            },
            "fork" =>
            {
                Some(format!("pfk"))
            },
            "connect_nbr" => None,
            _ => None,
        };

        cmd
    }

    pub fn craft_gfx_packet_pre_action(action: &Action, teams: &Vec<Team>) -> Option<String>
    {
        let mut cmd = match action.action_name.as_str()
        {
            //"avance" => Some("ppo"),
            //"droite" => Some("ppo"),
            //"gauche" => Some("ppo"),
            //"voir" => Some("bct"),
            //"inventaire" => Some("pin"),
            //"prend" => Some("pgt"),
            //"pose" => Some("pr"),
            //"expulse" => Some("pex"),
            //"broadcast" => Some("pbc"),
            "incantation" => Some("pic"),
            //"fork" => Some("pfk"),
            //"connect_nbr" => None,
            _ => None,
        };

        //gfx_pkt = format!("{}", );   
        None
    }
}