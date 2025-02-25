mod array;
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
