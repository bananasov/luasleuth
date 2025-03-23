use crate::common::ctx::BytecodeContext;
use luasleuth_common::{try_gread_vec_with, try_gwrite_vec_with, types::leb128::Uleb128};
use scroll::{ctx, Pread, Pwrite};

#[derive(Debug)]
pub struct Array<T> {
    pub size: usize,
    pub data: Vec<T>,
}

impl<T> Array<T> {
    pub fn read_size(src: &[u8], offset: &mut usize) -> Result<usize, scroll::Error> {
        let size: Uleb128 = src.gread_with(offset, ())?;

        Ok(size.into())
    }

    pub fn write_size(&self, dst: &mut [u8], offset: &mut usize) -> Result<usize, scroll::Error> {
        let bytes_written = dst.gwrite_with(Uleb128::from(self.size), offset, ())?;
        Ok(bytes_written)
    }
}

impl<T: Clone> From<Array<T>> for Vec<T> {
    fn from(value: Array<T>) -> Self {
        value.data.to_vec()
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

impl ctx::TryFromCtx<'_, BytecodeContext> for Array<i32> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'_ [u8], ctx: BytecodeContext) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let size = Array::<i32>::read_size(src, offset)?; // Is the generic really needed?
        let data: Vec<i32> = try_gread_vec_with!(src, offset, size, ctx.endian);

        Ok((Self { size, data }, *offset))
    }
}

impl ctx::TryIntoCtx<BytecodeContext> for Array<i32> {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: BytecodeContext) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        Self::write_size(&self, dst, offset)?;
        try_gwrite_vec_with!(dst, offset, self.data, ctx.endian);

        Ok(*offset)
    }
}

impl ctx::TryIntoCtx<BytecodeContext> for Array<u8> {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: BytecodeContext) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        Self::write_size(&self, dst, offset)?;
        try_gwrite_vec_with!(dst, offset, self.data, ctx.endian);

        Ok(*offset)
    }
}

impl<'a, T: 'a> ctx::TryFromCtx<'a, BytecodeContext> for Array<T>
where
    T: ctx::TryFromCtx<'a, BytecodeContext, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: BytecodeContext) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let size = Array::<T>::read_size(src, offset)?; // Is the generic really needed?
        let data: Vec<T> = try_gread_vec_with!(src, offset, size, ctx);

        Ok((Array { size, data }, *offset))
    }
}

impl<T> ctx::TryIntoCtx<BytecodeContext> for Array<T>
where
    T: ctx::TryIntoCtx<BytecodeContext, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: BytecodeContext) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        Self::write_size(&self, dst, offset)?;
        try_gwrite_vec_with!(dst, offset, self.data, ctx);

        Ok(*offset)
    }
}
