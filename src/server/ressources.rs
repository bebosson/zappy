pub mod ressources{
    use std::collections::HashMap;

    
    #[derive(Debug, Clone)]
    pub struct Ressources{
        pub food: u8,
        pub sibur: u8,
        pub mendiane: u8,
        pub linemate: u8,
        pub deraumere: u8,
        pub phiras: u8,
        pub thystate: u8,
    }

    impl Ressources{
        pub fn new() -> Self {
            Ressources { food: 0, sibur: 0, mendiane: 0, linemate: 0, deraumere: 0, phiras: 0, thystate: 0 }
        }
    }

    pub const LEVEL_RESSOURCES_REQUIREMENT: Vec<HashMap<String, u8>> = vec!
    [
        { // level 1-2
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 1);
            map.insert("linemate".to_string(), 1);
            map.insert("deraumere".to_string(), 0);
            map.insert("sibur".to_string(), 0);
            map.insert("mendiane".to_string(), 0);
            map.insert("phiras".to_string(), 0);
            map.insert("thystate".to_string(), 0);
            map
        },
        { // level 2-3
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 2);
            map.insert("linemate".to_string(), 1);
            map.insert("deraumere".to_string(), 1);
            map.insert("sibur".to_string(), 1);
            map.insert("mendiane".to_string(), 0);
            map.insert("phiras".to_string(), 0);
            map.insert("thystate".to_string(), 0);
            map
        },
        { // level 3-4
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 2);
            map.insert("linemate".to_string(), 2);
            map.insert("deraumere".to_string(), 0);
            map.insert("sibur".to_string(), 1);
            map.insert("mendiane".to_string(), 0);
            map.insert("phiras".to_string(), 2);
            map.insert("thystate".to_string(), 0);
            map
        },
        { // level 4-5
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 4);
            map.insert("linemate".to_string(), 1);
            map.insert("deraumere".to_string(), 1);
            map.insert("sibur".to_string(), 2);
            map.insert("mendiane".to_string(), 0);
            map.insert("phiras".to_string(), 1);
            map.insert("thystate".to_string(), 0);
            map
        },
        { // level 5-6
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 4);
            map.insert("linemate".to_string(), 1);
            map.insert("deraumere".to_string(), 2);
            map.insert("sibur".to_string(), 1);
            map.insert("mendiane".to_string(), 3);
            map.insert("phiras".to_string(), 0);
            map.insert("thystate".to_string(), 0);
            map
        },
        { // level 6-7
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 6);
            map.insert("linemate".to_string(), 1);
            map.insert("deraumere".to_string(), 2);
            map.insert("sibur".to_string(), 3);
            map.insert("mendiane".to_string(), 0);
            map.insert("phiras".to_string(), 1);
            map.insert("thystate".to_string(), 0);
            map
        },
        { // level 7-8
            let mut map = HashMap::new();
            map.insert("nb_players".to_string(), 6);
            map.insert("linemate".to_string(), 2);
            map.insert("deraumere".to_string(), 2);
            map.insert("sibur".to_string(), 2);
            map.insert("mendiane".to_string(), 2);
            map.insert("phiras".to_string(), 2);
            map.insert("thystate".to_string(), 1);
            map
        }
    ];
}