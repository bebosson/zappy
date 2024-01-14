
pub mod game_utils
{
    use crate::action::action::ReadyAction;
    use crate::player::player::{PlayerType, Player};
    use crate::teams::team::Team;

    /*
    **  take two list, one before and one after some players die. 
    **  Extract from the `before_id` list the dead players 
    **  params:
    **      before_id:  tuple with id player and player type (egg or player)
    **                  this list contains all players before updating
    *                   game datas (so players are not dead yet)
    **      after_id :  list of players after updating game data
    **                  (players dead are not in this list)
    **  return:
    **      vector of dead players
    **/
    pub fn get_dead_people_list(before_id: Vec<(u32, PlayerType)>, after_id: Vec<(u32, PlayerType)>) -> Vec<(u32, PlayerType)>
    {
        before_id
            .iter()
            .filter(|&x| !after_id.contains(&x))
            .chain(after_id.iter().filter(|&x| !before_id.contains(&x)))
            .cloned()
            .collect()
    }

    /*
    **  retreive player from it's id
    **  params:
    **      teams: all teams
    **      id: player id to find into `teams`
    **  return:
    **      Option<Player>: found player, None instead
    **/
    pub fn find_player_from_id(teams: Vec<Team>, id: &u32) -> Option<Player>
    {
        for team in teams
        {
            for player in team.players
            {
                if id == &player.id
                {
                    return Some(player);
                }
            }
        }
        None
    }

    /*
    **  find the ready action in the player actions list
    **  params:
    **      ready_action: ready action
    **      player: player concerned by this ready action
    **  return:
    **      usize: index of the action to find into player actions list
    **/
    pub fn find_index_action(ready_action: &ReadyAction, player: &Player) -> usize
    {
        let mut i: usize = 0;

        for action in &player.actions
        {
            if ready_action.action.action_name == action.action_name
                && action.count == 0
            {
                return i;
            }
            i = i + 1;
        }
        i
    }
}