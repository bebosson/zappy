pub mod paquet_crafter
{
    use crate::{find_player_from_id, ressources};
    use crate::game_utils::game_utils::find_team_from_player_id;
    use crate::teams::team::Team;
    use crate::gamecontrol::game::{GameController};
    use crate::ressources::ressources::Ressources;
    use crate::cell::cell::{Point, Cell};
    use crate::player::player::{Orientation, Player, PlayerType};
    use crate::action::action::{ActionResult, ReadyAction, Action};


    /*
    **  Craft GFX packet at the beginning of an action 
    **  (before decrementing `count` cycles)
    **  params:
    **      action: the action that need to send a gfx_pkt
    **              (only for fork and incantation)
    **      teams:  teams
    **  return:
    **      Option<Vec<String>>: list of pkt to send to gfx server
    **      None if the execution fail or if the cmd doesn't need to send gfx pkt
    **  
    */
    pub fn craft_gfx_packet_action_receive(action: &Action, teams: &Vec<Team>) -> Option<Vec<String>>
    {
        // TODO : attention ici pour gfx il faut crafter 2 types de paquets 
        // pic pour debu incantation et pfk pour debut de fork
        // mais je dois récupérer l'id des joueurs et la il y a un petit probleme
        // il faudra peut etre renvoyer un tuple (vec<string>, id) dans receive_action au lieu d'un simple vec<string>

        let mut pkt: Vec<String> = Vec::new();
        //let player = get_player_from_new_action();
        match action.action_name.as_str()
        {
            //"incantation" => pkt.push(get_incantation_players_id(id, teams)),
            //"fork" => pkt.push(get_player_id_from_action()),
            _ => (),
        }
        Some(Vec::new())
    }

    pub fn craft_client_packet_action_ready(ready_action_ref: &ReadyAction, action_result_ref: &Option<ActionResult>, game_ctrl: &GameController) -> Option<Vec<String>>
    {
        let ready_action: ReadyAction = ready_action_ref.clone();
        let action_result = action_result_ref.as_ref().clone().unwrap();
        let mut pkts: Vec<String> = Vec::new();
        let teams = game_ctrl.teams.clone();
        let player = find_player_from_id(teams.clone(), &ready_action.id).unwrap();
        let team = find_team_from_player_id(player.id, &teams.clone());

        match ready_action.action.action_name.as_str()
        {
            "voir" =>
            {
                match action_result
                {
                    ActionResult::ActionVecHashMap(x) =>
                    {
                        // attention ici ce n'est pas juste
                        // je dois renvoyer case1, case2 avec case 1 = phiras player food food
                        // actuellement je renvoie pas ca
                        let mut voir_pkt: String = format!("");
                        for elem in x
                        {
                            println!("{:?}", elem);
                            let ressources_name = ["food", "linemate", "sibur", "phiras", "thystate", "mendiane", "deraumere"];
                            for ressource_name in ressources_name
                            {
                                if let Some(val) = elem.get(ressource_name)
                                {
                                    for i in 0..*val
                                    {
                                        voir_pkt.push_str(&format!("{} ", ressource_name));
                                    }
                                }
                            }
                            if x[x.len()- 1] != *elem
                            {
                                voir_pkt.push_str(&format!(", "));
                            }
                        }
                        pkts.push(format!("{}", voir_pkt));
                    }
                    _ => ()
                };
            },
            "inventaire" =>
            {
                match action_result
                {
                    ActionResult::ActionHashMap(x) =>
                    {
                        // ici c'est plus simple d'aller checker direct dans ivt du player plutot que dans la hashmap x
                        pkts.push(format!("food {}, phiras {}, sibur {}, mendiane {}, linemate {}, thystate {}, deraumere {}",
                                    player.ivt.food,
                                    player.ivt.phiras,
                                    player.ivt.sibur,
                                    player.ivt.mendiane,
                                    player.ivt.linemate,
                                    player.ivt.thystate,
                                    player.ivt.deraumere));
                    }
                    _ => ()
                };
            },
            "connect_nbr" => { pkts.push(format!("{}", teams[0].connect_nbr)); },
            "avance" | "droite" | "gauche" | "fork" | "broadcast" => { pkts.push("ok".to_string()); },
            "prend" | "pose" | "expulse" =>
            {
                match action_result
                {
                    ActionResult::ActionBool(x) =>
                    {
                        if x == &true   { pkts.push("ok".to_string()); }
                        else            { pkts.push("ko".to_string()); }
                    }
                    _ => { pkts.push("ko".to_string()); }
                };
                
            },
            "incantation" =>
            {
                pkts.push(format!("niveau actuel : {}", player.level + 1));
            },
            _ => (),
        };
        println!(" les pkt pour le client sont ----------> {:?}", pkts);
        Some(pkts)
    }

    /*
    **  Craft GFX packet when a action finish to wait `count` cycles
    **  params:
    **      ready_action_ref:   the action ready to execute
    **      action_result:      result of action 
    **                          (true or false if the return of command is boolean,
    **                          string if the command is `voir` or `inventaire`...)
    **      game_ctrl:          game data
    **  return:
    **      Option<Vec<String>>: list of pkt to send to gfx server
    **      None if the execution fail or if the cmd doesn't need to send gfx pkt
    **  
    */
    pub fn craft_gfx_packet_action_ready(ready_action_ref: &ReadyAction, action_result_ref: &Option<ActionResult>, game_ctrl: &GameController) -> Option<Vec<String>>
    {
        let ready_action: ReadyAction = ready_action_ref.clone();
        let action_result = action_result_ref.as_ref().clone().unwrap();
        let mut cmd: Vec<String> = Vec::new();
        let teams = game_ctrl.teams.clone();
        let cells = game_ctrl.cells.clone();
        let player = find_player_from_id(teams.clone(), &ready_action.id).unwrap();
        match ready_action.action.action_name.as_str()
        {
            "voir" => { return None; },
            "inventaire" => { return None; },
            "connect_nbr" => { return None; },
            "fork" => { cmd.push(packet_gfx_fork(player.id)); },
            "broadcast" => { cmd.push(packet_gfx_broadcast(player.id, ready_action.action.arg.unwrap())); },
            "avance" | "droite" | "gauche" => { cmd.push(packet_gfx_player_position(player.id, player.coord, player.orientation)); },
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
                for team in teams
                {
                    for tmp_player in team.players
                    {
                       cmd.push(packet_gfx_player_position(tmp_player.id, tmp_player.coord, tmp_player.orientation));
                    }
                }
            },
            "incantation" =>
            {
                // ICI le pkt envoye est celui de fin d'incantation
                // TODO :   creer le paquet de debut d'incantation 
                //          + creer le paquet de debut d'incantation
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
            _ => (),
        };
        Some(cmd)
    }

    pub fn craft_gfx_packet_die(ids: &Vec<(u32, PlayerType)>) -> Option<Vec<String>>
    {
        let mut gfx_pkt: Vec<String> = Vec::new();
        let mut break_bool: bool = false;

        for id in ids 
        {
            match id.1
            {
                PlayerType::Player => { gfx_pkt.push(packet_gfx_player_die(id.0)); },
                PlayerType::Egg => { gfx_pkt.push(packet_gfx_egg_die(id.0)); },
            };
        }
        if gfx_pkt.len() == 0 { return None; }
        Some(gfx_pkt)
    }

    pub fn craft_client_packet_die(dead_players: &Vec<(u32, PlayerType)>) -> Option<Vec<String>>
    {
        let mut pkts: Vec<String> = Vec::new();
        for dead_player in dead_players
        {
            pkts.push(format!("mort\n"));
        }
        if pkts.len() == 0 { return None; }
        Some(pkts)
    }


///////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// craft GFX pkt //////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////

    /*
    **  generate pkt for new player connexion
    */
    pub fn packet_gfx_player_connexion(player: &Player) -> String
    {
        format!("pnw {} {} {} {} {}",
                player.id,
                player.coord.x,
                player.coord.y,
                format_orientation(player.orientation.clone()),
                player.level)
    }

    /*
    **  generate pkt for player position
    */
    fn packet_gfx_player_position(id: u32, coord: Point, orientation: Orientation) -> String
    {
        format!("ppo {} {} {} {}\n",
                id,
                coord.x,
                coord.y,
                format_orientation(orientation))
    }

    /*
    **  generate pkt for player `pose` command
    */
    fn packet_gfx_pose(id: u32, arg: String) -> String
    {
        format!("pdr {} {}\n", id, get_ressource_index_by_name(arg))
    }

    /*
    **  generate pkt for player `prend` command
    */
    fn packet_gfx_prend(id: u32, arg: String) -> String
    {
        format!("pgt {} {}\n", id, get_ressource_index_by_name(arg))
    }

    /*
    **  generate pkt for player `expulse` command
    */
    fn packet_gfx_expulse(id: u32) -> String
    {
        format!("pex {}", id)
    }

    /*
    **  generate pkt for player `broadcast` command
    */
    fn packet_gfx_broadcast(id: u32, msg: String) -> String
    {
        format!("pbc {} {}\n", id, msg)
    }

    /*
    **  generate pkt for player `inventaire` command
    */
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

    /*
    **  generate pkt to check cell content
    */
    fn packet_gfx_case_content(coord: Point, cell: Cell) -> String
    {
        format!("bct {} {} {} {} {} {} {} {} {}",
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

    /*
    **  generate pkt for player `fork` command
    */
    fn packet_gfx_fork(id: u32) -> String
    {
        format!("pfk {}\n", id)
    }

    /*
    **  generate pkt for player `incantation` command
    */
    fn packet_gfx_incantation(coord: Point) -> String
    {
        format!("pie {} {} 1\n", coord.x, coord.y)
    }

    /*
    **  generate pkt when player level up
    */
    fn packet_gfx_level_up(id: u32, level: u8) -> String
    {
        format!("plv {} {}\n", id, level)
    }

    pub fn packet_gfx_egg_die(id: u32) -> String
    {
        format!("edi {}\n", id)
    }

    pub fn packet_gfx_player_die(id: u32) -> String
    {
        format!("pdi {}\n", id)
    }


    pub fn craft_client_packet_action_receive(actions: &Action, teams: &Vec<Team>) -> Option<Vec<String>>
    {
        Some(Vec::new())
    }

    pub fn packet_client_player_die() -> String
    {
        format!("mort\n")
    }

    pub fn packet_client_egg_die() -> String
    {
        format!("mort\n")
    }


///////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////// pkt crafter utils /////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////

    /*
    **  convert ressource string into ressource index
    */
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

    /*
    **  convert orientation type into orientation index
    **
    **  TODO: peut etre remplacer le retour par un integer ?
    */
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

    /*
    **  compare ref_player with another player and verify if incantation is finish
    **
    **  TODO: changer cette fonction pour plusieurs raisons:
    **  il faudrait pousser l'action d'incantation des joueurs subissant une
    **  incantation forcee par un autre joueur en haut de leur liste d'actions
    **  et ainsi mettre en pause les actions en cours
    **  on pourrait ainsi juste verifier player.actions[0].count
    */
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