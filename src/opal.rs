<<<<<<< HEAD
use std::collections::VecDeque;
use std::fs;

use crate::tokens::Token;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::Program;
use crate::compiler::Compiler;

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

        let mut parser: Parser = Parser::new(tokens.into());
        let program: Program = parser.create_ast();

        println!("{:#?}", program);
        let mut compiler: Compiler = Compiler::new(x.clone(), program);
        compiler.run();   
    }
}
