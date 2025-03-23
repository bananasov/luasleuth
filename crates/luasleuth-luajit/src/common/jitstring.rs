//! A string type for LuaJIT v1 and v2 bytecode
//! A string type for LuaJIT v1 and v2 bytecode
use scroll::{
    ctx::{self, StrCtx},
    Pread, Uleb128,
};

/// String type identifier constant
const BCDUMP_STR_TYPE: u64 = 5;

#[derive(Debug)]
pub struct JitString<'a> {
    pub size: usize,
    pub data: &'a str,
}

impl<'a> JitString<'a> {
    /// Create a new JitString from a string slice
    #[inline]
    pub fn new(data: &'a str) -> Self {
        Self {
            size: data.len(),
            data,
        }
    }

    /// Create an empty JitString
    #[inline]
    pub fn empty() -> Self {
        Self { size: 0, data: "" }
    }
}

impl<'a> ctx::TryFromCtx<'a, ()> for JitString<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let type_and_len: Uleb128 = src.gread_with(offset, ())?;
        let type_and_len: u64 = type_and_len.into();
        if type_and_len < BCDUMP_STR_TYPE {
            return Err(scroll::Error::Custom(
                "Invalid string type identifier".into(),
            ));
        }

        let size = (type_and_len - BCDUMP_STR_TYPE) as usize;
        if size == 0 {
            return Ok((JitString::empty(), *offset));
        }

        let data: &str = src.gread_with(offset, StrCtx::Length(size))?;

        Ok((Self { size, data }, *offset))
    }
}

impl<'a> From<&'a str> for JitString<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            size: value.len(),
            data: value,
        }
    }
}

impl PartialEq for JitString<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}

impl Eq for JitString<'_> {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_success() {
        use super::*;

        let data: [u8; 0x0E] = [
            0x12, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ];

        let offset = &mut 0;
        let string: JitString = data
            .gread_with(offset, ())
            .expect("Failed to read JitString from data");
        assert_eq!(string, JitString::new("Hello, World!"));
    }
}
