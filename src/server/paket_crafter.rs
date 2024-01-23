pub mod paquet_crafter
{
    use std::ptr::read;

    use crate::action::action::{ActionResult, ReadyAction, Action};
    use crate::cell::cell::{Point, Cell};
    use crate::find_player_from_id;
    use crate::gamecontrol::game::GameController;
    use crate::player::player::{Orientation, Player};
    use crate::ressources::ressources::Ressources;
    use crate::teams::team::Team;

    pub fn craft_gfx_packet(ready_action_ref: &ReadyAction, action_result_ref: &Option<ActionResult>, game_ctrl: &GameController) -> Option<Vec<String>>
    {
        let ready_action: ReadyAction = ready_action_ref.clone();
        let action_result = action_result_ref.as_ref().clone().unwrap();
        let mut cmd: Vec<String> = Vec::new();
        let teams = game_ctrl.teams.clone();
        let cells = game_ctrl.cells.clone();
        let player = find_player_from_id(teams.clone(), &ready_action.id).unwrap();
        match ready_action.action.action_name.as_str()
        {
            "avance" | "droite" | "gauche" =>
            {
                cmd.push(packet_gfx_player_position(player.id, player.coord, player.orientation));
            },
            "voir" => { return None; },
            "inventaire" => { return None; },
            "prend" =>
            {
                let coord = player.coord.clone();
                if *action_result == ActionResult::ActionBool(false) { return None; }
                cmd.push(packet_gfx_prend(player.id, ready_action.action.arg.unwrap()));
                cmd.push(packet_gfx_inventaire(player.id, player.coord, player.ivt));
                cmd.push(packet_gfx_case_content(coord.clone(), cells[coord.y as usize][coord.x as usize].clone()));
            },
            "pose" =>
            {
                let coord = player.coord.clone();
                if *action_result == ActionResult::ActionBool(false) { return None; }
                cmd.push(packet_gfx_pose(player.id, ready_action.action.arg.unwrap()));
                cmd.push(packet_gfx_inventaire(player.id, player.coord, player.ivt));
                cmd.push(packet_gfx_case_content(coord.clone(), cells[coord.y as usize][coord.x as usize].clone()));
            },
            "expulse" =>
            {
                if *action_result == ActionResult::ActionBool(false) { return None; }
                cmd.push(packet_gfx_expulse(player.id));
                // attention ici on push toutes les positions des joueurs
                // TODO : ne push que les joueurs concerne par l'expulse
                // bebo to lyes: heuresement que tu precises 
                for team in teams
                {
                    for tmp_player in team.players
                    {
                       cmd.push(packet_gfx_player_position(tmp_player.id, tmp_player.coord, tmp_player.orientation));
                    }
                }
            },
            "broadcast" =>
            {
                cmd.push(packet_gfx_broadcast(player.id, ready_action.action.arg.unwrap()));
            },
            "incantation" =>
            {
                if *action_result == ActionResult::ActionBool(false) { return None; }
                cmd.push(packet_gfx_incantation(player.coord.clone()));
                for team in teams
                {
                    for tmp_player in team.players
                    {
                        if incantation_is_finish(&player, &tmp_player)
                        {
                            cmd.push(packet_gfx_level_up(tmp_player.id, tmp_player.level));
                        }
                    }
                }
            },
            "fork" =>
            {
                cmd.push(packet_gfx_fork(player.id));
            },
            "connect_nbr" => { return None; },
            _ => (),
        };

        Some(cmd)
    }

////////////////////////////////////////////////////////////////////////////////////////////////////////


    pub fn packet_gfx_player_connexion(player: &Player) -> String
    {
        format!("pnw {} {} {} {} {}",
                player.id,
                player.coord.x,
                player.coord.y,
                format_orientation(player.orientation.clone()),
                player.level)
    }

    fn packet_gfx_player_position(id: u32, coord: Point, orientation: Orientation) -> String
    {
        //"ppo #n X Y O\n"
        format!("ppo {} {} {} {}\n",
                id,
                coord.x,
                coord.y,
                format_orientation(orientation))
    }

    fn packet_gfx_pose(id: u32, arg: String) -> String
    {
        format!("pdr {} {}\n", id, get_ressource_index_by_name(arg))
    }

    fn packet_gfx_prend(id: u32, arg: String) -> String
    {
        format!("pgt {} {}\n", id, get_ressource_index_by_name(arg))
    }

    fn packet_gfx_expulse(id: u32) -> String
    {
        format!("pex {}\n", id) // add \n otherwise panic in server_gfx
    }

    fn packet_gfx_broadcast(id: u32, msg: String) -> String
    {
        format!("pbc {} {}\n", id, msg)
    }

    fn packet_gfx_inventaire(id: u32, coord: Point, ivt: Ressources) -> String
    {
        format!("pin {} {} {} {} {} {} {} {} {} {}\n",
                id,
                coord.x,
                coord.y,
                ivt.food,
                ivt.linemate,
                ivt.deraumere,
                ivt.sibur,
                ivt.mendiane,
                ivt.phiras,
                ivt.thystate)
    }

    fn packet_gfx_case_content(coord: Point, cell: Cell) -> String
    {
        // IL MANQUE UN \n ICI !!!!
        format!("bct {} {} {} {} {} {} {} {} {}\n",
            coord.x,
            coord.y,
            cell.ressources.food,
            cell.ressources.linemate,
            cell.ressources.deraumere,
            cell.ressources.sibur,
            cell.ressources.mendiane,
            cell.ressources.phiras,
            cell.ressources.thystate,
        )
    }

    fn packet_gfx_fork(id: u32) -> String
    {
        format!("pfk {}\n", id)
    }

    fn packet_gfx_incantation(coord: Point) -> String
    {
        format!("pie {} {} 1\n", coord.x, coord.y)
    }

    fn packet_gfx_level_up(id: u32, level: u8) -> String
    {
        format!("plv {} {}\n", id, level)
    }

////////////////////////////////////////////////////////////////////////////////////////////////////////

    fn get_ressource_index_by_name(ressource: String) -> u8
    {
        match ressource.as_str()
        {
            "food" => 0,
            "linemate" => 1,
            "deraumere" => 2,
            "sibur" => 3,
            "mendiane" => 4,
            "phiras" => 5,
            "thystate" => 6,
            _ => 42,
        }
    }

    fn format_orientation(orientation: Orientation) -> String
    {
        match orientation
        {
            Orientation::N => format!("{}", 1),
            Orientation::E => format!("{}", 2),
            Orientation::S => format!("{}", 3),
            Orientation::O => format!("{}", 4),
        }
    }

    fn incantation_is_finish(ref_player: &Player, player: &Player) -> bool
    {
        if ref_player.coord.x == player.coord.x &&
            ref_player.coord.y == player.coord.y
        {
            for action in &player.actions
            {
                if action.action_name == "incantation".to_string() &&
                    action.count == 0
                {
                    return true;
                }
            }
        }
        false
    }

}