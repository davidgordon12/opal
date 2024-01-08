# Opal

A(n in-progress) handwritten Lexer, Parser and Compiler written in Rust.

## Features
* [x] In-Depth error logging
* [x] Single arithmetic equations
* [x] Complex binary expressions
* [ ] Floating Point arithmetic
* [ ] Variable declerations
* [ ] Function Declarations
* [ ] Basic standard library

## Build
This project relies on Cargo to build. Please ensure you have it installed.
```bash
cargo run <file1.opal> <file2.opal> ...
```

## Assembling and Linking
This project generates NASM x86_64 Intel assembly, and is intended to be used with the GNU Linker.
```bash
nasm -f elf64 <file.asm> -o <output.o>
```
```bash
ld <output.o> -o <executable>
```

You should now have an executable file in a 64-bit ELF format.

## Todo
* [ ] Package up all code generation files into a single module
* [ ] Cleanup undescriptive variable names like 'x' wherever they are
* [ ] Remove any redundant clones, copies and borrows
* [ ] Any other small refactors I find along the way :)
