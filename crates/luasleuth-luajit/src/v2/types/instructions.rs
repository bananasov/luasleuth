use luasleuth_common::{mask, types::Packable};
use scroll::{ctx, Pread, Pwrite};

use crate::common::ctx::BytecodeContext;

pub mod constants {
    //! Constants related to LuaJIT v2 bytecode format

    // Bytecode instruction format fields width in bits
    pub const SIZE_OP: u8 = 8;
    pub const SIZE_A: u8 = 8;
    pub const SIZE_B: u8 = 8;
    pub const SIZE_C: u8 = 8;
    pub const SIZE_D: u8 = 16;

    // Bytecode instruction format fields position
    pub const POS_OP: u8 = 0;
    pub const POS_A: u8 = 8;
    pub const POS_C: u8 = 16;
    pub const POS_B: u8 = 24;
    pub const POS_D: u8 = 16;

    // Biased jump offset constant
    pub const BCBIAS_J: i32 = 0x8000;

    // Special register values
    pub const NO_REG: u8 = 0xff;

    /// Lookup table for bytecode operation modes
    /// This maps each operation to its mode (what kind of operands it takes)
    pub static BCMODE_TABLE: &[u16] = &[
        0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001,
        0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001,
        0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001,
        0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001,
        0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001,
        0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001,
        0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001,
        0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001,
        0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001,
    ];
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    /// Format ABC: OP A B C
    ABC(Opcode, u8, u8, u8),
    /// Format AD: OP A D
    AD(Opcode, u8, u16),
    /// Format AJ: OP A J (J is signed displacement)
    AJ(Opcode, u8, i32),
}

/// LuaJIT bytecode operations
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Opcode {
    // Comparison ops
    ISLT,
    ISGE,
    ISLE,
    ISGT,
    ISEQV,
    ISNEV,
    ISEQS,
    ISNES,
    ISEQN,
    ISNEN,
    ISEQP,
    ISNEP,

    // Unary test and copy ops
    ISTC,
    ISFC,
    IST,
    ISF,
    ISTYPE,
    ISNUM,

    // Unary ops
    MOV,
    NOT,
    UNM,
    LEN,

    // Binary ops
    ADDVN,
    SUBVN,
    MULVN,
    DIVVN,
    MODVN,
    ADDNV,
    SUBNV,
    MULNV,
    DIVNV,
    MODNV,
    ADDVV,
    SUBVV,
    MULVV,
    DIVVV,
    MODVV,
    POW,
    CAT,

    // Constant ops
    KSTR,
    KCDATA,
    KSHORT,
    KNUM,
    KPRI,
    KNIL,

    // Upvalue and function ops
    UGET,
    USETV,
    USETS,
    USETN,
    USETP,
    UCLO,
    FNEW,

    // Table ops
    TNEW,
    TDUP,
    GGET,
    GSET,
    TGETV,
    TGETS,
    TGETB,
    TGETR,
    TSETV,
    TSETS,
    TSETB,
    TSETM,
    TSETR,

    // Calls and vararg handling
    CALLM,
    CALL,
    CALLMT,
    CALLT,
    ITERC,
    ITERN,
    VARG,
    ISNEXT,

    // Returns
    RETM,
    RET,
    RET0,
    RET1,

    // Loops and branches
    FORI,
    JFORI,
    FORL,
    IFORL,
    JFORL,
    ITERL,
    IITERL,
    JITERL,
    LOOP,
    ILOOP,
    JLOOP,
    JMP,

    // Function headers
    FUNCF,
    IFUNCF,
    JFUNCF,
    FUNCV,
    IFUNCV,
    JFUNCV,
    FUNCC,
    FUNCCW,
}

impl Opcode {
    pub fn uses_ad_format(self) -> bool {
        match self {
            Self::CALL | Self::CALLM | Self::CALLMT | Self::CALLT | Self::ITERC | Self::ITERN => {
                false
            }
            _ => (constants::BCMODE_TABLE[self as usize] & 0x78) == 0,
        }
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ISLT,
            1 => Self::ISGE,
            2 => Self::ISLE,
            3 => Self::ISGT,
            4 => Self::ISEQV,
            5 => Self::ISNEV,
            6 => Self::ISEQS,
            7 => Self::ISNES,
            8 => Self::ISEQN,
            9 => Self::ISNEN,
            10 => Self::ISEQP,
            11 => Self::ISNEP,
            12 => Self::ISTC,
            13 => Self::ISFC,
            14 => Self::IST,
            15 => Self::ISF,
            16 => Self::ISTYPE,
            17 => Self::ISNUM,
            18 => Self::MOV,
            19 => Self::NOT,
            20 => Self::UNM,
            21 => Self::LEN,
            22 => Self::ADDVN,
            23 => Self::SUBVN,
            24 => Self::MULVN,
            25 => Self::DIVVN,
            26 => Self::MODVN,
            27 => Self::ADDNV,
            28 => Self::SUBNV,
            29 => Self::MULNV,
            30 => Self::DIVNV,
            31 => Self::MODNV,
            32 => Self::ADDVV,
            33 => Self::SUBVV,
            34 => Self::MULVV,
            35 => Self::DIVVV,
            36 => Self::MODVV,
            37 => Self::POW,
            38 => Self::CAT,
            39 => Self::KSTR,
            40 => Self::KCDATA,
            41 => Self::KSHORT,
            42 => Self::KNUM,
            43 => Self::KPRI,
            44 => Self::KNIL,
            45 => Self::UGET,
            46 => Self::USETV,
            47 => Self::USETS,
            48 => Self::USETN,
            49 => Self::USETP,
            50 => Self::UCLO,
            51 => Self::FNEW,
            52 => Self::TNEW,
            53 => Self::TDUP,
            54 => Self::GGET,
            55 => Self::GSET,
            56 => Self::TGETV,
            57 => Self::TGETS,
            58 => Self::TGETB,
            59 => Self::TGETR,
            60 => Self::TSETV,
            61 => Self::TSETS,
            62 => Self::TSETB,
            63 => Self::TSETM,
            64 => Self::TSETR,
            65 => Self::CALLM,
            66 => Self::CALL,
            67 => Self::CALLMT,
            68 => Self::CALLT,
            69 => Self::ITERC,
            70 => Self::ITERN,
            71 => Self::VARG,
            72 => Self::ISNEXT,
            73 => Self::RETM,
            74 => Self::RET,
            75 => Self::RET0,
            76 => Self::RET1,
            77 => Self::FORI,
            78 => Self::JFORI,
            79 => Self::FORL,
            80 => Self::IFORL,
            81 => Self::JFORL,
            82 => Self::ITERL,
            83 => Self::IITERL,
            84 => Self::JITERL,
            85 => Self::LOOP,
            86 => Self::ILOOP,
            87 => Self::JLOOP,
            88 => Self::JMP,
            89 => Self::FUNCF,
            90 => Self::IFUNCF,
            91 => Self::JFUNCF,
            92 => Self::FUNCV,
            93 => Self::IFUNCV,
            94 => Self::JFUNCV,
            95 => Self::FUNCC,
            96 => Self::FUNCCW,
            _ => panic!("Invalid opcode value: {}", value),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value as u8
    }
}

impl Packable for Instruction {
    fn decode(raw: u32) -> Self {
        use constants::*;

        // Extract opcode and operands
        let opcode: Opcode = ((raw & mask!(SIZE_OP, 0)) as u8).into();
        let a = ((raw >> POS_A) & mask!(SIZE_A, 0)) as u8;

        if opcode.uses_ad_format() {
            // Format AD
            let d = ((raw >> POS_D) & mask!(SIZE_D, 0)) as u16;

            // Special case: JMP instruction is handled as format AJ
            if opcode == Opcode::JMP {
                // Convert biased value to signed
                let j = (d as i32) - BCBIAS_J;
                return Instruction::AJ(opcode, a, j);
            }

            Instruction::AD(opcode, a, d)
        } else {
            // Format ABC
            let b = ((raw >> POS_B) & mask!(SIZE_B, 0)) as u8;
            let c = ((raw >> POS_C) & mask!(SIZE_C, 0)) as u8;
            Instruction::ABC(opcode, a, b, c)
        }
    }

    fn encode(inst: Self) -> u32 {
        use constants::*;

        match inst {
            Instruction::ABC(opcode, a, b, c) => {
                let op = opcode as u32;
                let a = (a as u32) << POS_A;
                let b = (b as u32) << POS_B;
                let c = (c as u32) << POS_C;
                op | a | b | c
            }
            Instruction::AD(opcode, a, d) => {
                let op = opcode as u32;
                let a = (a as u32) << POS_A;
                let d = (d as u32) << POS_D;
                op | a | d
            }
            Instruction::AJ(opcode, a, j) => {
                let op = opcode as u32;
                let a = (a as u32) << POS_A;
                let d = ((j + BCBIAS_J) as u32) << POS_D;
                op | a | d
            }
        }
    }
}

impl<'a> ctx::TryFromCtx<'a, BytecodeContext> for Instruction {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: BytecodeContext) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let instruction: u32 = src.gread_with(offset, ctx.endian)?;
        let instruction = Instruction::decode(instruction);

        Ok((instruction, *offset))
    }
}

impl ctx::TryIntoCtx<BytecodeContext> for Instruction {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: BytecodeContext) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        let n = Instruction::encode(self);
        src.gwrite_with(n, offset, ctx.endian)?;

        Ok(*offset)
    }
}
