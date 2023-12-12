pub mod ressources{
    
    #[derive(Debug)]
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
}