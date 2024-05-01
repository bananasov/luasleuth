pub mod instructions;
pub mod constants;
pub mod upvalues;
pub mod debug_info;
pub mod string;

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

#[derive(Debug)]
pub struct Prototype {
    pub name: Option<String>,
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub number_of_params: u8,
    pub is_vararg: bool,
    pub max_stack_size: u8,
    pub instructions: Vec<instructions::Instruction>,
    pub constants: Vec<constants::Constant>,
    pub upvalues: Vec<upvalues::Upvalue>,
    pub debug_info: debug_info::DebugInfo,
}

#[derive(Debug)]
pub struct Bytecode {
    pub header: Header,
    pub prototype: Prototype,
}