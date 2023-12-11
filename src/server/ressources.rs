pub mod ressources{
    
    #[derive(Debug)]
    pub struct Ressources{
        food: u8,
        sibur: u8,
        mendiane: u8,
        linemate: u8,
        deraumere: u8,
        phiras: u8,
        thystate: u8,
    }

    impl Ressources{
        pub fn new() -> Self {
            Ressources { food: 0, sibur: 0, mendiane: 0, linemate: 0, deraumere: 0, phiras: 0, thystate: 0 }
        }
    }
}