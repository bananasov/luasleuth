use scroll::{ctx, Pread, Pwrite};
use luasleuth_common::{types::LuaString, CommonCtx};

#[derive(Debug)]
pub enum Constant<'a> {
    Nil,
    Boolean(bool),
    Number(f64),
    String(LuaString<'a>),
}

impl Constant<'_> {
    /// Return the type of the constant in the Lua 5.1 bytecode format.
    ///
    /// The values returned are the following:
    ///
    /// * `0` for `Nil` constants
    /// * `1` for `Boolean` constants
    /// * `3` for `Number` constants
    /// * `4` for `String` constants
    pub fn get_type(&self) -> u8 {
        match self {
            Constant::Nil => 0,
            Constant::Boolean(_) => 1,
            Constant::Number(_) => 3,
            Constant::String(_) => 4,
        }
    }
}

impl<'a> ctx::TryFromCtx<'a, CommonCtx> for Constant<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let tag: u8 = src.gread_with(offset, ctx.endianness)?;

        let constant = match tag {
            0 => Constant::Nil,
            1 => {
                let value: u8 = src.gread_with(offset, ctx.endianness)?;
                Constant::Boolean(value != 0)
            },
            3 => Constant::Number(src.gread_with(offset, ctx.endianness)?),
            4 => Constant::String(src.gread_with(offset, ctx)?),
            _ => return Err(scroll::Error::BadInput { size: 1, msg: "Invalid constant type" }),
        };

        Ok((constant, *offset))
    }
}

impl ctx::TryIntoCtx<CommonCtx> for Constant<'_> {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: CommonCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        let constant = self.get_type();
        dst.gwrite_with(constant, offset, ctx.endianness)?;

        match self {
            Constant::Nil => {},
            Constant::Boolean(value) => { dst.gwrite_with(value as u8, offset, ctx.endianness)?; },
            Constant::Number(value) => { dst.gwrite_with(value, offset, ctx.endianness)?; },
            Constant::String(value) => { dst.gwrite_with(value, offset, ctx)?; },
        };

        Ok(*offset)
    }
}