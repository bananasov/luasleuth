use scroll::{Pread, Pwrite};

#[derive(Debug, Pread, Pwrite)]
pub struct Upvalue {
    pub in_stack: u8,
    pub index: u8,
}