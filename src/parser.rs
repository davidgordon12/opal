use std::collections::VecDeque;

use crate::error::error;
use crate::tokens::*;
use crate::ast::*;

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Parser {
        Parser {
            tokens: tokens,
        }
    }

    pub fn create_ast(&mut self) -> Program {
        let mut program: Program = Program::new();

        while !self.eof() {
            program.body.push(self.parse_statment());
        }

        program
    }

    fn eof(&mut self) -> bool {
        self.tokens[0].token_type == TokenType::TokenEof
    }

    fn get_token(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
    }

    fn peek(&mut self) -> Token {
        self.tokens[0].clone()
    }

    fn parse_statment(&mut self) -> Expr {
        self.parse_expression()
    }

    /* PRECEDENCE
        Assignment,
        Member,
        Function,
        Logical,
        Comparison,
        Primary,
        Unary,
        Multiplication,
        Additive,
    */

    fn parse_expression(&mut self) -> Expr {
        self.parse_additive_expression()
    }

    fn parse_additive_expression(&mut self) -> Expr {
        let mut left = self.parse_primary_expression();
        
        while self.peek().literal == "plus"
            || self.peek().literal == "minus"
        {
            let operator_token = self.get_token();
            let right = self.parse_primary_expression();
            left = Expr::BinaryExpr(BinaryExpr::new(Box::new(left.clone()), 
                Box::new(right.clone()), 
                operator_token.literal));
        }
    
        left
    }

    fn parse_primary_expression(&mut self) -> Expr {
        let token: Token = self.get_token().clone();

        match token.token_type {
            TokenType::TokenIdentifier => return Expr::Identifier(Identifier::new(token.literal)),
            TokenType::TokenNumber => return Expr::Number(Number::new(token.literal.parse::<f32>().unwrap())),
            _ => error("Failed to parse token", None, None, Some(&token.literal)),
        }

        unreachable!()
    }
}
