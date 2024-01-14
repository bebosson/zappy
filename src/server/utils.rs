pub mod utils
{
     /***********************************************************************************
     * Simple implementation of cpy_from_slice use for translate the buffer receive 
     * in the stream to the teamname
     * 
     * params:
     *      buffer: [u8; 32]
     * 
     * return:
     *       String
    *********************************************************************************/
    pub fn copy_until_char(buffer: &[u8], char: u8) -> String
    {
        let string_dst = buffer
            .iter() // into_iter 
            .take_while(|&x| *x != char)
            .map(|x| *x as char)
            .collect();
        string_dst
    }
}