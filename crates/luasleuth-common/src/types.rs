mod array;
pub mod leb128;
mod string;
mod unsigned;

pub use array::Array;
pub use string::LuaString;
pub use unsigned::*;

/// Helper trait for implementing instruction encoding/decoding.
pub trait Packable {
    /// Decode an instruction from its raw value
    fn decode(raw: u32) -> Self;
    /// Encode an instruction into its raw value
    fn encode(inst: Self) -> u32;
}

/// Trait for bytecode instructions.
///
/// This trait is used as a trait bound on the disassembler trait to ensure T is a valid bytecode struct.
pub trait Bytecode {
    /// The unique ID for this bytecode variant
    fn identifier() -> &'static str;

    /// Returns the human-readable display name for the bytecode variant
    fn display_name() -> &'static str;
}
