pub mod instructions;

use scroll::Pread;

#[derive(Debug, Pread)]
pub struct Header {
    pub signature: u32,
    pub version: u8,
    pub format_version: u8,
    pub error_detection_data: [u8; 6],
    pub size_of_int: u8,
    pub size_of_sizet: u8,
    pub size_of_instruction: u8,
    pub size_of_lua_integer: u8,
    pub size_of_lua_number: u8,
    pub luac_int: u64,
    pub luac_num: f64,
}
