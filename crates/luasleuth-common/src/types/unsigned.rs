use scroll::{ctx, Endian, Error, Pread, Pwrite, Result};

use crate::CommonCtx;

#[derive(Debug)]
pub struct LuaUnsigned {
    pub value: usize,
}

impl LuaUnsigned {
    #[inline]
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

impl<'a> ctx::TryFromCtx<'a, Endian> for LuaUnsigned {
    type Error = Error;

    fn try_from_ctx(src: &'a [u8], _ctx: Endian) -> Result<(Self, usize)> {
        let offset = &mut 0;
        let mut value: usize = 0;
        let limit = usize::MAX >> 7;

        loop {
            let b: u8 = src.gread(offset)?;

            if value >= limit {
                return Err(Error::Custom("integer overflow".into()));
            }

            value = (value << 7) | (b & 0x7f) as usize;

            // Continue loop while highest bit is 0
            if (b & 0x80) != 0 {
                break;
            }
        }

        Ok((LuaUnsigned { value }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for LuaUnsigned {
    type Error = Error;

    fn try_into_ctx(self, dst: &mut [u8], _ctx: Endian) -> Result<usize> {
        let offset = &mut 0;
        let mut x = self.value;

        loop {
            let mut byte = (x & 0x7f) as u8;
            x >>= 7;

            // If this is the last byte, set the highest bit
            if x == 0 {
                byte |= 0x80; // Set high bit to mark end
                dst.gwrite_with(byte, offset, _ctx)?;
                break;
            }

            // More bytes coming, leave high bit as 0
            dst.gwrite_with(byte, offset, _ctx)?;
        }

        Ok(*offset)
    }
}

impl<'a> ctx::TryFromCtx<'a, CommonCtx> for LuaUnsigned {
    type Error = Error;

    fn try_from_ctx(dst: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize)> {
        Self::try_from_ctx(dst, ctx.endianness)
    }
}

impl ctx::TryIntoCtx<CommonCtx> for LuaUnsigned {
    type Error = Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: CommonCtx) -> Result<usize> {
        Self::try_into_ctx(self, dst, ctx.endianness)
    }
}

impl std::fmt::Display for LuaUnsigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.value, f)
    }
}

impl PartialEq for LuaUnsigned {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<usize> for LuaUnsigned {
    fn eq(&self, other: &usize) -> bool {
        self.value == *other
    }
}

impl Eq for LuaUnsigned {}
