use std::io::Read;

use luasleuth::lua51::disassembler::Disassembler;

fn main() {
    let mut file = std::fs::File::open("data/bytecode/lua51.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let disassembler = Disassembler::new(&buffer);
    let bytecode = disassembler.disassemble();
    match bytecode {
        Ok(bytecode) => println!("{:#?}", bytecode),
        Err(err) => println!("Error: {err}"),
    }
}
