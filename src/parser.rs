use core::panicking::panic;

use crate::{tokens::Token, ast::Node};

pub struct Parser {
    tokens: Vec<Token>,
    program: Vec<Node>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            program: vec![],
        }
    }

    fn create_ast(&self) {
        while self.tokens.len() > 0 {
            self.program.push(self.parse_node())
        }
    }

    fn peek(&self, index: usize) -> &Token {
        if index >= self.tokens.len() {
            return &self.tokens[index]
        }

        panic!()
    }

    fn peek_next(&self) -> &Token {
        if self.tokens.len() > 0 {
            return &self.tokens[0]
        }

        panic!()
    }

    fn get(&self) -> Token {
        if self.tokens.len() > 0 {
            return self.tokens.pop().unwrap()
        }

        panic!()
    }

    fn parse_node(&self) -> Node {
        todo!()
    }
}
