pub mod debug_info;
pub mod instructions;

use luasleuth_common::types::leb128::Uleb128;
use scroll::{ctx, Endian, Pread};

use crate::common::{ctx::BytecodeContext, jitstring::JitString};

#[derive(Debug)]
pub struct Header<'a> {
    pub signature: [u8; 3],
    pub version: u8,
    pub flags: Uleb128,

    // this will be empty if the bytecode is stripped
    pub chunk_name: JitString<'a>,
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
    // pub upvalue_count: Uleb128,
    // pub gc_constant_count: Uleb128,
    // pub numeric_constant_count: Uleb128,
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

        let chunk_name: JitString = src.gread_with(offset, context.is_stripped())?;

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
        let numeric_constant_count: Uleb128 = src.gread_with(offset, ())?;
        let bytecode_instruction_count: Uleb128 = src.gread_with(offset, ())?;

        todo!()
    }
}
