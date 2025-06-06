use luasleuth_common::{
    types::{Array, LuaString},
    CommonCtx,
};
use scroll::{ctx, Pread, Pwrite};

#[derive(Debug)]
pub struct DebugInfo<'a> {
    /// Map from opcodes to source lines
    pub line_info: Array<i32>,
    /// Information about local variables
    pub local_variables: Array<LocalVariable<'a>>,
    /// Upvalue names
    pub upvalues: Array<LuaString<'a>>,
}

#[derive(Debug)]
pub struct LocalVariable<'a> {
    pub name: LuaString<'a>,
    /// First point where variable is active
    pub start_pc: i32,
    /// First point where variable is dead
    pub end_pc: i32,
}

impl<'a> ctx::TryFromCtx<'a, CommonCtx> for DebugInfo<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let line_info: Array<i32> = src.gread_with(offset, ctx)?;
        let local_variables: Array<LocalVariable> = src.gread_with(offset, ctx)?;
        let upvalues: Array<LuaString> = src.gread_with(offset, ctx)?;

        Ok((
            DebugInfo {
                line_info,
                local_variables,
                upvalues,
            },
            *offset,
        ))
    }
}

impl ctx::TryIntoCtx<CommonCtx> for DebugInfo<'_> {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: CommonCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        dst.gwrite_with(self.line_info, offset, ctx)?;
        dst.gwrite_with(self.local_variables, offset, ctx)?;
        dst.gwrite_with(self.upvalues, offset, ctx)?;

        Ok(*offset)
    }
}

impl<'a> ctx::TryFromCtx<'a, CommonCtx> for LocalVariable<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: CommonCtx) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let name: LuaString = src.gread_with(offset, ctx)?;
        let start_pc: i32 = src.gread_with(offset, ctx.endianness)?;
        let end_pc: i32 = src.gread_with(offset, ctx.endianness)?;

        Ok((
            LocalVariable {
                name,
                start_pc,
                end_pc,
            },
            *offset,
        ))
    }
}

impl ctx::TryIntoCtx<CommonCtx> for LocalVariable<'_> {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: CommonCtx) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        dst.gwrite_with(self.name, offset, ctx)?;
        dst.gwrite_with(self.start_pc, offset, ctx.endianness)?;
        dst.gwrite_with(self.end_pc, offset, ctx.endianness)?;

        Ok(*offset)
    }
}
