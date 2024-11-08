use scroll::{Pread, LE};

use crate::{
    common::string::{LuaString, LuaStringCtx},
    try_gread_vec_with,
};

use super::common::{
    self,
    constants::Constant,
    debug_info::{DebugInfo, LocalVariable},
    instructions::{Instruction, Opcode},
    upvalues::Upvalue,
    Bytecode, Prototype,
};

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
        let prototype = self.disassemble_prototype(header.size_of_sizet, offset)?;

        Ok(Bytecode { header, prototype })
    }

    fn disassemble_prototype(
        &self,
        size_of_sizet: u8,
        offset: &mut usize,
    ) -> Result<Prototype, scroll::Error> {
        let line_defined: i32 = self.buffer.gread_with(offset, LE)?;
        let last_line_defined: i32 = self.buffer.gread_with(offset, LE)?;
        let number_of_params: u8 = self.buffer.gread_with(offset, LE)?;
        let is_vararg: u8 = self.buffer.gread_with(offset, LE)?;
        let max_stack_size: u8 = self.buffer.gread_with(offset, LE)?;

        let instructions = self.disassemble_instructions(offset)?;
        let constants = self.disassemble_constants(size_of_sizet, offset)?;
        let prototypes = self.disassemble_prototypes(size_of_sizet, offset)?;
        let upvalues = self.disassemble_upvalues(offset)?;
        let debug_info = self.disassemble_debug_info(size_of_sizet, offset)?;

        Ok(Prototype {
            line_defined,
            last_line_defined,
            number_of_params,
            is_vararg: is_vararg != 0,
            max_stack_size,
            instructions,
            constants,
            prototypes,
            upvalues,
            debug_info,
        })
    }

    fn disassemble_instructions(
        &self,
        offset: &mut usize,
    ) -> Result<Vec<Instruction>, scroll::Error> {
        let amount: u32 = self.buffer.gread_with(offset, LE)?;
        let instruction_list: Vec<u32> = try_gread_vec_with!(self.buffer, offset, amount, LE);
        let instructions: Vec<Instruction> = instruction_list
            .iter()
            .map(|f| Opcode::decode(*f))
            .collect();

        Ok(instructions)
    }

    fn disassemble_constants(
        &self,
        size_of_sizet: u8,
        offset: &mut usize,
    ) -> Result<Vec<Constant>, scroll::Error> {
        let constant_amount: u32 = self.buffer.gread_with(offset, LE)?;
        let mut constants: Vec<Constant> = Vec::new();
        for _ in 0..constant_amount {
            let constant = Constant::decode(self.buffer, offset, size_of_sizet, LE)?;
            constants.push(constant);
        }

        Ok(constants)
    }

    fn disassemble_prototypes(
        &self,
        size_of_sizet: u8,
        offset: &mut usize,
    ) -> Result<Vec<Prototype>, scroll::Error> {
        let prototype_amount: u32 = self.buffer.gread_with(offset, LE)?;
        let mut prototypes: Vec<Prototype> = Vec::new();
        for _ in 0..prototype_amount {
            let prototype = self.disassemble_prototype(size_of_sizet, offset)?;
            prototypes.push(prototype);
        }

        Ok(prototypes)
    }

    fn disassemble_debug_info(
        &self,
        size_of_sizet: u8,
        offset: &mut usize,
    ) -> Result<DebugInfo, scroll::Error> {
        let source: LuaString = self.buffer.gread_with(
            offset,
            LuaStringCtx {
                endianess: LE,
                size_of_sizet,
            },
        )?;

        let amount: u32 = self.buffer.gread_with(offset, LE)?;
        let line_info: Vec<i32> = try_gread_vec_with!(self.buffer, offset, amount, LE);

        let local_variables = self.disassemble_local_variables(size_of_sizet, offset)?;
        let upvalues = self.disassemble_debug_upvalues(size_of_sizet, offset)?;

        Ok(DebugInfo {
            source: source.into_string(),
            line_info,
            local_variables,
            upvalues,
        })
    }

    fn disassemble_local_variables(
        &self,
        size_of_sizet: u8,
        offset: &mut usize,
    ) -> Result<Vec<LocalVariable>, scroll::Error> {
        let amount: u32 = self.buffer.gread_with(offset, LE)?;
        let mut variables: Vec<LocalVariable> = Vec::new();
        for _ in 0..amount {
            let local = self.disassemble_local_variable(size_of_sizet, offset)?;
            variables.push(local);
        }

        Ok(variables)
    }

    fn disassemble_local_variable(
        &self,
        size_of_sizet: u8,
        offset: &mut usize,
    ) -> Result<LocalVariable, scroll::Error> {
        let name: LuaString = self.buffer.gread_with(
            offset,
            LuaStringCtx {
                endianess: LE,
                size_of_sizet,
            },
        )?;
        let start: i32 = self.buffer.gread_with(offset, LE)?;
        let end: i32 = self.buffer.gread_with(offset, LE)?;

        Ok(LocalVariable {
            name: name.into_string(),
            start,
            end,
        })
    }

    fn disassemble_upvalues(&self, offset: &mut usize) -> Result<Vec<Upvalue>, scroll::Error> {
        let n: i32 = self.buffer.gread_with(offset, LE)?;
        let mut upvalues = Vec::new();

        for _ in 0..n {
            let upvalue = self.disassemble_upvalue(offset)?;
            upvalues.push(upvalue);
        }

        Ok(upvalues)
    }

    fn disassemble_upvalue(&self, offset: &mut usize) -> Result<Upvalue, scroll::Error> {
        let in_stack: u8 = self.buffer.gread_with(offset, LE)?;
        let index: u8 = self.buffer.gread_with(offset, LE)?;

        Ok(Upvalue { in_stack, index })
    }

    fn disassemble_debug_upvalues(
        &self,
        size_of_sizet: u8,
        offset: &mut usize,
    ) -> Result<Vec<String>, scroll::Error> {
        let amount: u32 = self.buffer.gread_with(offset, LE)?;
        let mut upvalues: Vec<String> = Vec::new();
        for _ in 0..amount {
            let local: LuaString = self.buffer.gread_with(
                offset,
                LuaStringCtx {
                    endianess: LE,
                    size_of_sizet,
                },
            )?;
            upvalues.push(local.into_string());
        }

        Ok(upvalues)
    }
}
