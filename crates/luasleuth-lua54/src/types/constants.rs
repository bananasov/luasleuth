use luasleuth_common::{types::LuaString, CommonCtx};
use scroll::{ctx, Pread, Pwrite};

const LUAI_MAXSHORTLEN: usize = 40;

#[derive(Debug)]
pub enum Constant<'a> {
    Nil,
    Boolean(bool),
    Float(f64),
    Integer(i64),
    String(LuaString<'a>),
}

impl Constant<'_> {
    /// Return the type of the constant in the Lua 5.1 bytecode format.
    ///
    /// The values returned are the following:
    ///
    /// * `0` for `Nil` constants
    /// * `1` for `Boolean` constants
    /// * `3` for `Float` constants
    /// * `19` for `Integer` constants
    /// * `4` for short `String` constants
    /// * `20` for long `String` constants
    pub fn get_type(&self) -> u8 {
        match self {
            Constant::Nil => 0,
            Constant::Boolean(bool) => match bool {
                true => 17,
                false => 1,
            },
            Constant::Float(_) => 19,
            Constant::Integer(_) => 3,
            Constant::String(string) => {
                if string.size >= LUAI_MAXSHORTLEN {
                    20
                } else {
                    4
                }
            }
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
            1 => Constant::Boolean(false),
            17 => Constant::Boolean(true),
            19 => Constant::Float(src.gread_with(offset, ctx.endianness)?),
            3 => Constant::Integer(src.gread_with(offset, ctx.endianness)?),
            4 | 20 => Constant::String(src.gread_with(offset, ctx)?),
            _ => {
                return Err(scroll::Error::BadInput {
                    size: 1,
                    msg: "Invalid constant type",
                })
            }
        };

        Ok((constant, *offset))
    }
}

impl ctx::TryIntoCtx<CommonCtx> for Constant<'_> {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: CommonCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        let tag = self.get_type();
        dst.gwrite_with(tag, offset, ctx.endianness)?;

        match self {
            Constant::Nil => {}
            Constant::Boolean(_) => {} // `true` and `false` are different constant types in Lua 5.4, this is handled by `Constant::get_type`
            Constant::Float(value) => {
                dst.gwrite_with(value, offset, ctx.endianness)?;
            }
            Constant::Integer(value) => {
                dst.gwrite_with(value, offset, ctx.endianness)?;
            }
            Constant::String(value) => {
                dst.gwrite_with(value, offset, ctx)?;
            }
        };

        Ok(*offset)
    }
}
