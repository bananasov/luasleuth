use super::types::Bytecode;

pub trait Disassemble<T: Bytecode> {
    type Error;
    fn disassemble(self) -> Result<T, Self::Error>;
}
