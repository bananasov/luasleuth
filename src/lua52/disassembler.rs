use scroll::{Pread, LE};

use super::common::{
    self,
    constants::Constant,
    debug_info::{DebugInfo, LocalVariable},
    instructions::{Instruction, Opcode},
    string::LuaString,
    Bytecode, Prototype,
};

pub struct Disassembler<'a> {
    buffer: &'a [u8],
}

impl<'a> Disassembler<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { buffer: bytes }
    }

    pub fn disassemble(&self) -> Result<Bytecode, scroll::Error> {
        let offset = &mut 0;
        let header: common::Header = self.buffer.gread_with(offset, LE)?;
        let prototype = self.disassemble_prototype(offset)?;

        Ok(Bytecode {
            header,
            prototype,
        })
    }

    fn disassemble_prototype(&self, offset: &mut usize) -> Result<Prototype, scroll::Error> {
        todo!()
    }

    fn disassemble_instructions(
        &self,
        offset: &mut usize,
    ) -> Result<Vec<Instruction>, scroll::Error> {
        todo!()
    }

    fn disassemble_constants(&self, offset: &mut usize) -> Result<Vec<Constant>, scroll::Error> {
        todo!()
    }

    fn disassemble_prototypes(&self, offset: &mut usize) -> Result<Vec<Prototype>, scroll::Error> {
        todo!()
    }

    fn disassemble_debug_info(&self, offset: &mut usize) -> Result<DebugInfo, scroll::Error> {
        todo!()
    }

    fn disassemble_local_variables(
        &self,
        offset: &mut usize,
    ) -> Result<Vec<LocalVariable>, scroll::Error> {
        todo!()
    }

    fn disassemble_local_variable(
        &self,
        offset: &mut usize,
    ) -> Result<LocalVariable, scroll::Error> {
        todo!()
    }

    fn disassemble_upvalues(&self, offset: &mut usize) -> Result<Vec<String>, scroll::Error> {
        todo!()
    }
}
