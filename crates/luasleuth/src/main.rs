mod types;

use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use luasleuth_common::disassembler::Disassemble;

use luasleuth_lua51::disassembler::Disassembler as Lua51Disassembler;
use luasleuth_lua52::disassembler::Disassembler as Lua52Disassembler;
use luasleuth_lua53::disassembler::Disassembler as Lua53Disassembler;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand
}

#[derive(Debug, Parser)]
enum Subcommand {
    Disassemble {
        #[clap(short, long)]
        path: PathBuf,

        #[clap(short, long)]
        version: types::LuaVersion
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.subcommand {
        Subcommand::Disassemble { path, version } => match version {
            types::LuaVersion::Lua51 => {
                let mut file = File::open(path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;

                let bytecode = Lua51Disassembler::new(&buffer).disassemble()?;
                println!("{:#?}", bytecode);
            },
            types::LuaVersion::Lua52 => {
                let mut file = File::open(path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;

                let bytecode = Lua52Disassembler::new(&buffer).disassemble()?;
                println!("{:#?}", bytecode);
            },
            types::LuaVersion::Lua53 => {
                let mut file = File::open(path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;

                let bytecode = Lua53Disassembler::new(&buffer).disassemble()?;
                println!("{:#?}", bytecode);
            },
            _ => todo!(),
        }
    };

    Ok(())
}
