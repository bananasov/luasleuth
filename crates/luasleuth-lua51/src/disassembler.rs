use crate::types::{Bytecode, Header, Prototype};
use luasleuth_common::{disassembler::Disassemble, CommonCtx};
use scroll::Pread;

pub struct Disassembler<'a> {
    bytes: &'a [u8],
}

impl<'a> Disassembler<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl<'a> Disassemble<Bytecode<'a>> for Disassembler<'a> {
    type Error = scroll::Error;

    fn disassemble(self) -> Result<Bytecode<'a>, Self::Error> {
        let offset = &mut 0;

        let header: Header = self.bytes.gread_with(offset, scroll::LE)?;
        let ctx = CommonCtx {
            size_of_size_t: header.size_of_size_t,
            lua_version: header.version,
            endianness: scroll::LE,
        };
        let prototype: Prototype = self.bytes.gread_with(offset, ctx)?;

        Ok(Bytecode { header, prototype })
    }
}
