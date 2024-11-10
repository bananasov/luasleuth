pub mod instructions;

use scroll::Pread;

use crate::common::string::LuaString;

#[derive(Debug, Pread)]
pub struct Header {
    pub signature: u32,
    pub version: u8,
    pub format_version: u8,
    pub error_detection_data: [u8; 6],
    pub size_of_int: u8,
    pub size_of_size_t: u8,
    pub size_of_instruction: u8,
    pub size_of_lua_integer: u8,
    pub size_of_lua_number: u8,
    pub luac_int: u64,
    pub luac_num: f64,
}

#[derive(Debug)]
pub struct Prototype<'b> {
    pub source: LuaString<'b>,
    pub line_defined: i32,
    pub last_line_defined: i32,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
}

#[derive(Debug)]
pub struct Bytecode<'b> {
    pub header: Header,
    pub size_of_upvalues: u8,
    pub prototype: Prototype<'b>
}
