#[derive(Debug)]
pub struct Upvalue {
    pub in_stack: bool,
    pub index: u8,
}