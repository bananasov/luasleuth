use scroll::{ctx, Pread, Pwrite};

#[derive(Copy, Clone)]
pub struct LuaStringCtx {
    pub endianess: scroll::Endian,
    pub size_of_sizet: u8,
}

pub struct LuaString<'b> {
    length: usize,
    data: &'b str,
}

impl<'b> LuaString<'b> {
    pub fn into_string(self) -> String {
        String::from(self.data)
    }
}

impl LuaStringCtx {
    #[inline]
    pub fn new_le(size_of_sizet: u8) -> Self {
        Self {
            endianess: scroll::LE,
            size_of_sizet,
        }
    }
}

impl<'a> ctx::TryFromCtx<'a, LuaStringCtx> for LuaString<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: LuaStringCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let size: usize = match ctx.size_of_sizet {
            4 => src.gread_with::<u32>(offset, ctx.endianess)? as usize,
            8 => src.gread_with::<u64>(offset, ctx.endianess)? as usize,
            _ => unreachable!(), // TODO: Custom error
        };
        let str: &str = src.gread_with(offset, ctx::StrCtx::Length(size - 1))?;

        Ok((
            LuaString {
                length: size,
                data: str,
            },
            *offset + 1, // Add the removed null byte from the string back to the offset
        ))
    }
}

impl<'a> ctx::TryIntoCtx<LuaStringCtx> for LuaString<'a> {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: LuaStringCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        // Write the appropiate size of the string
        match ctx.size_of_sizet {
            4 => src.gwrite_with(self.length as u32, offset, ctx.endianess)?,
            8 => src.gwrite_with(self.length as u64, offset, ctx.endianess)?,
            _ => unreachable!("Invalid size_t size"),
        };

        src.gwrite_with(self.data, offset, ())?;
        src.gwrite(b'\0', offset)?; // Add the missing null byte

        Ok(*offset)
    }
}
