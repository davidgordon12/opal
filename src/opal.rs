use std::collections::VecDeque;
use std::fs;

use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::tokens::Token;

fn read_file(path: String) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn opalc(files: VecDeque<String>) {
    for x in files {
        let source: String = read_file(x);

        let mut lexer: Lexer = Lexer::new(source);
        let tokens: Vec<Token> = lexer.tokenize();

        for x in tokens {
            println!("{:#?} : {:#?} : {}", x.literal, x.token_type, x.line);
        }

        // parser

        let compiler: Compiler = Compiler {};
        compiler.run();   
    }
}