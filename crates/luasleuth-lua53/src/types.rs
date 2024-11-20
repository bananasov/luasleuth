pub mod constants;
pub mod debug_info;
pub mod instructions;
pub mod upvalues;

use luasleuth_common::{
    types::{Array, LuaString},
    CommonCtx, Version,
};
use scroll::{ctx, Pread};

#[derive(Debug, Pread)]
pub struct Header {
    pub signature: [u8; 4],
    pub version: Version,
    pub format_version: u8,
    pub error_correction_data: [u8; 6],
    pub size_of_int: u8,
    pub size_of_size_t: u8,
    pub size_of_instruction: u8,
    pub size_of_integer: u8,
    pub size_of_lua_number: u8,
    pub luac_int: u64,
    pub luac_num: f64,
}

#[derive(Debug)]
pub struct Prototype<'a> {
    pub source: LuaString<'a>,
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub number_of_parameters: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub instructions: Array<instructions::Instruction>,
    pub constants: Array<constants::Constant<'a>>,
    pub upvalues: Array<upvalues::Upvalue>,
    pub prototypes: Array<Prototype<'a>>,
    pub debug_info: debug_info::DebugInfo<'a>,
}

#[derive(Debug)]
pub struct Bytecode<'a> {
    pub header: Header,
    pub size_of_upvalues: u8,
    pub prototype: Prototype<'a>
}

impl<'a> ctx::TryFromCtx<'a, CommonCtx> for Prototype<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(from: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let source: LuaString = from.gread_with(offset, ctx)?;
        let line_defined: u32 = from.gread_with(offset, ctx.endianness)?;
        let last_line_defined: u32 = from.gread_with(offset, ctx.endianness)?;
        let number_of_parameters: u8 = from.gread_with(offset, ctx.endianness)?;
        let is_vararg: u8 = from.gread_with(offset, ctx.endianness)?;
        let max_stack_size: u8 = from.gread_with(offset, ctx.endianness)?;

        let instructions: Array<instructions::Instruction> = from.gread_with(offset, ctx)?;
        let constants: Array<constants::Constant> = from.gread_with(offset, ctx)?;
        let upvalues: Array<upvalues::Upvalue> = from.gread_with(offset, ctx)?;
        let prototypes: Array<Prototype> = from.gread_with(offset, ctx)?;
        let debug_info: debug_info::DebugInfo = from.gread_with(offset, ctx)?;

        Ok((
            Prototype {
                source,
                line_defined,
                last_line_defined,
                number_of_parameters,
                is_vararg,
                max_stack_size,
                instructions,
                constants,
                upvalues,
                prototypes,
                debug_info,
            },
            *offset,
        ))
    }
}
