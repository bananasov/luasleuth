// Code copied from: https://github.com/Sculas/scroll/blob/998df554f374e1f13e166c720a1f69a2db06a53e/src/leb128.rs

use core::convert::{AsRef, From};
use core::u8;
use scroll::ctx::{TryFromCtx, TryIntoCtx};
use scroll::{Error, Pread, Pwrite};

#[derive(Debug, PartialEq, Copy, Clone)]
/// An unsigned leb128 integer
pub struct Uleb128 {
    value: u64,
    count: usize,
}

impl Uleb128 {
    /// Return how many bytes this Uleb128 takes up in memory
    #[inline]
    pub fn size(&self) -> usize {
        self.count
    }

    /// Read a variable length u64 from `src` at `offset`
    #[inline]
    pub fn read(src: &[u8], offset: &mut usize) -> Result<u64, Error> {
        let tmp = src.pread::<Uleb128>(*offset)?;
        *offset += tmp.size();
        Ok(tmp.into())
    }

    /// Write a variable length u64 to `src` at `offset`
    #[inline]
    pub fn write(dst: &mut [u8], offset: &mut usize, value: u64) -> Result<(), Error> {
        dst.gwrite(Uleb128 { value, count: 0 }, offset)?;
        Ok(())
    }
}

impl AsRef<u64> for Uleb128 {
    fn as_ref(&self) -> &u64 {
        &self.value
    }
}

impl From<Uleb128> for u64 {
    #[inline]
    fn from(uleb128: Uleb128) -> u64 {
        uleb128.value
    }
}

impl From<u64> for Uleb128 {
    fn from(value: u64) -> Self {
        Self { value, count: 0 }
    }
}

impl From<Uleb128> for usize {
    fn from(value: Uleb128) -> Self {
        value.value as usize
    }
}

impl From<usize> for Uleb128 {
    fn from(value: usize) -> Self {
        Self {
            value: value as u64,
            count: 0,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// An signed leb128 integer
pub struct Sleb128 {
    value: i64,
    count: usize,
}

impl Sleb128 {
    /// Return how many bytes this Sleb128 takes up in memory
    #[inline]
    pub fn size(&self) -> usize {
        self.count
    }

    /// Read a variable length i64 from `src` at `offset`
    #[inline]
    pub fn read(src: &[u8], offset: &mut usize) -> Result<i64, Error> {
        let tmp = src.pread::<Sleb128>(*offset)?;
        *offset += tmp.size();
        Ok(tmp.into())
    }

    /// Write a variable length i64 to `dst` at `offset`
    #[inline]
    pub fn write(dst: &mut [u8], offset: &mut usize, value: i64) -> Result<(), Error> {
        dst.gwrite(Sleb128 { value, count: 0 }, offset)?;
        Ok(())
    }
}

impl AsRef<i64> for Sleb128 {
    fn as_ref(&self) -> &i64 {
        &self.value
    }
}

impl From<Sleb128> for i64 {
    #[inline]
    fn from(sleb128: Sleb128) -> i64 {
        sleb128.value
    }
}

impl From<i64> for Sleb128 {
    fn from(value: i64) -> Self {
        Self { value, count: 0 }
    }
}

// Below implementation heavily adapted from:
// - https://github.com/fitzgen/leb128
// - https://github.com/rjsberry/nano
const CONTINUATION_BIT: u8 = 1 << 7;
const SIGN_BIT: u8 = 1 << 6;

#[inline]
fn mask_continuation(byte: u8) -> u8 {
    byte & !CONTINUATION_BIT
}

impl<'a> TryFromCtx<'a> for Uleb128 {
    type Error = scroll::Error;

    #[inline]
    fn try_from_ctx(src: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut result = 0;
        let mut shift = 0;
        let mut count = 0;
        loop {
            let byte: u8 = src.pread(count)?;

            if shift == 63 && byte != 0x00 && byte != 0x01 {
                return Err(Error::BadInput {
                    size: src.len(),
                    msg: "failed to parse",
                });
            }

            let low_bits = u64::from(mask_continuation(byte));
            result |= low_bits << shift;

            count += 1;
            shift += 7;

            if byte & CONTINUATION_BIT == 0 {
                return Ok((
                    Uleb128 {
                        value: result,
                        count,
                    },
                    count,
                ));
            }
        }
    }
}

impl<'a> TryFromCtx<'a> for Sleb128 {
    type Error = Error;

    #[inline]
    fn try_from_ctx(src: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let o = 0;
        let offset = &mut 0;
        let mut result = 0;
        let mut shift = 0;
        let size = 64;
        let mut byte: u8;
        loop {
            byte = src.gread_with(offset, scroll::LE)?;

            if shift == 63 && byte != 0x00 && byte != 0x7f {
                return Err(Error::BadInput {
                    size: src.len(),
                    msg: "failed to parse",
                });
            }

            let low_bits = i64::from(mask_continuation(byte));
            result |= low_bits << shift;
            shift += 7;

            if byte & CONTINUATION_BIT == 0 {
                break;
            }
        }

        if shift < size && (SIGN_BIT & byte) == SIGN_BIT {
            // Sign extend the result.
            result |= !0 << shift;
        }
        let count = *offset - o;
        Ok((
            Sleb128 {
                value: result,
                count,
            },
            count,
        ))
    }
}

impl TryIntoCtx for Uleb128 {
    type Error = Error;

    #[inline]
    fn try_into_ctx(self, dst: &mut [u8], _: ()) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        let mut value = self.value;
        loop {
            let mut byte = (value as u8) & !CONTINUATION_BIT;
            value >>= 7;
            if value != 0 {
                byte |= CONTINUATION_BIT;
            }
            dst.gwrite_with(byte, offset, scroll::LE)?;
            if value == 0 {
                break;
            }
        }
        Ok(*offset)
    }
}

impl TryIntoCtx for Sleb128 {
    type Error = Error;

    #[inline]
    fn try_into_ctx(self, dst: &mut [u8], _: ()) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        let mut value = self.value;
        loop {
            let mut byte = (value as u8) & !CONTINUATION_BIT;
            value >>= 7;
            if (value == 0 && (byte & SIGN_BIT) == 0) || (value == -1 && (byte & SIGN_BIT) != 0) {
                dst.gwrite_with(byte, offset, scroll::LE)?;
                break;
            } else {
                byte |= CONTINUATION_BIT;
                dst.gwrite_with(byte, offset, scroll::LE)?;
            }
        }
        Ok(*offset)
    }
}
