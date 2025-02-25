# luasleuth
A versatile Lua disassembler written in Rust, supporting the most used Lua 5.x versions.

> [!NOTE]
> This project is currently really basic, the cli is lacking and honestly it can't do much except just printing a struct.

## Features

- Support for most 5.x versions (5.1 - 5.4)
- Cross-platform compatibility
- Written in Rust for fun and masochism

## Installation

### Building from source

```bash
git clone https://github.com/bananasov/luasleuth
cd luasleuth
cargo build --release
```

The compiled binary will be available in `target/release/luasleuth`

## Usage
```bash
Usage: luasleuth.exe disassemble --path <PATH> --version <VERSION>

Options:
  -p, --path <PATH>
  -v, --version <VERSION>  [possible values: lua51, lua52, lua53, lua54, luajitv1, luajitv2]
  -h, --help               Print help
```

### Example usage
```bash
luasleuth disassemble --path .\data\bytecode\lua54.bin --version lua54
```
