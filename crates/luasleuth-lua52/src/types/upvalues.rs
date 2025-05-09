use luasleuth_common::CommonCtx;
use scroll::{ctx, Pread, Pwrite};

#[derive(Debug, Pwrite)]
pub struct Upvalue {
    pub in_stack: u8,
    pub index: u8,
}

impl<'a> ctx::TryFromCtx<'a, CommonCtx> for Upvalue {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let in_stack: u8 = src.gread_with(offset, ctx.endianness)?;
        let index: u8 = src.gread_with(offset, ctx.endianness)?;

        Ok((Self { in_stack, index }, *offset))
    }
}
