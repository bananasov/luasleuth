#[derive(Debug)]
pub struct DebugInfo {
    /// map from opcodes to source lines.
    pub line_info: Vec<i32>,
    /// information about local variables
    pub local_variables: Vec<LocalVariable>,
    /// upvalue information
    pub upvalues: Vec<String>
}

#[derive(Debug)]
pub struct LocalVariable {
    pub name: String,
    /// first point where variable is active
    pub start: i32,
    /// first point where variable is dead
    pub end: u32
}