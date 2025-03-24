#pragma description LuaJIT v2.1 bytecode
#pragma author bananasov
#pragma magic [ 0x1B 0x4C 0x4A 0x02 ] @ 0x00

// NOTE: As of now, this is unfinished.
//       This pattern reads the bytecode in data/bytecode/luajitv2.bin correctly
//       but it is missing a lot of stuff.

import type.magic;
import type.leb128;

const u8 BCDUMP_F_STRIP = 0x02;

using Prototype;

struct Header {
    type::Magic<"\x1BLJ"> magic;
    u8 version;
    type::uLEB128 flags;

    // Check if the bytecode is stripped
    // if not, add the chunk name
    if (!(flags & BCDUMP_F_STRIP != 0)) {
        type::uLEB128 chunkNameLen;
        char chunkName[chunkNameLen];
    }
};

struct Table {

};

struct Constant {
    type::uLEB128 constantType;

    if (constantType == 0) {
        Prototype childPrototype;
    } else if (constantType == 1) {
        Table table;
    } else if (constantType == 2) { // i64 ffi
    } else if (constantType == 3) { // u64 ffi
    } else if (constantType == 4) { // complex
    } else if (constantType >= 5) {
        u8 stringLength = constantType - 5;
        char string[stringLength];
    }
};

struct Prototype {
    type::uLEB128 prototypeLen;
    u8 flags;
    u8 numParams;
    u8 frameSize;
    u8 numUv; // Upvalue count
    u8 numKgc; // GC Constant count
    type::uLEB128 numKn;
    type::uLEB128 numBc; // Instruction count

    // Check if the bytecode is stripped
    // if not, add the prototype name
    if (flags & BCDUMP_F_STRIP == 0) {
        type::uLEB128 debugLen;
        type::uLEB128 firstLine;
        type::uLEB128 numLine;
    }

    u32 instructions[numBc];
    u16 upvalues[numUv];
    Constant constants[numKgc];
};

struct Bytecode {
    Header header;
    Prototype prototype;
};

Bytecode file @ 0x00;
