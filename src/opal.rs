use std::collections::VecDeque;
use std::fs;

use crate::tokens::Token;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;

fn read_file(path: String) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn opalc(files: VecDeque<String>) {
    for x in files {
        let source: String = read_file(x);

        let mut lexer: Lexer = Lexer::new(source);
        let tokens: Vec<Token> = lexer.tokenize();

        let parser: Parser = Parser::new(tokens);

        let compiler: Compiler = Compiler {};
        compiler.run();   
    }
}