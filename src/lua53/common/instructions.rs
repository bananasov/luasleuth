use scroll::{ctx, Pread, Pwrite};

mod constants {
    pub const SIZE_A: u8 = 8;
    pub const SIZE_B: u8 = 9;
    pub const SIZE_C: u8 = 9;
    pub const SIZE_BX: u8 = SIZE_C + SIZE_B;
    pub const SIZE_AX: u8 = SIZE_C + SIZE_B + SIZE_A;

    pub const SIZE_OP: u8 = 6;

    pub const POS_OP: u8 = 0;
    pub const POS_A: u8 = POS_OP + SIZE_OP;
    pub const POS_C: u8 = POS_A + SIZE_A;
    pub const POS_B: u8 = POS_C + SIZE_C;
    pub const POS_BX: u8 = POS_C;
    pub const POS_AX: u8 = POS_A;
}

macro_rules! mask {
    ($n:expr) => {
        !((!0) << ($n))
    };
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    iABC(Opcode, i32, i32, i32),
    iABx(Opcode, i32, u32),
    iAsBx(Opcode, i32, i32),
    iAx(Opcode, i32),
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Opcode {
    OP_MOVE,     /*	A B	R(A) := R(B)					*/
    OP_LOADK,    /*	A Bx	R(A) := Kst(Bx)					*/
    OP_LOADKX,   /*	A 	R(A) := Kst(extra arg)				*/
    OP_LOADBOOL, /*	A B C	R(A) := (Bool)B; if (C) pc++			*/
    OP_LOADNIL,  /*	A B	R(A), R(A+1), ..., R(A+B) := nil		*/
    OP_GETUPVAL, /*	A B	R(A) := UpValue[B]				*/

    OP_GETTABUP, /*	A B C	R(A) := UpValue[B][RK(C)]			*/
    OP_GETTABLE, /*	A B C	R(A) := R(B)[RK(C)]				*/

    OP_SETTABUP, /*	A B C	UpValue[A][RK(B)] := RK(C)			*/
    OP_SETUPVAL, /*	A B	UpValue[B] := R(A)				*/
    OP_SETTABLE, /*	A B C	R(A)[RK(B)] := RK(C)				*/

    OP_NEWTABLE, /*	A B C	R(A) := {} (size = B,C)				*/

    OP_SELF, /*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/

    OP_ADD,  /*	A B C	R(A) := RK(B) + RK(C)				*/
    OP_SUB,  /*	A B C	R(A) := RK(B) - RK(C)				*/
    OP_MUL,  /*	A B C	R(A) := RK(B) * RK(C)				*/
    OP_MOD,  /*	A B C	R(A) := RK(B) % RK(C)				*/
    OP_POW,  /*	A B C	R(A) := RK(B) ^ RK(C)				*/
    OP_DIV,  /*	A B C	R(A) := RK(B) / RK(C)				*/
    OP_IDIV, /*	A B C	R(A) := RK(B) // RK(C)				*/
    OP_BAND, /*	A B C	R(A) := RK(B) & RK(C)				*/
    OP_BOR,  /*	A B C	R(A) := RK(B) | RK(C)				*/
    OP_BXOR, /*	A B C	R(A) := RK(B) ~ RK(C)				*/
    OP_SHL,  /*	A B C	R(A) := RK(B) << RK(C)				*/
    OP_SHR,  /*	A B C	R(A) := RK(B) >> RK(C)				*/
    OP_UNM,  /*	A B	R(A) := -R(B)					*/
    OP_BNOT, /*	A B	R(A) := ~R(B)					*/
    OP_NOT,  /*	A B	R(A) := not R(B)				*/
    OP_LEN,  /*	A B	R(A) := length of R(B)				*/

    OP_CONCAT, /*	A B C	R(A) := R(B).. ... ..R(C)			*/

    OP_JMP, /*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
    OP_EQ,  /*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
    OP_LT,  /*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
    OP_LE,  /*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/

    OP_TEST,    /*	A C	if not (R(A) <=> C) then pc++			*/
    OP_TESTSET, /*	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/

    OP_CALL,     /*	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
    OP_TAILCALL, /*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
    OP_RETURN,   /*	A B	return R(A), ... ,R(A+B-2)	(see note)	*/

    OP_FORLOOP, /*	A sBx	R(A)+=R(A+2);
                if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
    OP_FORPREP, /*	A sBx	R(A)-=R(A+2); pc+=sBx				*/

    OP_TFORCALL, /*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));	*/
    OP_TFORLOOP, /*	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/

    OP_SETLIST, /*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/

    OP_CLOSURE, /*	A Bx	R(A) := closure(KPROTO[Bx])			*/

    OP_VARARG, /*	A B	R(A), R(A+1), ..., R(A+B-2) = vararg		*/

    OP_EXTRAARG, /*	Ax	extra (larger) argument for previous opcode	*/
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> u8 {
        op as u8
    }
}

impl From<u8> for Opcode {
    fn from(op: u8) -> Self {
        match op {
            0 => Opcode::OP_MOVE,
            1 => Opcode::OP_LOADK,
            2 => Opcode::OP_LOADKX,
            3 => Opcode::OP_LOADBOOL,
            4 => Opcode::OP_LOADNIL,
            5 => Opcode::OP_GETUPVAL,
            6 => Opcode::OP_GETTABUP,
            7 => Opcode::OP_GETTABLE,
            8 => Opcode::OP_SETTABUP,
            9 => Opcode::OP_SETUPVAL,
            10 => Opcode::OP_SETTABLE,
            11 => Opcode::OP_NEWTABLE,
            12 => Opcode::OP_SELF,
            13 => Opcode::OP_ADD,
            14 => Opcode::OP_SUB,
            15 => Opcode::OP_MUL,
            16 => Opcode::OP_MOD,
            17 => Opcode::OP_POW,
            18 => Opcode::OP_DIV,
            19 => Opcode::OP_IDIV,
            20 => Opcode::OP_BAND,
            21 => Opcode::OP_BOR,
            22 => Opcode::OP_BXOR,
            23 => Opcode::OP_SHL,
            24 => Opcode::OP_SHR,
            25 => Opcode::OP_UNM,
            26 => Opcode::OP_BNOT,
            27 => Opcode::OP_NOT,
            28 => Opcode::OP_LEN,
            29 => Opcode::OP_CONCAT,
            30 => Opcode::OP_JMP,
            31 => Opcode::OP_EQ,
            32 => Opcode::OP_LT,
            33 => Opcode::OP_LE,
            34 => Opcode::OP_TEST,
            35 => Opcode::OP_TESTSET,
            36 => Opcode::OP_CALL,
            37 => Opcode::OP_TAILCALL,
            38 => Opcode::OP_RETURN,
            39 => Opcode::OP_FORLOOP,
            40 => Opcode::OP_FORPREP,
            41 => Opcode::OP_TFORCALL,
            42 => Opcode::OP_TFORLOOP,
            43 => Opcode::OP_SETLIST,
            44 => Opcode::OP_CLOSURE,
            45 => Opcode::OP_VARARG,
            46 => Opcode::OP_EXTRAARG,
            _ => unreachable!("Invalid opcode: {}", op),
        }
    }
}

impl Opcode {
    pub fn decode(op: u32) -> Instruction {
        use constants::*;
        use Opcode::*;

        let opcode: Opcode = ((op & mask!(SIZE_OP)) as u8).into();
        let a = (op >> POS_A) & mask!(SIZE_A);

        match opcode {
            // iABC
            OP_MOVE | OP_LOADBOOL | OP_LOADNIL | OP_GETUPVAL | OP_GETTABUP | OP_GETTABLE
            | OP_SETTABUP | OP_SETUPVAL | OP_SETTABLE | OP_NEWTABLE | OP_SELF | OP_ADD | OP_SUB
            | OP_MUL | OP_MOD | OP_POW | OP_DIV | OP_IDIV | OP_BAND | OP_BOR | OP_BXOR | OP_SHL
            | OP_SHR | OP_UNM | OP_BNOT | OP_NOT | OP_LEN | OP_CONCAT | OP_EQ | OP_LT | OP_LE
            | OP_TEST | OP_TESTSET | OP_CALL | OP_TAILCALL | OP_RETURN | OP_TFORCALL
            | OP_SETLIST | OP_VARARG => {
                let b = (op >> POS_B) & mask!(SIZE_B);
                let c = (op >> POS_C) & mask!(SIZE_C);
                Instruction::iABC(opcode, a as i32, b as i32, c as i32)
            }
            // iABx
            OP_LOADK | OP_LOADKX | OP_CLOSURE => {
                let bx = (op >> POS_BX) & mask!(SIZE_BX);
                Instruction::iABx(opcode, a as i32, bx)
            }
            // iAsBx
            OP_JMP | OP_FORLOOP | OP_FORPREP | OP_TFORLOOP => {
                let sbx = (((op >> POS_BX) & mask!(SIZE_BX)) as i32) - 131071;
                Instruction::iAsBx(opcode, a as i32, sbx)
            }
            // iAx
            OP_EXTRAARG => {
                let ax = (op >> POS_AX) & mask!(SIZE_AX);
                Instruction::iAx(opcode, ax as i32)
            }
        }
    }
    pub fn encode(instruction: Instruction) -> u32 {
        use constants::*;

        match instruction {
            Instruction::iABC(opcode, a, b, c) => {
                let opcode = opcode as u32;
                let a = (a as u32) << POS_A;
                let b = (b as u32) << POS_B;
                let c = (c as u32) << POS_C;

                opcode | a | b | c
            },
            Instruction::iABx(opcode, a, bx) => {
                let opcode = opcode as u32;
                let a = (a << POS_A) as u32;
                let bx = (bx << POS_BX) as u32;
                opcode | a | bx
            },
            Instruction::iAsBx(opcode, a, sbx) => {
                let opcode = opcode as u32;
                let a = (a << POS_A) as u32;
                let sbx = ((sbx + 131071) << POS_BX) as u32;
                opcode | a | sbx
            },
            Instruction::iAx(opcode, ax) => {
                let opcode = opcode as u32;
                let ax = (ax << POS_AX) as u32;
                opcode | ax
            },
        }
    }
}

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for Instruction {
    type Error = scroll::Error;

    fn try_from_ctx(from: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let instruction: u32 = from.gread_with(offset, ctx)?;
        let instruction = Opcode::decode(instruction);

        Ok((instruction, *offset))
    }
}

impl<'a> ctx::TryIntoCtx<scroll::Endian> for Instruction {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        let n = Opcode::encode(self);
        src.gwrite_with(n, offset, ctx)?;

        Ok(*offset)
    }
}
