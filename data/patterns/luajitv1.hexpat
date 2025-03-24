#pragma description LuaJIT v2.0 bytecode
#pragma author bananasov
#pragma magic [ 0x1B 0x4C 0x4A 0x01 ] @ 0x00

import type.magic;
import type.leb128;

const u8 BCDUMP_F_BE = 0x01; // Flag is set if bytecode is using big endian byte order
const u8 BCDUMP_F_STRIP = 0x02; // Flag is set when debug info is stripped
const u8 BCDUMP_F_FFI = 0x04; // Flag is set if bytecode uses FFI features

const u8 BCDUMP_KGC_CHILD = 0x00; // Reference to a child function prototype
const u8 BCDUMP_KGC_TAB = 0x01; // A table constant
const u8 BCDUMP_KGC_I64 = 0x02; // A 64-bit signed integer (when FFI is enabled)
const u8 BCDUMP_KGC_U64 = 0x03; // A 64-bit unsigned integer (when FFI is enabled)
const u8 BCDUMP_KGC_COMPLEX = 0x04; // A complex number (when FFI is enabled)
const u8 BCDUMP_KGC_STR = 0x05; // A string constant, with length encoded in the type value

const u8 BCDUMP_KTAB_NIL = 0x00; // A nil value
const u8 BCDUMP_KTAB_FALSE = 0x01; // A boolean false
const u8 BCDUMP_KTAB_TRUE = 0x02; // A boolean true
const u8 BCDUMP_KTAB_INT = 0x03; // An integer as unsigned LEB128
const u8 BCDUMP_KTAB_NUM = 0x04; // A floating-point number (two unsigned LEB128 values)
const u8 BCDUMP_KTAB_STR = 0x05; // A string constant, with length encoded in the type value

using Prototype;

struct Header {
    type::Magic<"\x1BLJ"> magic;
    u8 version;
    type::uLEB128 flags;

    if (!(flags & BCDUMP_F_STRIP) != 0) {
        type::uLEB128 chunk_name_length;
        char chunk_name[chunk_name_length];
    }
};

struct GCConstant {
    type::uLEB128 constant_type;

    if (constant_type == BCDUMP_KGC_CHILD) {
    } else if (constant_type == BCDUMP_KGC_TAB) {
    } else if (constant_type == BCDUMP_KGC_I64) {
    } else if (constant_type == BCDUMP_KGC_U64) {
    } else if (constant_type == BCDUMP_KGC_COMPLEX) {
    } else if (constant_type >= BCDUMP_KGC_STR) {
        u8 string_length = constantType - 5;
        char string[string_length];
    }
};

struct Prototype {
    type::uLEB128 prototype_length;
    if (prototype_length == 0) {
        return;
    }

    u8 flags;
    u8 parameter_count;
    u8 frame_size;
    u8 upvalue_count;
    type::uLEB128 gc_constant_count;
    type::uLEB128 num_constant_count;
    type::uLEB128 instruction_count;

    if (!(flags & BCDUMP_F_STRIP) != 0) {
        type::uLEB128 debug_length;
        if (debug_length > 0) {
            type::uLEB128 first_line;
            type::uLEB128 line_count;
        }
    }

    u32 instructions[instruction_count];
    u16 upvalue_refs[upvalue_count];
};

struct Bytecode {
    Header header;
    Prototype prototype;
};

Bytecode file @ 0x00;
