use crate::try_gread_vec_with;
use scroll::{Endian, Pread};

#[derive(Debug)]
pub struct LuaString(Vec<u8>);

impl<'a> LuaString {
    #[cfg(target_arch = "x86")]
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<LuaString, scroll::Error> {
        let size: u32 = src.gread_with(offset, endian)?;
        let mut data: Vec<u8> = try_gread_vec_with!(src, offset, size, endian);
        let _ = data.pop(); // We remove the null byte at the end, lmao!

        Ok(LuaString(data))
    }

    #[cfg(target_arch = "x86_64")]
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<LuaString, scroll::Error> {
        let size: u64 = src.gread_with(offset, endian)?;
        let mut data: Vec<u8> = try_gread_vec_with!(src, offset, size, endian);
        let _ = data.pop(); // We remove the null byte at the end, lmao!

        Ok(LuaString(data))
    }
}

impl From<LuaString> for String {
    fn from(value: LuaString) -> Self {
        String::from_utf8(value.0).unwrap()
    }
}

impl std::fmt::Display for LuaString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from_utf8(self.0.clone()).unwrap();
        f.write_str(&str)
    }
}
