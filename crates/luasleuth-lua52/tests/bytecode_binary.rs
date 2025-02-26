use luasleuth_common::disassembler::Disassemble as _;
use luasleuth_lua52::disassembler::Disassembler;

#[test]
fn test_can_parse_bytecode_file() {
    let bytes = include_bytes!("../../../data/bytecode/lua52.bin");
    let bytecode = Disassembler::new(bytes)
        .disassemble()
        .expect("Failed to read bytecode data");

    // Check if most important parts of the disassembly is valid
    assert_eq!(bytecode.header.size_of_size_t, 8);
    assert_eq!(bytecode.header.version.into_tuple(), (5, 2));

    // Check prototype related values
    let prototype = bytecode.prototype;
    assert_eq!(prototype.debug_info.source, "@.\\example.lua"); // Checks size and actual data
    assert_eq!(prototype.line_defined, 0);
    assert_eq!(prototype.last_line_defined, 0);
    assert_eq!(prototype.number_of_parameters, 0);
    assert_eq!(prototype.is_vararg, 1);
    assert_eq!(prototype.max_stack_size, 2);
}
