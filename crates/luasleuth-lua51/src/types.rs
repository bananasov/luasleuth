pub mod constants;
pub mod debug_info;
pub mod instructions;

use luasleuth_common::types::{Array, Bytecode as BytecodeTrait, LuaString};
use luasleuth_common::{CommonCtx, Version};
use scroll::{ctx, Pread};

#[derive(Debug, Pread)]
pub struct Header {
    pub signature: [u8; 4],
    pub version: Version,
    pub format_version: u8,
    pub endianess_flag: u8,
    pub size_of_int: u8,
    pub size_of_size_t: u8,
    pub size_of_instruction: u8,
    pub size_of_lua_number: u8,
    pub integral_flag: u8,
}

#[derive(Debug)]
pub struct Prototype<'a> {
    pub source: LuaString<'a>,
    pub line_defined: i32,
    pub last_line_defined: i32,
    pub number_of_upvalues: u8,
    pub number_of_parameters: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub code: Array<instructions::Instruction>,
    pub constants: Array<constants::Constant<'a>>,
    pub prototypes: Array<Prototype<'a>>,
    pub debug_info: debug_info::DebugInfo<'a>,
}

#[derive(Debug)]
pub struct Bytecode<'a> {
    pub header: Header,
    pub prototype: Prototype<'a>,
}

impl<'a> ctx::TryFromCtx<'a, CommonCtx> for Prototype<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let source: LuaString = src.gread_with(offset, ctx)?;
        let line_defined: i32 = src.gread_with(offset, ctx.endianness)?;
        let last_line_defined: i32 = src.gread_with(offset, ctx.endianness)?;

        let number_of_upvalues: u8 = src.gread_with(offset, ctx.endianness)?;
        let number_of_parameters: u8 = src.gread_with(offset, ctx.endianness)?;
        let is_vararg: u8 = src.gread_with(offset, ctx.endianness)?;
        let max_stack_size: u8 = src.gread_with(offset, ctx.endianness)?;

        let code: Array<instructions::Instruction> = src.gread_with(offset, ctx)?;
        let constants: Array<constants::Constant> = src.gread_with(offset, ctx)?;
        let prototypes: Array<Prototype> = src.gread_with(offset, ctx)?;

        let debug_info: debug_info::DebugInfo = src.gread_with(offset, ctx)?;

        Ok((
            Prototype {
                source,
                line_defined,
                last_line_defined,
                number_of_upvalues,
                number_of_parameters,
                is_vararg,
                max_stack_size,
                code,
                constants,
                prototypes,
                debug_info,
            },
            *offset,
        ))
    }
}

impl BytecodeTrait for Bytecode<'_> {
    fn identifier() -> &'static str {
        "lua51"
    }

    fn display_name() -> &'static str {
        "Lua 5.1"
    }
}
