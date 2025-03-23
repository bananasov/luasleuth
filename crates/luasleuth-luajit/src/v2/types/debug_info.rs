use scroll::{ctx, Pread};

use crate::common::ctx::BytecodeContext;
use luasleuth_common::types::leb128::Uleb128;

#[derive(Debug)]
pub struct DebugInfoMetadata {
    pub size: Uleb128,
    pub first_line: Uleb128,
    pub num_lines: Uleb128,
}

#[derive(Debug)]
pub struct DebugInfo {}

impl<'a> ctx::TryFromCtx<'a, BytecodeContext> for DebugInfoMetadata {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: BytecodeContext) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        if ctx.is_stripped() {
            return Ok((
                Self {
                    size: Uleb128::from(0u64),
                    first_line: Uleb128::from(0u64),
                    num_lines: Uleb128::from(0u64),
                },
                *offset,
            ));
        }

        let size: Uleb128 = src.gread_with(offset, ())?;
        let first_line: Uleb128 = src.gread_with(offset, ())?;
        let num_lines: Uleb128 = src.gread_with(offset, ())?;

        Ok((
            Self {
                size,
                first_line,
                num_lines,
            },
            *offset,
        ))
    }
}

impl<'a> ctx::TryFromCtx<'a, BytecodeContext> for DebugInfo {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: BytecodeContext) -> Result<(Self, usize), Self::Error> {
        todo!()
    }
}
