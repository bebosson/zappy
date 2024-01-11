pub mod cell
{

/**********************************************************************
 * Struct Point
***********************************************************************/
    pub struct Point
    {
        pub x: u8,
        pub y: u8,
    }

    impl Point
    {
        pub fn new(x: u8, y: u8) -> Self
        {
            Point {x, y}
        }
    }

/**********************************************************************
 * Struct Entity
***********************************************************************/

    pub struct Entity
    {
        name: String,
        count: u8,
        timestamp: u64,
    }

    impl Entity
    {
        pub fn new(name: &String, count: u8, timestamp: u64) -> Self
        {
            Entity
            {
                name: name.to_string(),
                count,
                timestamp,
            }
        }
    }

/**********************************************************************
 * Struct Cell
***********************************************************************/

    pub struct Cell
    {
        pub player: Entity,
        pub food: Entity,
        pub sibur: Entity,
        pub mendiane: Entity,
        pub linemate: Entity,
        pub deraumere: Entity,
        pub phiras: Entity,
        pub thystate: Entity,    
    }
}