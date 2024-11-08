use scroll::{Endian, Pread};

use crate::common::string::{LuaString, LuaStringCtx};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Constant {
    LUA_TNIL,
    LUA_TBOOLEAN(bool),
    LUA_TNUMBER(f64),
    LUA_TSTRING(String),
}

impl<'a> Constant {
    pub fn decode(
        src: &'a [u8],
        offset: &mut usize,
        size_of_sizet: u8,
        endian: Endian,
    ) -> Result<Constant, scroll::Error> {
        let const_type: u8 = src.gread_with(offset, endian)?;
        let constant = match const_type {
            0 => Constant::LUA_TNIL,
            1 => {
                let value: u8 = src.gread_with(offset, endian)?;
                Constant::LUA_TBOOLEAN(value != 0)
            }
            3 => {
                let value: f64 = src.gread_with(offset, endian)?;
                Constant::LUA_TNUMBER(value)
            }
            4 => {
                let str: LuaString = src.gread_with(
                    offset,
                    LuaStringCtx {
                        endianess: endian,
                        size_of_sizet,
                    },
                )?;
                Constant::LUA_TSTRING(str.into_string())
            }
            _ => unreachable!("Somehow got an invalid constant type"),
        };

        Ok(constant)
    }
}
