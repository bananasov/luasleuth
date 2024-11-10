use scroll::{Pread, LE};

use crate::try_gread_vec_with;

use super::common::{
    self,
    constants::Constant,
    debug_info::{DebugInfo, LocalVariable},
    instructions::Instruction,
    Bytecode, Prototype,
};
use crate::common::string::{LuaString, LuaStringCtx};
use crate::common::Array;

#[derive(Debug)]
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
        let prototype = self.disassemble_prototype(header.size_of_size_t, offset)?;

        Ok(Bytecode { header, prototype })
    }

    fn disassemble_prototype(
        &self,
        size_of_size_t: u8,
        offset: &mut usize,
    ) -> Result<Prototype, scroll::Error> {
        let name: LuaString = self
            .buffer
            .gread_with(offset, LuaStringCtx::new_le(size_of_size_t))?;
        let line_defined: u32 = self.buffer.gread_with(offset, LE)?;
        let last_line_defined: u32 = self.buffer.gread_with(offset, LE)?;
        let number_of_upvalues: u8 = self.buffer.gread_with(offset, LE)?;
        let number_of_params: u8 = self.buffer.gread_with(offset, LE)?;
        let is_vararg: u8 = self.buffer.gread_with(offset, LE)?;
        let max_stack_size: u8 = self.buffer.gread_with(offset, LE)?;

        let instructions = self.disassemble_instructions(offset)?;
        let constants = self.disassemble_constants(size_of_size_t, offset)?;
        let prototypes = self.disassemble_prototypes(size_of_size_t, offset)?;

        let debug_info = self.disassemble_debug_info(size_of_size_t, offset)?;

        Ok(Prototype {
            name: name.into_string(),
            line_defined,
            last_line_defined,
            number_of_upvalues,
            number_of_params,
            is_vararg: is_vararg != 0,
            max_stack_size,
            instructions,
            constants,
            prototypes,
            debug_info,
        })
    }

    fn disassemble_instructions(
        &self,
        offset: &mut usize,
    ) -> Result<Array<Instruction>, scroll::Error> {
        let instructions: Array<Instruction> = self.buffer.gread_with(offset, LE)?;

        Ok(instructions)
    }

    fn disassemble_constants(
        &self,
        size_of_size_t: u8,
        offset: &mut usize,
    ) -> Result<Array<Constant>, scroll::Error> {
        let ctx = LuaStringCtx::new_le(size_of_size_t);
        let constants: Array<Constant> = self.buffer.gread_with(offset, ctx)?;

        Ok(constants)
    }

    fn disassemble_prototypes(
        &self,
        size_of_size_t: u8,
        offset: &mut usize,
    ) -> Result<Array<Prototype>, scroll::Error> {
        let prototype_amount: u32 = self.buffer.gread_with(offset, LE)?;
        let mut prototypes: Vec<Prototype> = Vec::new();
        for _ in 0..prototype_amount {
            let prototype = self.disassemble_prototype(size_of_size_t, offset)?;
            prototypes.push(prototype);
        }

        Ok(prototypes.into())
    }

    fn disassemble_debug_info(
        &self,
        size_of_size_t: u8,
        offset: &mut usize,
    ) -> Result<DebugInfo, scroll::Error> {
        let amount: u32 = self.buffer.gread_with(offset, LE)?;
        let line_info: Vec<i32> = try_gread_vec_with!(self.buffer, offset, amount, LE);

        let local_variables = self.disassemble_local_variables(size_of_size_t, offset)?;
        let upvalues = self.disassemble_upvalues(size_of_size_t, offset)?;

        Ok(DebugInfo {
            line_info,
            local_variables,
            upvalues,
        })
    }

    fn disassemble_local_variables(
        &self,
        size_of_size_t: u8,
        offset: &mut usize,
    ) -> Result<Vec<LocalVariable>, scroll::Error> {
        let amount: u32 = self.buffer.gread_with(offset, LE)?;
        let mut variables: Vec<LocalVariable> = Vec::new();
        for _ in 0..amount {
            let local = self.disassemble_local_variable(size_of_size_t, offset)?;
            variables.push(local);
        }

        Ok(variables)
    }

    fn disassemble_local_variable(
        &self,
        size_of_size_t: u8,
        offset: &mut usize,
    ) -> Result<LocalVariable, scroll::Error> {
        let name: LuaString = self
            .buffer
            .gread_with(offset, LuaStringCtx::new_le(size_of_size_t))?;
        let start: i32 = self.buffer.gread_with(offset, LE)?;
        let end: i32 = self.buffer.gread_with(offset, LE)?;

        Ok(LocalVariable {
            name: name.into_string(),
            start,
            end,
        })
    }

    fn disassemble_upvalues(
        &self,
        size_ofsize_of_size_tsizet: u8,
        offset: &mut usize,
    ) -> Result<Vec<String>, scroll::Error> {
        let amount: u32 = self.buffer.gread_with(offset, LE)?;
        let mut upvalues: Vec<String> = Vec::new();
        for _ in 0..amount {
            let local: LuaString = self
                .buffer
                .gread_with(offset, LuaStringCtx::new_le(size_ofsize_of_size_tsizet))?;
            upvalues.push(local.into_string());
        }

        Ok(upvalues)
    }
}
