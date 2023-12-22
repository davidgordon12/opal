use crate::tokens::*;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
        }
    }

    pub fn create_ast() {
        todo!()
    }
}