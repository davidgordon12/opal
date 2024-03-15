use core::panic;
use std::collections::VecDeque;

use crate::{tokens::Token, tokens::TokenType, ast::*};

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Parser {
            tokens: tokens,
        }
    }

    pub fn create_ast(&mut self) -> Program {
        let mut program = Program::new();

        while !self.eof() {
            let node = self.parse_node();
            program.body.push(node);
        }

        program
    }

    fn peek(&self, index: usize) -> &Token {
        if index < self.tokens.len() && index >= 0 {
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

    fn get_token(&mut self) -> Token {
        if !self.eof() {
            return self.tokens.pop_front().unwrap()
        }

        panic!()
    }

    fn eof(&self) -> bool {
        self.peek_next().token_type == TokenType::TokenEof
    }

    fn parse_node(&mut self) -> Node {
        match self.peek_next().token_type {
            TokenType::TokenProc => return Node::ProcDeclaration(self.parse_procedure()),
            TokenType::TokenLet => return Node::LetDeclaration(self.parse_variable()),
            _ => return self.parse_addition(),
        }
    }

    fn parse_procedure(&mut self) -> ProcDeclaration {
        self.get_token();

        let ident = self.get_token();

        if ident.token_type != TokenType::TokenIdentifier {
            panic!()
        }

        let mut proc = ProcDeclaration::new(ident.literal);

        if self.get_token().token_type != TokenType::TokenLeftParen {
            panic!()
        }

        while self.get_token().token_type != TokenType::TokenRightParen {
            if self.peek_next().token_type == TokenType::TokenRightParen { 
                self.get_token();
                break;
            }
        }

        if self.get_token().token_type != TokenType::TokenLeftBrace {
            panic!();
        }

        while self.peek_next().token_type != TokenType::TokenRightBrace {
            let node = self.parse_node();
            proc.body.push(node);
        }

        if self.get_token().token_type != TokenType::TokenRightBrace {
            panic!()
        }

        proc
    }

    fn parse_variable(&mut self) -> LetDeclaration {
        self.get_token();

        let ident = self.get_token();

        if ident.token_type != TokenType::TokenIdentifier {
            panic!()
        }

        if self.get_token().token_type != TokenType::TokenEqual {
            panic!()
        }

        let value = self.parse_node();

        if self.get_token().token_type != TokenType::TokenSemicolon {
            panic!()
        }

        LetDeclaration::new(ident.literal, Box::from(value))
    }

    fn parse_primary(&mut self) -> Node {
        let token = self.get_token();
        match token.token_type {
            TokenType::TokenNumber => return Node::Number(Number::new(token.literal.parse::<i64>().unwrap())),
            TokenType::TokenString => return Node::OString(OString::new(token.literal)),
            TokenType::TokenFloat => return Node::Float(Float::new(token.literal.parse::<f64>().unwrap())),
            TokenType::TokenIdentifier => return Node::Identifier(Identifier::new(token.literal)),
            _ => panic!(),
        }
    }

    fn parse_addition(&mut self) -> Node {
        let mut left = self.parse_primary();

        while self.peek_next().token_type == TokenType::TokenPlus
            || self.peek_next().token_type == TokenType::TokenMinus 
        {
            let op = self.get_token();
            let right = self.parse_primary();
            left = Node::BinaryExpr(BinaryExpr::new(Box::from(left), Box::from(right), 
                op.literal.as_bytes()[0 as usize] as char))
        }

        left
    }
}
