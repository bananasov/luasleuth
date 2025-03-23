/// A common ctx used between each LuaJIT version
#[derive(Copy, Clone)]
pub struct BytecodeContext {
    /// If the bytecode is stripped
    pub stripped_bytecode: bool,
}
