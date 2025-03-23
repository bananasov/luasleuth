pub mod debug_info;
pub mod instructions;

use luasleuth_common::{
    try_gread_vec_with,
    types::{leb128::Uleb128, Bytecode as BytecodeTrait},
};
use scroll::{ctx, Endian, Pread};

use crate::common::{ctx::BytecodeContext, jitstring::JitString};

#[derive(Debug)]
pub struct Header<'a> {
    pub signature: [u8; 3],
    pub version: u8,
    pub flags: Uleb128,
    pub chunk_name: Option<JitString<'a>>,
}

#[derive(Debug)]
pub struct Prototype {
    /// Total size of the prototype
    pub prototype_length: Uleb128,

    /// Metadata for the prototype
    pub flags: u8,

    /// Number of parameters
    pub parameter_count: u8,

    /// Fixed frame size
    pub frame_size: u8,

    pub upvalue_count: u8,
    pub gc_constant_count: Uleb128,
    pub num_constant_count: Uleb128,
    pub instruction_count: Uleb128,

    pub debug_metadata: Option<debug_info::DebugInfoMetadata>,
    pub instructions: Vec<instructions::Instruction>,
}

#[derive(Debug)]
pub struct Bytecode<'a> {
    pub header: Header<'a>,
    pub prototype: Prototype,
}

impl<'a> ctx::TryFromCtx<'a, Endian> for Header<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let signature: [u8; 3] = src.gread_with(offset, ctx)?;
        let version: u8 = src.gread_with(offset, ctx)?;
        let flags: Uleb128 = src.gread_with(offset, ())?;

        let context = BytecodeContext {
            flags: flags.into(),
            endian: ctx,
        };

        let chunk_name: Option<JitString> = if !context.is_stripped() {
            Some(src.gread_with(offset, ())?)
        } else {
            None
        };

        Ok((
            Self {
                signature,
                version,
                flags,
                chunk_name,
            },
            *offset,
        ))
    }
}

impl<'a> ctx::TryFromCtx<'a, BytecodeContext> for Prototype {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: BytecodeContext) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let prototype_length: Uleb128 = src.gread_with(offset, ())?;
        let flags: u8 = src.gread_with(offset, ctx.endian)?;
        let parameter_count: u8 = src.gread_with(offset, ctx.endian)?;
        let frame_size: u8 = src.gread_with(offset, ctx.endian)?;

        let upvalue_count: u8 = src.gread_with(offset, ctx.endian)?;
        let gc_constant_count: Uleb128 = src.gread_with(offset, ())?;
        let num_constant_count: Uleb128 = src.gread_with(offset, ())?;

        let instruction_count: Uleb128 = src.gread_with(offset, ())?;
        let instruction_count: usize = instruction_count.into();

        let debug_metadata: Option<debug_info::DebugInfoMetadata> = if !ctx.is_stripped() {
            Some(src.gread_with(offset, ctx)?)
        } else {
            None
        };

        let instructions: Vec<instructions::Instruction> =
            try_gread_vec_with!(src, offset, instruction_count, ctx);

        Ok((
            Self {
                prototype_length,
                flags,
                parameter_count,
                frame_size,
                upvalue_count,
                gc_constant_count,
                num_constant_count,
                instruction_count: instruction_count.into(),
                debug_metadata,
                instructions,
            },
            *offset,
        ))
    }
}

impl BytecodeTrait for Bytecode<'_> {
    fn identifier() -> &'static str {
        "luajit-v2"
    }

    fn display_name() -> &'static str {
        "LuaJIT v2"
    }
}
