use luasleuth_common::{mask, CommonCtx};
use scroll::{ctx, Pread, Pwrite};

pub mod constants {
    //! Constants related to Lua 5.1 instructions

    pub const SIZE_C: u8 = 9;
    pub const SIZE_B: u8 = 9;
    pub const SIZE_BX: u8 = SIZE_C + SIZE_B;
    pub const SIZE_A: u8 = 8;

    pub const SIZE_OP: u8 = 6;

    pub const POS_OP: u8 = 0;
    pub const POS_A: u8 = POS_OP + SIZE_OP;
    pub const POS_C: u8 = POS_A + SIZE_A;
    pub const POS_B: u8 = POS_C + SIZE_C;
    pub const POS_BX: u8 = POS_C;

    pub const MAXARG_BX: u32 = (1 << SIZE_BX) - 1;
    pub const MAXARG_S_BX: u32 = MAXARG_BX >> 1;
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    iABC(Opcode, u8, u16, u16),
    iABx(Opcode, u8, u32),
    iAsBx(Opcode, u8, i32),
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Opcode {
    OP_MOVE,
    OP_LOADK,
    OP_LOADBOOL,
    OP_LOADNIL,
    OP_GETUPVAL,
    OP_GETGLOBAL,
    OP_GETTABLE,
    OP_SETGLOBAL,
    OP_SETUPVAL,
    OP_SETTABLE,
    OP_NEWTABLE,
    OP_SELF,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    OP_MOD,
    OP_POW,
    OP_UNM,
    OP_NOT,
    OP_LEN,
    OP_CONCAT,
    OP_JMP,
    OP_EQ,
    OP_LT,
    OP_LE,
    OP_TEST,
    OP_TESTSET,
    OP_CALL,
    OP_TAILCALL,
    OP_RETURN,
    OP_FORLOOP,
    OP_FORPREP,
    OP_TFORLOOP,
    OP_SETLIST,
    OP_CLOSE,
    OP_CLOSURE,
    OP_VARARG,
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value as u8
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::OP_MOVE,
            1 => Opcode::OP_LOADK,
            2 => Opcode::OP_LOADBOOL,
            3 => Opcode::OP_LOADNIL,
            4 => Opcode::OP_GETUPVAL,
            5 => Opcode::OP_GETGLOBAL,
            6 => Opcode::OP_GETTABLE,
            7 => Opcode::OP_SETGLOBAL,
            8 => Opcode::OP_SETUPVAL,
            9 => Opcode::OP_SETTABLE,
            10 => Opcode::OP_NEWTABLE,
            11 => Opcode::OP_SELF,
            12 => Opcode::OP_ADD,
            13 => Opcode::OP_SUB,
            14 => Opcode::OP_MUL,
            15 => Opcode::OP_DIV,
            16 => Opcode::OP_MOD,
            17 => Opcode::OP_POW,
            18 => Opcode::OP_UNM,
            19 => Opcode::OP_NOT,
            20 => Opcode::OP_LEN,
            21 => Opcode::OP_CONCAT,
            22 => Opcode::OP_JMP,
            23 => Opcode::OP_EQ,
            24 => Opcode::OP_LT,
            25 => Opcode::OP_LE,
            26 => Opcode::OP_TEST,
            27 => Opcode::OP_TESTSET,
            28 => Opcode::OP_CALL,
            29 => Opcode::OP_TAILCALL,
            30 => Opcode::OP_RETURN,
            31 => Opcode::OP_FORLOOP,
            32 => Opcode::OP_FORPREP,
            33 => Opcode::OP_TFORLOOP,
            34 => Opcode::OP_SETLIST,
            35 => Opcode::OP_CLOSE,
            36 => Opcode::OP_CLOSURE,
            37 => Opcode::OP_VARARG,
            _ => unreachable!(),
        }
    }
}

impl Instruction {
    pub fn decode(inst: u32) -> Instruction {
        use constants::*;
        use Opcode::*;

        let opcode = (inst >> POS_OP) & mask!(SIZE_OP, 0);
        let a = ((inst >> POS_A) & mask!(SIZE_A, 0)) as u8;

        let opcode: Opcode = (opcode as u8).into();
        match opcode {
            OP_MOVE | OP_LOADBOOL | OP_LOADNIL | OP_GETUPVAL | OP_GETTABLE | OP_SETUPVAL
            | OP_SETTABLE | OP_NEWTABLE | OP_SELF | OP_ADD | OP_SUB | OP_MUL | OP_DIV | OP_MOD
            | OP_POW | OP_UNM | OP_NOT | OP_LEN | OP_CONCAT | OP_EQ | OP_LT | OP_LE | OP_TEST
            | OP_TESTSET | OP_CALL | OP_TAILCALL | OP_RETURN | OP_TFORLOOP | OP_SETLIST
            | OP_CLOSE | OP_VARARG => {
                let b = ((inst >> POS_B) & mask!(SIZE_B, 0)) as u16;
                let c = ((inst >> POS_C) & mask!(SIZE_C, 0)) as u16;
                Instruction::iABC(opcode, a, b, c)
            }
            OP_LOADK | OP_GETGLOBAL | OP_SETGLOBAL | OP_CLOSURE => {
                let bx = (inst >> POS_BX) & MAXARG_BX;
                Instruction::iABx(opcode, a, bx)
            }
            OP_JMP | OP_FORLOOP | OP_FORPREP => {
                let sbx = (((inst >> POS_BX) & MAXARG_BX) as i32) - (MAXARG_S_BX as i32);
                Instruction::iAsBx(opcode, a, sbx)
            }
        }
    }

    pub fn encode(instruction: Instruction) -> u32 {
        use constants::*;

        match instruction {
            Instruction::iABC(opcode, a, b, c) => {
                let opcode = opcode as u32;
                let a = (a as u32) << POS_A;
                let c = (c as u32) << POS_C;
                let b = (b as u32) << POS_B;

                opcode | a | c | b
            }
            Instruction::iABx(opcode, a, bx) => {
                let opcode = opcode as u32;
                let a = (a << POS_A) as u32;
                let bx = bx << POS_BX;

                opcode | a | bx
            }
            Instruction::iAsBx(opcode, a, sbx) => {
                let opcode = opcode as u32;
                let a = (a << POS_A) as u32;
                let sbx = ((sbx + (MAXARG_S_BX as i32)) << POS_BX) as u32;

                opcode | a | sbx
            }
        }
    }
}

impl<'a> ctx::TryFromCtx<'a, CommonCtx> for Instruction {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let instruction: u32 = src.gread_with(offset, ctx.endianness)?;
        let instruction = Instruction::decode(instruction);

        Ok((instruction, *offset))
    }
}

impl ctx::TryIntoCtx<CommonCtx> for Instruction {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: CommonCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        dst.gwrite_with(Instruction::encode(self), offset, ctx.endianness)?;

        Ok(*offset)
    }
}
