pub mod args{
    #[derive(Debug)]
    pub struct Args{
        // n: Option<LinkedList<String>>,
        // c: Option<u8>,
        // p: Option<u16>,
        // x: Option<u8>,
        // y: Option<u16>,
        t: Option<u16>
    }

    impl Args{
        pub fn new(p_t: &u16) -> Self{
            Args{
                t: Some(*p_t),
            }
        }
    }
}