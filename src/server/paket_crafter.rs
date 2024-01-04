pub mod paquet_crafter
{
    use crate::action::action::{ActionResult, ReadyAction};
    use crate::teams::team::Team;

    pub fn craft_gfx_packet(ready_action: &ReadyAction, action_result: &Option<ActionResult>, teams: &Vec<Team>) -> Option<String>
    {
        //gfx_pkt = format!("{}", );
        None
    }
}