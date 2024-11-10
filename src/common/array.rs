use scroll::{ctx, Endian, Pread, Pwrite};
use crate::{try_gread_vec_with, try_gwrite_vec_with};
use crate::common::string::LuaStringCtx;

#[derive(Debug)]
pub struct Array<T> {
    pub size: i32,
    pub data: Vec<T>,
}

impl<T> From<Array<T>> for Vec<T> {
    fn from(value: Array<T>) -> Self {
        value.data
    }
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            size: value.len() as i32,
            data: value,
        }
    }
}

impl<'a, T: 'a> ctx::TryFromCtx<'a, Endian> for Array<T>
where
    T: ctx::TryFromCtx<'a, Endian, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let size: i32 = src.gread_with(offset, ctx)?;
        let data: Vec<T> = try_gread_vec_with!(src, offset, size, ctx);

        Ok((
            Array {
                size,
                data,
            },
            *offset,
        ))
    }
}

impl<'a, T: 'a> ctx::TryFromCtx<'a, LuaStringCtx> for Array<T>
where
    T: ctx::TryFromCtx<'a, LuaStringCtx, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: LuaStringCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let size: i32 = src.gread_with(offset, ctx.endianess)?;
        let data: Vec<T> = try_gread_vec_with!(src, offset, size, ctx);

        Ok((
            Array {
                size,
                data,
            },
            *offset,
        ))
    }
}

impl<'a, T: 'a> ctx::TryIntoCtx<Endian> for Array<T>
where
    T: ctx::TryIntoCtx<Endian, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        dst.gwrite_with(self.size, offset, ctx)?;
        try_gwrite_vec_with!(dst, offset, self.data, ctx);

        Ok(*offset)
    }
}

impl<'a, T: 'a> ctx::TryIntoCtx<LuaStringCtx> for Array<T>
where
    T: ctx::TryIntoCtx<LuaStringCtx, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: LuaStringCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        dst.gwrite_with(self.size, offset, ctx.endianess)?;
        try_gwrite_vec_with!(dst, offset, self.data, ctx);

        Ok(*offset)
    }
}