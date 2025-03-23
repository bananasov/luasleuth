pub mod instructions;

use luasleuth_common::types::LuaString;
use scroll::{ctx, Endian, Pread, Uleb128};

/// Flag to determine whether or not bytecode is stripped
pub const BYTECODE_IS_STRIPPED: u8 = 0x02;

#[derive(Debug)]
pub struct Header<'a> {
    pub signature: [u8; 3],
    pub version: u8,
    pub flags: Uleb128,

    // this will be empty if the bytecode is stripped
    pub chunk_name: LuaString<'a>,
}

#[derive(Debug)]
pub struct Bytecode<'a> {
    pub header: Header<'a>,
}

impl<'a> ctx::TryFromCtx<'a, Endian> for Header<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let signature: [u8; 3] = src.gread_with(offset, ctx)?;
        let version: u8 = src.gread_with(offset, ctx)?;
        let flags: Uleb128 = src.gread_with(offset, ())?;

        todo!()
    }
}
