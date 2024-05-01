//! Lua disassembly framework.
//!
//!

pub mod lua51;
pub mod lua52;
pub mod lua53;
pub mod lua54;
pub mod luajit;
//pub mod luau;
// maybe do Luau?

#[macro_use]
pub mod utils;

mod errors;
pub use errors::Error;
