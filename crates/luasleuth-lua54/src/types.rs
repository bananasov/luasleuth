pub mod constants;
pub mod debug_info;
pub mod instructions;
pub mod upvalues;

use luasleuth_common::{
    types::{Array, LuaString, LuaUnsigned},
    CommonCtx, Version,
};
use scroll::{ctx, Pread};

#[derive(Debug, Pread)]
pub struct Header {
    pub signature: [u8; 4],
    pub version: Version,
    pub format_version: u8,
    pub error_correction_data: [u8; 6],
    pub size_of_instruction: u8,
    pub size_of_integer: u8,
    pub size_of_number: u8,
    pub luac_int: u64,
    pub luac_num: f64,
}

#[derive(Debug)]
pub struct Prototype<'a> {
    pub source: LuaString<'a>,
    pub line_defined: LuaUnsigned,
    pub last_line_defined: LuaUnsigned,
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
    pub prototype: Prototype<'a>,
}

impl<'a> ctx::TryFromCtx<'a, CommonCtx> for Prototype<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let source: LuaString = src.gread_with(offset, ctx)?;
        println!("name: {source}, {}", source.size);
        let line_defined: LuaUnsigned = src.gread_with(offset, ctx.endianness)?;
        let last_line_defined: LuaUnsigned = src.gread_with(offset, ctx.endianness)?;
        let number_of_parameters: u8 = src.gread_with(offset, ctx.endianness)?;
        let is_vararg: u8 = src.gread_with(offset, ctx.endianness)?;
        let max_stack_size: u8 = src.gread_with(offset, ctx.endianness)?;

        println!("before inst");
        let instructions: Array<instructions::Instruction> = src.gread_with(offset, ctx)?;
        println!("after inst");
        let constants: Array<constants::Constant> = src.gread_with(offset, ctx)?;
        println!("after constants");
        let upvalues: Array<upvalues::Upvalue> = src.gread_with(offset, ctx)?;
        println!("after upvalues");
        let prototypes: Array<Prototype> = src.gread_with(offset, ctx)?;
        println!("after prototypes");
        let debug_info: debug_info::DebugInfo = src.gread_with(offset, ctx)?;
        println!("after debug info");

        Ok((
            Self {
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
