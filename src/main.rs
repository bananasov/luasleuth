use std::io::Read;

use luasleuth::lua51::common::Header;
use scroll::{Pread, LE};

fn main() {
    let mut file = std::fs::File::open("data/bytecode/lua51.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let offset = &mut 0;
    let header: Header = buffer.gread_with(offset, LE).unwrap();
    println!("{:#?}", header);
}
