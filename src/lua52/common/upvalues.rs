#[derive(Debug)]
pub struct Upvalue {
    pub in_stack: u8,
    pub index: u8,
}
