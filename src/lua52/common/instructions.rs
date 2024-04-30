#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    iABC(Opcode, u8, u16, u16),
    iABx(Opcode, u8, u32),
    iAsBx(Opcode, u8, i32),
    iAx(Opcode, u32),
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Opcode {
    OP_MOVE = 0,     /*	A B	R(A) := R(B)					*/
    OP_LOADK = 1,    /*	A Bx	R(A) := Kst(Bx)					*/
    OP_LOADKX = 2,   /*	A 	R(A) := Kst(extra arg)				*/
    OP_LOADBOOL = 3, /*	A B C	R(A) := (Bool)B; if (C) pc++			*/
    OP_LOADNIL = 4,  /*	A B	R(A), R(A+1), ..., R(A+B) := nil		*/
    OP_GETUPVAL = 5, /*	A B	R(A) := UpValue[B]				*/

    OP_GETTABUP = 6, /*	A B C	R(A) := UpValue[B][RK(C)]			*/
    OP_GETTABLE = 7, /*	A B C	R(A) := R(B)[RK(C)]				*/

    OP_SETTABUP = 8,  /*	A B C	UpValue[A][RK(B)] := RK(C)			*/
    OP_SETUPVAL = 9,  /*	A B	UpValue[B] := R(A)				*/
    OP_SETTABLE = 10, /*	A B C	R(A)[RK(B)] := RK(C)				*/

    OP_NEWTABLE = 11, /*	A B C	R(A) := {} (size = B,C)				*/

    OP_SELF = 12, /*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/

    OP_ADD = 13, /*	A B C	R(A) := RK(B) + RK(C)				*/
    OP_SUB = 14, /*	A B C	R(A) := RK(B) - RK(C)				*/
    OP_MUL = 15, /*	A B C	R(A) := RK(B) * RK(C)				*/
    OP_DIV = 16, /*	A B C	R(A) := RK(B) / RK(C)				*/
    OP_MOD = 17, /*	A B C	R(A) := RK(B) % RK(C)				*/
    OP_POW = 18, /*	A B C	R(A) := RK(B) ^ RK(C)				*/
    OP_UNM = 19, /*	A B	R(A) := -R(B)					*/
    OP_NOT = 20, /*	A B	R(A) := not R(B)				*/
    OP_LEN = 21, /*	A B	R(A) := length of R(B)				*/

    OP_CONCAT = 22, /*	A B C	R(A) := R(B).. ... ..R(C)			*/

    OP_JMP = 23, /*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
    OP_EQ = 24,  /*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
    OP_LT = 25,  /*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
    OP_LE = 26,  /*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/

    OP_TEST = 27,    /*	A C	if not (R(A) <=> C) then pc++			*/
    OP_TESTSET = 28, /*	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/

    OP_CALL = 29,     /*	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
    OP_TAILCALL = 30, /*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
    OP_RETURN = 31,   /*	A B	return R(A), ... ,R(A+B-2)	(see note)	*/

    OP_FORLOOP = 32, /*	A sBx	R(A)+=R(A+2);
                     if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
    OP_FORPREP = 33, /*	A sBx	R(A)-=R(A+2); pc+=sBx				*/

    OP_TFORCALL = 34, /*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));	*/
    OP_TFORLOOP = 35, /*	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/

    OP_SETLIST = 36, /*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/

    OP_CLOSURE = 37, /*	A Bx	R(A) := closure(KPROTO[Bx])			*/

    OP_VARARG = 38, /*	A B	R(A), R(A+1), ..., R(A+B-2) = vararg		*/

    OP_EXTRAARG = 39, /*	Ax	extra (larger) argument for previous opcode	*/
}
