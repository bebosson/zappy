pub mod args{
    use std::error::Error;
    use std::fmt;
    use std::collections::{HashSet, HashMap};

    #[derive(Debug)]
    pub struct Args{
        pub n: Vec<String>,
        pub c: u8,
        pub p: u16,
        pub x: u8,
        pub y: u8,
        pub t: u16
    }

    #[derive(Debug)]
    pub struct ParsingError {
        description : String
    }

    impl ParsingError {
        fn new(description : &str) -> ParsingError {
            ParsingError { description: description.to_string() }
        }
    }

    impl Error for ParsingError {}

    impl fmt::Display for ParsingError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.description)
        }
    }

/*
    get teams name from the arg -n until -c is reach

    params : 
        env_args : arguments list
    
    return :
        list of string with each team name

    TODO:   - changer la dependance au -c (ex: il n'y a pa de -c)
            - -c placer avant -n ne doit pas declencher d'erreur
*/

    fn get_team_name(env_args: &Vec<String>) -> Result<Vec<String>, ParsingError>
    {
        let flag_n = "-n";
        let flag_c: &str = "-c";
        let name_team: Vec<String>;

        let index_n = env_args.iter()
            .position(|r| r == flag_n)
            .ok_or(ParsingError::new("argument -n missing"))?;
        
        if let Some(_) = env_args[index_n + 1..].iter()
            .position(|r| r == flag_n) {
                return Err(ParsingError::new("argument -n specified twice"));
            };
        
        
        let index_c = env_args.iter()
            .position(|r| r == flag_c)
            .ok_or(ParsingError::new("argument -c missing"))?;

        println!("index_c {:?}", index_c);
        println!("index_n {:?}", index_n);
        if index_c <= index_n + 1 {
            return Err(ParsingError::new("your -c argument must be after -n or you must specify team name"));
        }
        name_team = env_args[index_n + 1..index_c].to_vec();
        Ok(name_team)
    }

/*
    get teams name from the arg -n until -c is reach

    params : 
        env_args : arguments list
    
    return :
        list of string with each team name
*/
    fn get_nb_team(env_args: &Vec<String>) -> Result<u8, ParsingError>
    {
        let flag_c: &str = "-c";
        let index_c = env_args
            .iter()
            .position(|r| r == flag_c)
            .ok_or(ParsingError::new("argument -c missing"))?;

        if index_c >= env_args.len() - 1 {
            return Err(ParsingError::new("parameter -c require integer but missing here"));
        }

        let player_by_team_slice = &env_args[index_c + 1];
        let ret = player_by_team_slice
            .parse::<u8>()
            .ok();
        ret.ok_or(ParsingError::new("Error: -c option require integer"))
    }


    fn find_duplicates(list: &Vec<String>) -> Vec<String> 
    {
        let mut seen = HashSet::new();
        let mut duplicates = Vec::new();

        for item in list.iter() {
            if !seen.insert(item.clone()) {
                // If the item is not added to the set (i.e., it was already present), it's a duplicate
                duplicates.push(item.clone());
            }
        }
        duplicates
    }

    impl Args{
        pub fn parial_new(env_args: Vec<String>) -> Result<Self, ParsingError>{
            // env_args.
            // let vec_env_args: Vec<String> = env::args().collect();
            let name_teams: Vec<String> = get_team_name(&env_args)?;

            let duplicates = find_duplicates(&name_teams);
            if duplicates.is_empty() == false {
                return Err(ParsingError::new(format!("team name {:?} are duplicates", duplicates).as_str()));
            }

            let num_by_team = get_nb_team(&env_args)?;
            //println!("wesh c'est quoi les bails {}", num_by_team);

            let args = Args{
                n: name_teams,
                c: num_by_team,
                x: 10,
                y: 10,
                p: 1312,
                t: 1,
            };
            //println!("igo faut qu j me taille {:?}", args);

            Ok(args)
        }
    
        pub fn client_all_connect(&self, hashmap: &mut HashMap<String, u8>) -> bool
        {
                let len_team = self.n.len();
                match hashmap
                    .iter()
                    .filter(|&(_, &key)| key == self.c)
                    .map(|_| 1)
                    .count() {
                        hashmap_len if hashmap_len == self.n.len() => {return true;}
                        _ => {return false;}
                    }
                
                //on regrette rien 
        }
    }
}
