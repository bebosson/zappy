
pub mod game_utils
{
    use std::collections::HashMap;

    use crate::action::action::{ReadyAction, SpecialActionParam, FORK, INCANTATION};
    use crate::cell::cell::Point;
    use crate::player;
    use crate::player::player::{Egg, Player, PlayerType};
    use crate::teams::team::Team;

    pub fn get_dead_player_list(teams: &Vec<Team>) -> Vec<(u32, PlayerType)>
    {
        let mut dead_list: Vec<(u32, PlayerType)> = Vec::new();

        for team in teams
        {
            for player in team.players.clone()
            {
                if player.life - 1 == 0
                {
                    dead_list.push((player.id, PlayerType::Player));
                }
            }
            for egg in team.eggs.clone()
            {
                if egg.life - 1 == 0
                {
                    dead_list.push((egg.id, PlayerType::Egg));
                }
            }
        }
        dead_list
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

    pub fn get_post_actions(teams: &Vec<Team>) -> Option<Vec<(u32, SpecialActionParam)>>
    {
        let mut actions: Vec<(u32, SpecialActionParam)> = Vec::new();

        for team in teams
        {
            for player in &team.players
            {
                if player.actions.len() > 0
                {
                    if player.actions[0].action_name == format!("fork") && player.actions[0].count == 0
                    {
                        actions.push((player.id, SpecialActionParam::ActionFork(player.id)));
                    }
                    else if player.actions[0].action_name == format!("incantation") && player.actions[0].count == 0
                    {
                        let mut ids: Vec<u32> = find_players_from_coord(player.coord.clone(), teams);
                        //let index = ids.iter().position(|x| *x == player.id).unwrap();
                        //ids.remove(index);
                        actions.push((player.id, SpecialActionParam::ActionIncantation(player.coord.clone(), player.level, ids)));
                    }
                }
            }
        }
        //println!("action for sending before pkt ---> {:?}", actions);
        if actions.len() == 0 { return  None; }
        Some(actions)
    }

    pub fn get_pre_actions(teams: &Vec<Team>) -> Option<Vec<(u32, SpecialActionParam)>>
    {
        let mut actions: Vec<(u32, SpecialActionParam)> = Vec::new();

        for team in teams
        {
            for player in &team.players
            {
                if player.actions.len() > 0
                {
                    if player.actions[0].action_name == format!("fork") && player.actions[0].count + 1 == FORK.count
                    {
                        actions.push((player.id, SpecialActionParam::ActionFork(player.id)));
                    }
                    else if player.actions[0].action_name == format!("incantation") && player.actions[0].count + 1 == INCANTATION.count
                    {
                        let mut ids: Vec<u32> = find_players_from_coord(player.coord.clone(), teams);
                        //let index = ids.iter().position(|x| *x == player.id).unwrap();
                        //ids.remove(index);
                        actions.push((player.id, SpecialActionParam::ActionIncantation(player.coord.clone(), player.level, ids)));
                    }
                }
            }
        }
        //println!("action for sending before pkt ---> {:?}", actions);
        if actions.len() == 0 { return  None; }
        Some(actions)
    }

    pub fn find_players_from_coord(coord: Point, teams: &Vec<Team>) -> Vec<u32>
    {
        let mut ids: Vec<u32> = Vec::new();

        for team in teams
        {
            for player in &team.players
            {
                if coord.x == player.coord.x && coord.y == player.coord.y
                {
                    ids.push(player.id);
                }
            }
        }
        ids
    }

    pub fn find_hatch_egg(teams: Vec<Team>) -> Option<Egg>
    {
        for team in teams
        {
            for egg in &team.eggs
            {
                if egg.count == 0
                {
                    return Some(egg.clone())
                }
            }
        }
        None
    }

    pub fn get_players_id_from_coord(coord: Point, teams: &Vec<Team>) -> Vec<u32>
    {
        let mut ids: Vec<u32> = Vec::new();

        for team in teams
        {
            for player in &team.players
            {
                if coord.x == player.coord.x && coord.y == player.coord.y
                {
                    ids.push(player.id);
                }
            }
        }
        ids
    }

    pub fn find_team_from_player_id(id: u32, teams: &Vec<Team>) -> &Team
    {
        let mut tmp: Option<&Team> = None;

        for team in teams
        {
            for player in &team.players
            {
                if id == player.id
                {
                    tmp = Some(team);
                }
            }
        }
        &tmp.unwrap() // ici faut gerer avec des opt
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