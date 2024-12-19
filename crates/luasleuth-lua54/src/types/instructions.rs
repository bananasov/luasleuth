use luasleuth_common::{mask, types::Packable, CommonCtx};
use scroll::{ctx, Pread, Pwrite};

pub mod constants {
    //! Constants related to Lua 5.4 instructions

    pub const SIZE_C: u8 = 8;
    pub const SIZE_B: u8 = 8;
    pub const SIZE_BX: u8 = SIZE_C + SIZE_B + 1;
    pub const SIZE_A: u8 = 8;
    pub const SIZE_AX: u8 = SIZE_BX + SIZE_A;
    pub const SIZE_S_J: u8 = SIZE_BX + SIZE_A;

    pub const SIZE_OP: u8 = 7;
    pub const POS_OP: u8 = 0;

    pub const POS_A: u8 = POS_OP + SIZE_OP;
    pub const POS_K: u8 = POS_A + SIZE_A;
    pub const POS_B: u8 = POS_K + 1;
    pub const POS_C: u8 = POS_B + SIZE_B;
    pub const POS_BX: u8 = POS_K;
    pub const POS_AX: u8 = POS_A;
    pub const POS_S_J: u8 = POS_A;

    pub const MAXARG_BX: u32 = (1 << SIZE_BX) - 1;
    pub const OFFSET_SBX: u32 = MAXARG_BX >> 1;
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    iABC(Opcode, u8, u8, u8, u8), // Op, A, B, C, K
    iABx(Opcode, u8, u32),
    iAsBx(Opcode, u8, i32),
    iAx(Opcode, u32),
    isJ(Opcode, i32),
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Opcode {
    OP_MOVE,
    OP_LOADI,
    OP_LOADF,
    OP_LOADK,
    OP_LOADKX,
    OP_LOADFALSE,
    OP_LFALSESKIP,
    OP_LOADTRUE,
    OP_LOADNIL,
    OP_GETUPVAL,
    OP_SETUPVAL,
    OP_GETTABUP,
    OP_GETTABLE,
    OP_GETI,
    OP_GETFIELD,
    OP_SETTABUP,
    OP_SETTABLE,
    OP_SETI,
    OP_SETFIELD,
    OP_NEWTABLE,
    OP_SELF,
    OP_ADDI,
    OP_ADDK,
    OP_SUBK,
    OP_MULK,
    OP_MODK,
    OP_POWK,
    OP_DIVK,
    OP_IDIVK,
    OP_BANDK,
    OP_BORK,
    OP_BXORK,
    OP_SHRI,
    OP_SHLI,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_MOD,
    OP_POW,
    OP_DIV,
    OP_IDIV,
    OP_BAND,
    OP_BOR,
    OP_BXOR,
    OP_SHL,
    OP_SHR,
    OP_MMBIN,
    OP_MMBINI,
    OP_MMBINK,
    OP_UNM,
    OP_BNOT,
    OP_NOT,
    OP_LEN,
    OP_CONCAT,
    OP_CLOSE,
    OP_TBC,
    OP_JMP,
    OP_EQ,
    OP_LT,
    OP_LE,
    OP_EQK,
    OP_EQI,
    OP_LTI,
    OP_LEI,
    OP_GTI,
    OP_GEI,
    OP_TEST,
    OP_TESTSET,
    OP_CALL,
    OP_TAILCALL,
    OP_RETURN,
    OP_RETURN0,
    OP_RETURN1,
    OP_FORLOOP,
    OP_FORPREP,
    OP_TFORPREP,
    OP_TFORCALL,
    OP_TFORLOOP,
    OP_SETLIST,
    OP_CLOSURE,
    OP_VARARG,
    OP_VARARGPREP,
    OP_EXTRAARG,
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
            1 => Opcode::OP_LOADI,
            2 => Opcode::OP_LOADF,
            3 => Opcode::OP_LOADK,
            4 => Opcode::OP_LOADKX,
            5 => Opcode::OP_LOADFALSE,
            6 => Opcode::OP_LFALSESKIP,
            7 => Opcode::OP_LOADTRUE,
            8 => Opcode::OP_LOADNIL,
            9 => Opcode::OP_GETUPVAL,
            10 => Opcode::OP_SETUPVAL,
            11 => Opcode::OP_GETTABUP,
            12 => Opcode::OP_GETTABLE,
            13 => Opcode::OP_GETI,
            14 => Opcode::OP_GETFIELD,
            15 => Opcode::OP_SETTABUP,
            16 => Opcode::OP_SETTABLE,
            17 => Opcode::OP_SETI,
            18 => Opcode::OP_SETFIELD,
            19 => Opcode::OP_NEWTABLE,
            20 => Opcode::OP_SELF,
            21 => Opcode::OP_ADDI,
            22 => Opcode::OP_ADDK,
            23 => Opcode::OP_SUBK,
            24 => Opcode::OP_MULK,
            25 => Opcode::OP_MODK,
            26 => Opcode::OP_POWK,
            27 => Opcode::OP_DIVK,
            28 => Opcode::OP_IDIVK,
            29 => Opcode::OP_BANDK,
            30 => Opcode::OP_BORK,
            31 => Opcode::OP_BXORK,
            32 => Opcode::OP_SHRI,
            33 => Opcode::OP_SHLI,
            34 => Opcode::OP_ADD,
            35 => Opcode::OP_SUB,
            36 => Opcode::OP_MUL,
            37 => Opcode::OP_MOD,
            38 => Opcode::OP_POW,
            39 => Opcode::OP_DIV,
            40 => Opcode::OP_IDIV,
            41 => Opcode::OP_BAND,
            42 => Opcode::OP_BOR,
            43 => Opcode::OP_BXOR,
            44 => Opcode::OP_SHL,
            45 => Opcode::OP_SHR,
            46 => Opcode::OP_MMBIN,
            47 => Opcode::OP_MMBINI,
            48 => Opcode::OP_MMBINK,
            49 => Opcode::OP_UNM,
            50 => Opcode::OP_BNOT,
            51 => Opcode::OP_NOT,
            52 => Opcode::OP_LEN,
            53 => Opcode::OP_CONCAT,
            54 => Opcode::OP_CLOSE,
            55 => Opcode::OP_TBC,
            56 => Opcode::OP_JMP,
            57 => Opcode::OP_EQ,
            58 => Opcode::OP_LT,
            59 => Opcode::OP_LE,
            60 => Opcode::OP_EQK,
            61 => Opcode::OP_EQI,
            62 => Opcode::OP_LTI,
            63 => Opcode::OP_LEI,
            64 => Opcode::OP_GTI,
            65 => Opcode::OP_GEI,
            66 => Opcode::OP_TEST,
            67 => Opcode::OP_TESTSET,
            68 => Opcode::OP_CALL,
            69 => Opcode::OP_TAILCALL,
            70 => Opcode::OP_RETURN,
            71 => Opcode::OP_RETURN0,
            72 => Opcode::OP_RETURN1,
            73 => Opcode::OP_FORLOOP,
            74 => Opcode::OP_FORPREP,
            75 => Opcode::OP_TFORPREP,
            76 => Opcode::OP_TFORCALL,
            77 => Opcode::OP_TFORLOOP,
            78 => Opcode::OP_SETLIST,
            79 => Opcode::OP_CLOSURE,
            80 => Opcode::OP_VARARG,
            81 => Opcode::OP_VARARGPREP,
            82 => Opcode::OP_EXTRAARG,
            _ => unimplemented!("Opcode::from({})", value),
        }
    }
}

impl Packable for Instruction {
    fn decode(raw: u32) -> Instruction {
        use constants::*;
        use Opcode::*;

        let opcode: Opcode = ((raw & mask!(SIZE_OP, 0)) as u8).into();
        let a = ((raw >> POS_A) & mask!(SIZE_A, 0)) as u8;
        let k = ((raw >> POS_K) & 0x1) as u8;

        match opcode {
            // iABC instructions
            OP_MOVE | OP_LOADNIL | OP_GETUPVAL | OP_SETUPVAL | OP_GETTABUP | OP_GETTABLE
            | OP_GETI | OP_GETFIELD | OP_SETTABUP | OP_SETTABLE | OP_SETI | OP_SETFIELD
            | OP_NEWTABLE | OP_SELF | OP_ADDK | OP_SUBK | OP_MULK | OP_MODK | OP_POWK | OP_DIVK
            | OP_IDIVK | OP_BANDK | OP_BORK | OP_BXORK | OP_ADD | OP_SUB | OP_MUL | OP_MOD
            | OP_POW | OP_DIV | OP_IDIV | OP_BAND | OP_BOR | OP_BXOR | OP_SHL | OP_SHR
            | OP_MMBIN | OP_MMBINI | OP_MMBINK | OP_UNM | OP_BNOT | OP_NOT | OP_LEN | OP_CONCAT
            | OP_CLOSE | OP_TBC | OP_TEST | OP_TESTSET | OP_CALL | OP_TAILCALL | OP_RETURN
            | OP_RETURN0 | OP_RETURN1 | OP_TFORCALL | OP_SETLIST | OP_VARARG | OP_VARARGPREP
            | OP_EQ | OP_LT | OP_LE | OP_EQK | OP_EQI | OP_LTI | OP_LEI | OP_GTI | OP_GEI
            | OP_ADDI | OP_SHRI | OP_SHLI => {
                let b = ((raw >> POS_B) & mask!(SIZE_B, 0)) as u8;
                let c = ((raw >> POS_C) & mask!(SIZE_C, 0)) as u8;
                Instruction::iABC(opcode, a, b, c, k)
            }

            // iABx instructions
            OP_LOADK | OP_LOADKX | OP_CLOSURE => {
                let bx = (raw >> POS_BX) & mask!(SIZE_BX, 0);
                Instruction::iABx(opcode, a, bx)
            }

            // iAsBx instructions
            OP_LOADI | OP_LOADF | OP_FORLOOP | OP_FORPREP | OP_TFORPREP | OP_TFORLOOP => {
                let sbx = (((raw >> POS_BX) & mask!(SIZE_BX, 0)) as i32) - (OFFSET_SBX as i32);
                Instruction::iAsBx(opcode, a, sbx)
            }

            // iAx instructions
            OP_EXTRAARG => {
                let ax = (raw >> POS_AX) & mask!(SIZE_AX, 0);
                Instruction::iAx(opcode, ax)
            }

            // isJ instructions
            OP_JMP => {
                let sj = (((raw >> POS_S_J) & mask!(SIZE_S_J, 0)) as i32) - (OFFSET_SBX as i32);
                Instruction::isJ(opcode, sj)
            }

            // Simple instructions (no additional arguments needed)
            OP_LOADFALSE | OP_LFALSESKIP | OP_LOADTRUE => Instruction::iABC(opcode, a, 0, 0, 0),
        }
    }

    fn encode(inst: Instruction) -> u32 {
        use constants::*;

        match inst {
            Instruction::iABC(opcode, a, b, c, k) => {
                let opcode = opcode as u32;
                let a = (a as u32) << POS_A;
                let k = (k as u32) << POS_K;
                let b = (b as u32) << POS_B;
                let c = (c as u32) << POS_C;

                opcode | a | k | b | c
            }
            Instruction::iABx(opcode, a, bx) => {
                let opcode = opcode as u32;
                let a = (a as u32) << POS_A;
                let bx = bx << POS_BX;
                opcode | a | bx
            }
            Instruction::iAsBx(opcode, a, sbx) => {
                let opcode = opcode as u32;
                let a = (a as u32) << POS_A;
                let sbx = ((sbx + OFFSET_SBX as i32) as u32) << POS_BX;
                opcode | a | sbx
            }
            Instruction::iAx(opcode, ax) => {
                let opcode = opcode as u32;
                let ax = ax << POS_AX;
                opcode | ax
            }
            Instruction::isJ(opcode, sj) => {
                let opcode = opcode as u32;
                let sj = ((sj + OFFSET_SBX as i32) as u32) << POS_S_J;
                opcode | sj
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

    fn try_into_ctx(self, src: &mut [u8], ctx: CommonCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        let n = Instruction::encode(self);
        src.gwrite_with(n, offset, ctx.endianness)?;

        Ok(*offset)
    }
}
