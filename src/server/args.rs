pub mod args
{
    use std::error::Error;
    use std::fmt;
    use std::collections::{HashSet, HashMap};


/************************************************************************************
 * ParsingError structure
*************************************************************************************/
    #[derive(Debug)]
    pub struct ParsingError
    {
        description : String
    }

    impl ParsingError
    {
        fn new(description : &str) -> ParsingError
        {
            ParsingError { description: description.to_string() }
        }
    }

    impl Error for ParsingError {}

    impl fmt::Display for ParsingError
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            write!(f, "{}", self.description)
        }
    }


/************************************************************************************
 * Args structure
*************************************************************************************/
    #[derive(Debug)]
    pub struct Args
    {
        pub n: Vec<String>,
        pub c: u8,
        pub p: u16,
        pub x: u8,
        pub y: u8,
        pub t: u16
    }


    /**************************************************************************************
     *  get teams name from the arg -n until -c is reach
     *  params : 
     *      env_args : arguments list
     *
     *  return :
     *      list of string with each team name
     *
     *  TODO:   - changer la dependance au -c (ex: il n'y a pa de -c)
     *          - -c placer avant -n ne doit pas declencher d'erreur
    **************************************************************************************/
    fn get_team_name(env_args: &Vec<String>) -> Result<Vec<String>, ParsingError>
    {
        let flag_n = "-n";
        let next_flag: &str = "-";
        let name_team: Vec<String>;

        // find the -n flag in env_args
        let index_flag_n = env_args.iter()
            .position(|r| r == flag_n)
            .ok_or(ParsingError::new("argument -n missing"))?;
        
        // verify if the flag is found twice
        if let Some(_) = env_args[index_flag_n + 1..].iter()
            .position(|r| r == flag_n)
        {
            return Err(ParsingError::new("argument -n specified twice"));
        };
        
        // find the next flag in env_args
        let index_next_flag = env_args[index_flag_n + 1..].iter()
            .position(|r| r.starts_with(next_flag));

        let index_next_flag = match index_next_flag
        {
            Some(val) => val + index_flag_n,
            None => env_args.len() - 1
        };

        if index_next_flag < index_flag_n + 1
        {
            let msg = "".to_string();
            return Err(ParsingError::new(&msg));
        }
        name_team = env_args[index_flag_n + 1..index_next_flag + 1].to_vec();
        Ok(name_team)
    }


    /**************************************************************************************
     * This function will return the integer value found just after the flag parameter
     * (if we found the flag in the env_args list)
     * params:
     *      env_args : list of arguments
     *      flag : can be -c, -t, -p, -x or -y
     * 
     * return:
     *      integer that follow the flag parameter
    **************************************************************************************/
    fn get_integer_param(env_args: &Vec<String>, flag: String) -> Result<u16, ParsingError>
    {
        let flag_tmp: &str = flag.as_str();
        let msg : String = format!("parameter {} is missing", flag);

        // find the flag in env_args
        let index_flag = env_args
            .iter()
            .position(|r| r == flag_tmp)
            .ok_or(ParsingError::new(&msg))?;

        // verify if the flag is found twice
        if let Some(_) = env_args[index_flag + 1..].iter()
            .position(|r| r == flag_tmp)
        {
            let msg = format!("argument {} specified twice", flag);
            return Err(ParsingError::new(&msg));
        };

        // test if the integer value is present
        if index_flag >= env_args.len() - 1
        {
            let msg : String = format!("parameter {} require integer but missing here", flag);
            return Err(ParsingError::new(&msg));
        }

        // test if a param has two value --> if yes, return Err()
        if index_flag < env_args.len() - 2
        {
            let tmp = &env_args[index_flag + 2];
            if tmp.starts_with("-") == false
            {
                let msg : String = format!("parameter {} require only one value", flag);
                return Err(ParsingError::new(&msg));
            }
        }

        // convert String parameter value into integer
        let player_by_team_slice = &env_args[index_flag + 1];
        let ret = player_by_team_slice
            .parse::<u16>()
            .ok();

        let msg : String = format!("Error: {} option require integer", flag);
        ret.ok_or(ParsingError::new(&msg))
    }



    /**************************************************************************************
     * Return the duplicates from the list list in prameter
    **************************************************************************************/
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
        pub fn new(env_args: Vec<String>) -> Result<Self, ParsingError>
        {
            // get the -n param value(s), this param represent the team(s) name
            let name_teams: Vec<String> = get_team_name(&env_args)?;

            // trigger an error if a team name is duplicate
            let duplicates = find_duplicates(&name_teams);
            if duplicates.is_empty() == false
            {
                return Err(ParsingError::new(format!("team name {:?} are duplicates", duplicates).as_str()));
            }

            // get the -c param value, this param represent the number of client in a team
            let num_by_team = get_integer_param(&env_args, "-c".to_string())? as u8;

            // get the -x param value, 
            let x = get_integer_param(&env_args, "-x".to_string())? as u8;

            // get the -y param value, 
            let y = get_integer_param(&env_args, "-y".to_string())? as u8;

            // get the -t param value, 
            let t = get_integer_param(&env_args, "-t".to_string())?;

            // get the -p param value, 
            let p = get_integer_param(&env_args, "-p".to_string())?;

            return Ok(Args{ n: name_teams,
                            c: num_by_team,
                            x: x,
                            y: y,
                            p: p,
                            t: t});
        }
        
        pub fn client_all_connect(&self, hashmap: &mut HashMap<String, u8>) -> bool
        {
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
