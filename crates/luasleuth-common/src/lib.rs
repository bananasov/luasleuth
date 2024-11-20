pub mod types;
#[macro_use]
mod macros;
pub mod assembler;
pub mod disassembler;

use scroll::{ctx, Endian, Pread, Pwrite};

#[derive(Debug, Copy, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

/// A common context used for reading and writing within Scroll.
#[derive(Copy, Clone)]
pub struct CommonCtx {
    /// The size of a `size_t` in bytes, used for string reading.
    ///
    /// When reading a string, the size is encoded in a `size_t` value.
    pub size_of_size_t: u8,

    /// The version of the Lua compiler.
    ///
    /// Because string reading differs between versions, this is required to decide how to read strings.
    ///
    /// Note that this assumes the format is official.
    pub lua_version: Version,

    /// The endianness of the file.
    ///
    /// Used when reading integers and alike.
    pub endianness: Endian,
}

impl Version {
    /// Returns the major and minor version as a tuple.
    ///
    /// Used for easily matching version numbers.
    pub fn into_tuple(self) -> (u8, u8) {
        (self.major, self.minor)
    }
}

impl<'a> ctx::TryFromCtx<'a, Endian> for Version {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let version: u8 = src.gread_with(offset, ctx)?;

        let major = version >> 4;
        let minor = version & 0xF;

        Ok((Version { major, minor }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Version {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        let version = self.major << 4 | self.minor;
        dst.gwrite_with(version, offset, ctx)?;

        Ok(*offset)
    }
}
