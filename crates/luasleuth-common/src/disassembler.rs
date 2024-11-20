pub trait Disassemble<T> {
    type Error;
    fn disassemble(self) -> Result<T, Self::Error>;
}
