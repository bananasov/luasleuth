use scroll::Endian;

/// Flag to determine whether or not bytecode is stripped
pub const BYTECODE_IS_STRIPPED: u64 = 0x02;

/// A common ctx used between each LuaJIT version
#[derive(Copy, Clone)]
pub struct BytecodeContext {
    /// Bytecode flags
    pub flags: u64,

    /// The endian to read with (when needed)
    pub endian: Endian,
}

impl BytecodeContext {
    /// Determine if the bytecode is stripped of debug information
    #[inline]
    pub fn is_stripped(&self) -> bool {
        self.flags & BYTECODE_IS_STRIPPED == 0
    }
}
