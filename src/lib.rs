//! Lua disassembly framework.
//! 
//! 

pub mod lua54;
pub mod lua53;
pub mod lua52;
pub mod lua51;
pub mod luajit;

mod errors;
pub use errors::Error;
//pub mod luau;
// maybe do Luau?