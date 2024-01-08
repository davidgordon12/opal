use std::collections::VecDeque;

use crate::error::error;
use crate::error::parse_token_error;
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

    fn parse_statment(&mut self) -> Stmt {
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

    fn parse_expression(&mut self) -> Stmt {
        self.parse_additive_expression()
    }

    fn parse_additive_expression(&mut self) -> Stmt {
        let mut left = self.parse_multiplicative_expression();
        
        while self.peek().token_type == TokenType::TokenPlus
            || self.peek().token_type == TokenType::TokenMinus
        {
            let operator_token = self.get_token();
            let right = self.parse_multiplicative_expression();
            left = Stmt::BinaryExpr(BinaryExpr::new(Box::new(left.clone()), 
                Box::new(right.clone()), 
                operator_token.literal.as_bytes()[0] as char));
        }
    
        left
    }

    fn parse_multiplicative_expression(&mut self) -> Stmt {
        let mut left = self.parse_power_expression();
        
        while self.peek().token_type == TokenType::TokenStar
            || self.peek().token_type == TokenType::TokenSlash
            || self.peek().token_type == TokenType::TokenModulo
        {
            let operator_token = self.get_token();
            let right = self.parse_primary_expression();
            left = Stmt::BinaryExpr(BinaryExpr::new(Box::new(left.clone()), 
                Box::new(right.clone()), 
                operator_token.literal.as_bytes()[0] as char));
        }
    
        left
    }

    fn parse_power_expression(&mut self) -> Stmt {
        let mut left = self.parse_primary_expression();
        
        while self.peek().token_type == TokenType::TokenPower {
            let operator_token = self.get_token();
            let right = self.parse_primary_expression();
            left = Stmt::BinaryExpr(BinaryExpr::new(Box::new(left.clone()), 
                Box::new(right.clone()), 
                operator_token.literal.as_bytes()[0] as char));
        }
    
        left
    }

    fn parse_primary_expression(&mut self) -> Stmt {
        let token: Token = self.get_token();

        match token.token_type {
            TokenType::TokenIdentifier => return Stmt::Identifier(Identifier::new(token.literal)),
            TokenType::TokenNumber => return Stmt::Number(Number::new(token.literal.parse::<i64>().unwrap())),
            TokenType::TokenFloat => return Stmt::Float(Float::new(token.literal.parse::<f64>().unwrap())),
            TokenType::TokenNull => return Stmt::NullLiteral(NullLiteral::new()),
            TokenType::TokenLeftParen => {
                let val = self.parse_expression();
                let expected = self.get_token(); // Closing parenthesis
                if expected.token_type != TokenType::TokenRightParen {
                    error("Unclosed parenthesis. '(' found on line", Some(&token.line.to_string()))
                }
                return val;
            },
            _ => parse_token_error("Failed to parse token", &token.literal, &token.line.to_string()),
        }

        unreachable!()
    }
}
