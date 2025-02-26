use crate::{types::LuaUnsigned, CommonCtx};
use scroll::{
    ctx::{self, StrCtx},
    Pread, Pwrite,
};

#[derive(Debug)]
pub struct LuaString<'a> {
    pub size: usize,
    pub data: &'a str,
}

impl<'a> LuaString<'a> {
    /// Read a Lua 5.1/5.2 string.
    pub fn read_lua51_string(
        src: &'a [u8],
        offset: &mut usize,
        ctx: CommonCtx,
    ) -> Result<Self, scroll::Error> {
        let size: usize = match ctx.size_of_size_t {
            4 => src.gread_with::<i32>(offset, ctx.endianness)? as usize,
            8 => src.gread_with::<i64>(offset, ctx.endianness)? as usize,
            _ => unimplemented!(),
        };
        let data: &str = src.gread_with(offset, StrCtx::Length(size - 1))?;
        *offset += 1; // null terminator

        Ok(LuaString { size, data })
    }

    /// Read a Lua 5.3 string.
    pub fn read_lua53_string(
        src: &'a [u8],
        offset: &mut usize,
        ctx: CommonCtx,
    ) -> Result<Self, scroll::Error> {
        let size = src.gread_with::<u8>(offset, ctx.endianness)? as usize;
        if size == 0xFF {
            return Self::read_lua51_string(src, offset, ctx);
        }

        // I dont know why we're doing size - 1, but if i dont it fucks up.
        let data: &str = src.gread_with(offset, StrCtx::Length(size - 1))?;

        Ok(LuaString { size, data })
    }

    /// Read a Lua 5.4 string.
    pub fn read_lua54_string(
        src: &'a [u8],
        offset: &mut usize,
        ctx: CommonCtx,
    ) -> Result<Self, scroll::Error> {
        let size: LuaUnsigned = src.gread_with(offset, ctx.endianness)?;
        let data: &str = src.gread_with(offset, StrCtx::Length(size.value - 1))?;

        Ok(LuaString {
            size: size.value,
            data,
        })
    }

    /// Write a Lua 5.1/5.2 string.
    pub fn write_lua51_string(
        self,
        dst: &mut [u8],
        offset: &mut usize,
        ctx: CommonCtx,
    ) -> Result<usize, scroll::Error> {
        match ctx.size_of_size_t {
            4 => dst.gwrite_with(self.size as i32, offset, ctx.endianness)?,
            8 => dst.gwrite_with(self.size as i64, offset, ctx.endianness)?,
            _ => unimplemented!(),
        };

        dst.gwrite_with(self.data, offset, ())?;
        dst.gwrite_with(b'\0', offset, ctx.endianness)?;

        Ok(*offset)
    }

    /// Write a Lua 5.3 string.
    pub fn write_lua53_string(
        self,
        _dst: &mut [u8],
        _offset: &mut usize,
        _ctx: CommonCtx,
    ) -> Result<usize, scroll::Error> {
        todo!()
    }

    /// Write a Lua 5.4 string.
    pub fn write_lua54_string(
        self,
        _dst: &mut [u8],
        _offset: &mut usize,
        _ctx: CommonCtx,
    ) -> Result<usize, scroll::Error> {
        todo!()
    }
}

impl<'a> ctx::TryFromCtx<'a, CommonCtx> for LuaString<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let string = match ctx.lua_version.into_tuple() {
            (5, 1) => Self::read_lua51_string(src, offset, ctx)?,
            (5, 2) => Self::read_lua51_string(src, offset, ctx)?, // Lua 5.1 and 5.2 string dumping is the exact same.
            (5, 3) => Self::read_lua53_string(src, offset, ctx)?,
            (5, 4) => Self::read_lua54_string(src, offset, ctx)?,
            _ => unimplemented!(),
        };

        Ok((string, *offset))
    }
}

impl ctx::TryIntoCtx<CommonCtx> for LuaString<'_> {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: CommonCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        match ctx.lua_version.into_tuple() {
            (5, 1) => self.write_lua51_string(dst, offset, ctx),
            (5, 2) => self.write_lua51_string(dst, offset, ctx),
            (5, 3) => self.write_lua53_string(dst, offset, ctx),
            (5, 4) => self.write_lua54_string(dst, offset, ctx),
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a str> for LuaString<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            size: value.len(),
            data: value,
        }
    }
}

impl PartialEq for LuaString<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}

impl Eq for LuaString<'_> {}

impl std::fmt::Display for LuaString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.data)
    }
}
