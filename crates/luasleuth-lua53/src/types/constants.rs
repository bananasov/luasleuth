use luasleuth_common::{CommonCtx, types::LuaString};
use scroll::{ctx, Pread};

#[derive(Debug)]
pub enum Constant<'a> {
    Nil,
    Boolean(bool),
    Float(f64),
    Integer(i64),
    String(LuaString<'a>),
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
            }
            3 => Constant::Float(src.gread_with(offset, ctx.endianness)?),
            19 => Constant::Integer(src.gread_with(offset, ctx.endianness)?),
            4 | 20 => Constant::String(src.gread_with(offset, ctx)?),
            _ => return Err(scroll::Error::BadInput { size: 1, msg: "Invalid constant type" }),
        };

        Ok((constant, *offset))
    }
}

// TODO: Implement Lua 5.3 constant writing.
// NOTE: To implement Lua 5.3 constant writing, we'd have to distinguish between a
//       long string and a short string
