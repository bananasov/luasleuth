use crate::{try_gread_vec_with, try_gwrite_vec_with, CommonCtx};
use scroll::{ctx, Pread, Pwrite, Uleb128};

#[derive(Debug)]
pub struct Array<T> {
    pub size: usize,
    pub data: Vec<T>,
}

impl<T> Array<T> {
    pub fn read_size(
        src: &[u8],
        offset: &mut usize,
        ctx: CommonCtx,
    ) -> Result<usize, scroll::Error> {
        let size = match ctx.lua_version.into_tuple() {
            (5, 1) => src.gread_with::<i32>(offset, ctx.endianness)? as usize,
            (5, 2) => src.gread_with::<i32>(offset, ctx.endianness)? as usize,
            (5, 3) => src.gread_with::<i32>(offset, ctx.endianness)? as usize,
            (5, 4) => Uleb128::read(src, offset)? as usize,
            _ => return Err(scroll::Error::Custom("Unsupported Lua version".into())),
        };

        Ok(size)
    }

    pub fn write_size(
        &self,
        dst: &mut [u8],
        offset: &mut usize,
        ctx: CommonCtx,
    ) -> Result<usize, scroll::Error> {
        let bytes_written = match ctx.lua_version.into_tuple() {
            (5, 1) => dst.gwrite_with(self.size as i32, offset, ctx.endianness)?,
            (5, 2) => dst.gwrite_with(self.size as i32, offset, ctx.endianness)?,
            (5, 3) => dst.gwrite_with(self.size as i32, offset, ctx.endianness)?,
            // (5, 4) => Uleb128::read(src, offset)? as u64, // No writing support for Uleb128 in scroll
            _ => return Err(scroll::Error::Custom("Unsupported Lua version".into())),
        };

        Ok(bytes_written)
    }
}

impl<T> From<Array<T>> for Vec<T> {
    fn from(value: Array<T>) -> Self {
        value.data
    }
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            size: value.len(),
            data: value,
        }
    }
}

impl ctx::TryFromCtx<'_, CommonCtx> for Array<i32> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'_ [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let size = Array::<i32>::read_size(src, offset, ctx)?; // Is the generic really needed?
        let data: Vec<i32> = try_gread_vec_with!(src, offset, size, ctx.endianness);

        Ok((Self { size, data }, *offset))
    }
}

impl ctx::TryIntoCtx<CommonCtx> for Array<i32> {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: CommonCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        Self::write_size(&self, dst, offset, ctx)?;
        try_gwrite_vec_with!(dst, offset, self.data, ctx.endianness);

        Ok(*offset)
    }
}

impl<'a, T: 'a> ctx::TryFromCtx<'a, CommonCtx> for Array<T>
where
    T: ctx::TryFromCtx<'a, CommonCtx, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let size = Array::<T>::read_size(src, offset, ctx)?; // Is the generic really needed?
        let data: Vec<T> = try_gread_vec_with!(src, offset, size, ctx);

        Ok((Array { size, data }, *offset))
    }
}

impl<T> ctx::TryIntoCtx<CommonCtx> for Array<T>
where
    T: ctx::TryIntoCtx<CommonCtx, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: CommonCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        Self::write_size(&self, dst, offset, ctx)?;
        try_gwrite_vec_with!(dst, offset, self.data, ctx);

        Ok(*offset)
    }
}
