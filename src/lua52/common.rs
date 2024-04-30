pub mod instructions;

use scroll::Pread;

#[derive(Debug, Pread)]
pub struct Header {
    pub signature: u32,
    pub version: u8,
    pub format_version: u8,
    pub endianess_flag: u8,  
    pub size_of_int: u8,        
    pub size_of_sizet: u8,     
    pub size_of_instruction: u8,
    pub size_of_lua_number: u8, 
    pub integral_flag: u8,   
}