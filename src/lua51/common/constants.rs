use scroll::{ctx, Pread};

use crate::common::string::{LuaString, LuaStringCtx};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Constant<'b> {
    Nil,
    Boolean(bool),
    Number(f64),
    String(LuaString<'b>),
}

impl<'a> ctx::TryFromCtx<'a, LuaStringCtx> for Constant<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: LuaStringCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let constant_type: u8 = src.gread_with(offset, ctx.endianess)?;
        let constant = match constant_type {
            0 => Constant::Nil,
            1 => {
                let value: u8 = src.gread_with(offset, ctx.endianess)?;
                Constant::Boolean(value != 0)
            }
            3 => {
                let value: f64 = src.gread_with(offset, ctx.endianess)?;
                Constant::Number(value)
            }
            4 => {
                let str: LuaString = src.gread_with(offset, ctx)?;
                Constant::String(str)
            }
            _ => unreachable!("Somehow got an invalid constant type"),
        };

        Ok((constant, *offset))
    }
}