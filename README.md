# Opal

A(n in-progress) handwritten Lexer, Parser and VM written in Rust.

## Features
* [ ] In-Depth error logging
* [x] Single arithmetic equations
* [x] Complex binary expressions
* [x] Floating Point arithmetic
* [x] Variable declerations
* [x] Function Declarations
* [ ] Basic standard library

## Build
This project relies on Cargo to build. Please ensure you have it installed.
```bash
cargo run <file1.opal> <file2.opal> ...
```

## Todo
* [x] Package up all code generation files into a single module
* [ ] Cleanup undescriptive variable names like 'x' wherever they are
* [ ] Remove any redundant clones, copies and borrows
* [ ] Any other small refactors I find along the way :)
