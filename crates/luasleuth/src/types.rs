use clap::ValueEnum;


#[derive(Debug, Clone, ValueEnum)]
pub enum LuaVersion {
    Lua51,
    Lua52,
    Lua53,
    Lua54,
    Luajitv1,
    Luajitv2,
}