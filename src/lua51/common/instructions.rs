use scroll::{ctx, Pread, Pwrite};

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
    OP_MOVE = 0,     /*	A B	R(A) := R(B)					*/
    OP_LOADK = 1,    /*	A Bx	R(A) := Kst(Bx)					*/
    OP_LOADBOOL = 2, /*	A B C	R(A) := (Bool)B; if (C) pc++			*/
    OP_LOADNIL = 3,  /*	A B	R(A) := ... := R(B) := nil			*/
    OP_GETUPVAL = 4, /*	A B	R(A) := UpValue[B]				*/

    OP_GETGLOBAL = 5, /*	A Bx	R(A) := Gbl[Kst(Bx)]				*/
    OP_GETTABLE = 6,  /*	A B C	R(A) := R(B)[RK(C)]				*/

    OP_SETGLOBAL = 7, /*	A Bx	Gbl[Kst(Bx)] := R(A)				*/
    OP_SETUPVAL = 8,  /*	A B	UpValue[B] := R(A)				*/
    OP_SETTABLE = 9,  /*	A B C	R(A)[RK(B)] := RK(C)				*/

    OP_NEWTABLE = 10, /*	A B C	R(A) := {} (size = B,C)				*/

    OP_SELF = 11, /*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/

    OP_ADD = 12, /*	A B C	R(A) := RK(B) + RK(C)				*/
    OP_SUB = 13, /*	A B C	R(A) := RK(B) - RK(C)				*/
    OP_MUL = 14, /*	A B C	R(A) := RK(B) * RK(C)				*/
    OP_DIV = 15, /*	A B C	R(A) := RK(B) / RK(C)				*/
    OP_MOD = 16, /*	A B C	R(A) := RK(B) % RK(C)				*/
    OP_POW = 17, /*	A B C	R(A) := RK(B) ^ RK(C)				*/
    OP_UNM = 18, /*	A B	R(A) := -R(B)					*/
    OP_NOT = 19, /*	A B	R(A) := not R(B)				*/
    OP_LEN = 20, /*	A B	R(A) := length of R(B)				*/

    OP_CONCAT = 21, /*	A B C	R(A) := R(B).. ... ..R(C)			*/

    OP_JMP = 22, /*	sBx	pc+=sBx					*/

    OP_EQ = 23, /*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
    OP_LT = 24, /*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++  		*/
    OP_LE = 25, /*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++  		*/

    OP_TEST = 26,    /*	A C	if not (R(A) <=> C) then pc++			*/
    OP_TESTSET = 27, /*	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/

    OP_CALL = 28,     /*	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
    OP_TAILCALL = 29, /*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
    OP_RETURN = 30,   /*	A B	return R(A), ... ,R(A+B-2)	(see note)	*/

    OP_FORLOOP = 31, /*	A sBx	R(A)+=R(A+2);
                     if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
    OP_FORPREP = 32, /*	A sBx	R(A)-=R(A+2); pc+=sBx				*/

    OP_TFORLOOP = 33, /*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));
                      if R(A+3) ~= nil then R(A+2)=R(A+3) else pc++	*/
    OP_SETLIST = 34, /*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/

    OP_CLOSE = 35,   /*	A 	close all variables in the stack up to (>=) R(A)*/
    OP_CLOSURE = 36, /*	A Bx	R(A) := closure(KPROTO[Bx], R(A), ... ,R(A+n))	*/

    OP_VARARG = 37, /*	A B	R(A), R(A+1), ..., R(A+B-1) = vararg		*/
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> u8 {
        op as u8
    }
}

impl From<u8> for Opcode {
    fn from(val: u8) -> Self {
        match val {
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
            _ => unreachable!("Invalid instruction"),
        }
    }
}

impl Opcode {
    pub fn decode(op: u32) -> Instruction {
        use Opcode::*;

        let opcode: Opcode = ((op & 0x3F) as u8).into();
        let a = ((op >> 6) & 0xFF) as u8;

        match opcode {
            OP_MOVE | OP_LOADBOOL | OP_LOADNIL | OP_GETUPVAL | OP_GETTABLE | OP_SETUPVAL
            | OP_SETTABLE | OP_NEWTABLE | OP_SELF | OP_ADD | OP_SUB | OP_MUL | OP_DIV | OP_MOD
            | OP_POW | OP_UNM | OP_NOT | OP_LEN | OP_CONCAT | OP_EQ | OP_LT | OP_LE | OP_TEST
            | OP_TESTSET | OP_CALL | OP_TAILCALL | OP_RETURN | OP_TFORLOOP | OP_SETLIST
            | OP_CLOSE | OP_VARARG => {
                let b = ((op >> 23) & 0x1FF) as u16;
                let c = ((op >> 14) & 0x1FF) as u16;
                Instruction::iABC(opcode, a, b, c)
            }
            OP_LOADK | OP_GETGLOBAL | OP_SETGLOBAL | OP_CLOSURE => {
                let bx = (op >> 14) & 0x3FFFF;
                Instruction::iABx(opcode, a, bx)
            }
            OP_JMP | OP_FORLOOP | OP_FORPREP => {
                let sbx = (((op >> 14) & 0x3FFFF) as i32) - 137071;
                Instruction::iAsBx(opcode, a, sbx)
            }
        }
    }

    pub fn encode(instruction: Instruction) -> u32 {
        match instruction {
            Instruction::iABC(op, a, b, c) => {
                let op = op as u32;
                let a = (a as u32) << 6;
                let c = (c as u32) << 14;
                let b = (b as u32) << 23;

                op | a | c | b
            }
            Instruction::iABx(op, a, bx) => {
                let op = op as u32;
                let a = (a << 6) as u32;
                let bx = bx << 14;

                op | a | bx
            }
            Instruction::iAsBx(op, a, sbx) => {
                let op = op as u32;
                let a = (a << 6) as u32;
                let sbx = ((sbx + 131071) as u32) << 14;
                op | a | sbx
            }
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