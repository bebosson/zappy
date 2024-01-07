pub mod paquet_crafter
{
    use crate::action::action::{ActionResult, ReadyAction, Action};
    use crate::teams::team::Team;

    pub fn craft_gfx_packet_post_action(ready_action: &ReadyAction, action_result: &Option<ActionResult>, teams: &Vec<Team>) -> Option<String>
    {
        //gfx_pkt = format!("{}", );
        Some("ppo 1 1 1 1".to_string())
        //None
    }

    pub fn craft_gfx_packet_pre_action(action: &Action, teams: &Vec<Team>) -> Option<String>
    {
        //gfx_pkt = format!("{}", );
        Some("ppo 1 2 1 1".to_string())
        
        //None
    }
}