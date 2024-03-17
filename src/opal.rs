use std::collections::VecDeque;
use std::fs;

use crate::parser::Parser;
use crate::tokens::Token;
use crate::lexer::Lexer;

fn read_file(path: String) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn opalc(files: VecDeque<String>) {
    for x in files {
        let mut source: String = read_file(x.clone());
        source.push('\0');

        let mut lexer: Lexer = Lexer::new(source);
        let tokens: Vec<Token> = lexer.tokenize();

        for x in &tokens {
            println!("{} | {:#?} | {}", &x.literal, &x.token_type, &x.line)
        }

        let mut parser = Parser::new(tokens.into());
        let program = parser.create_ast();

        for x in &program.body {
            println!("{:#?}", x);
        }
    }
}
