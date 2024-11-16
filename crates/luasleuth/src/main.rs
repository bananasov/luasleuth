use std::{fs::File, io::Read};

use luasleuth_common::disassembler::Disassemble;
use luasleuth_lua53::disassembler::Disassembler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("data/bytecode/lua53.bin")?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let disassembler = Disassembler::new(&bytes);
    let bytecode = disassembler.disassemble()?;

    println!("{:#?}", bytecode);

    Ok(())
}
