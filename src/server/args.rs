pub mod args{

    #[derive(Debug)]
    pub struct Args{
        pub n: Vec<String>,
        c: u8,
        // p: Option<u16>,
        // x: Option<u8>,
        // y: Option<u16>,
        // t: Option<u16>
    }
    fn get_arg_of_teams(env_args: &Vec<String>) -> Option<Vec<String>>
    {
        let flag_n = "-n";
        let flag_c: &str = "-c";
        let name_team: Vec<String>;
        let index_n = env_args.iter().position(|r| r == flag_n).unwrap();
        let index_c = env_args.iter().position(|r| r == flag_c).unwrap();
        
        name_team = env_args[index_n..index_c].to_vec();
        // println!("{:?}", env_args.index(flag_n));
        Some(name_team)
    }
    fn get_arg_num_by_team(env_args: &Vec<String>) -> Option<u8>
    {
        let flag_c: &str = "-c";
        let index_c = env_args.iter().position(|r| r == flag_c).unwrap();
        let player_by_team_slice = &env_args[index_c + 1 ..];
        if player_by_team_slice.len() > 1 {
            return None
        }
        else{
            return Some(player_by_team_slice[0].parse::<u8>().unwrap()) // DAMN
            
        }
        

    }
    impl Args{
        pub fn parial_new(env_args: Vec<String>) -> Self{
            // env_args.
            // let vec_env_args: Vec<String> = env::args().collect();
            let name_teams: Vec<String> = get_arg_of_teams(&env_args).unwrap();
            let num_by_team = get_arg_num_by_team(&env_args).unwrap();
            Args{
                n: name_teams,
                c: num_by_team,
            }
        }
    }
}