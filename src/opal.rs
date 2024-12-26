use std::collections::VecDeque;
use std::fs;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::vm::OVM;
use crate::tokens::Token;

fn read_file(path: String) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn opalc(files: VecDeque<String>) {
    // Loop through all the files provided by the user
    for x in files {
        // Reads the entire file into a string and appends a null-byte
        let mut source: String = read_file(x.clone());
        source.push('\0');

        // Tokenize the source code
        let mut lexer: Lexer = Lexer::new(source);
        let tokens: Vec<Token> = lexer.tokenize();

        /* Uncomment for debugging the tokenizer
        for x in &tokens {
            println!("{} | {:#?} | {}", &x.literal, &x.token_type, &x.line)
        }
        */

        let mut parser = Parser::new(tokens.into());
        let program = parser.create_ast();

        // Uncomment for debugging the parser
        for x in &program {
            println!("{:#?}", x);
        }

        // Execute the program by walking through the AST
        let mut vm = OVM::new(program);
        vm.execute();
    }
}
