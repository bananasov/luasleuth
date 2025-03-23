use crate::{
    common::ctx::BytecodeContext,
    v2::types::{Bytecode, Header, Prototype},
};
use luasleuth_common::disassembler::Disassemble;
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
        println!("{:#?}", header);
        let ctx = BytecodeContext {
            flags: header.flags.into(),
            endian: scroll::LE, // cursed but alright, fix this laterrr
        };
        let prototype: Prototype = self.bytes.gread_with(offset, ctx)?;
        println!("{:#?}", prototype);

        Ok(Bytecode { header, prototype })
    }
}
