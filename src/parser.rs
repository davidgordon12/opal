use crate::error;
use crate::tokens::*;
use crate::ast::*;

pub struct Parser {
    index: i32,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            index: 0,
            tokens: tokens,
        }
    }

    fn get_token(&self, index: usize) -> &Token {
        &self.tokens[index]
    }

    fn parse_statment(&mut self) -> Stmt {
        Stmt::Expr(self.parse_expression())
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_primary_expression()
    }

    fn parse_primary_expression(&mut self) -> Expr {
        let token: Token = self.get_token(self.index as usize).clone();
        self.index += 1;

        match token.token_type {
            TokenType::TokenIdentifier => return Expr::Identifier(Identifier::new(token.literal)),
            _ => parse_error("Failed to parse token", &token),
        }
    }

    pub fn create_ast(&mut self) -> Program {
        let mut program: Program = Program::new();

        for t in self.tokens.clone() {
            if t.token_type == TokenType::TokenEof 
                || t.token_type == TokenType::TokenError
            {
                break;
            }

            match self.parse_statment() {
                Stmt::Expr(Expr::ParseErr) => break,
                _ => continue
            }
        }

        program
    }
}

fn parse_error(message: &str, token: &Token) -> Expr {
    println!("Opal: \x1b[91mFatal Error\x1b[0m");
    println!("{}: \x1b[93m{}\x1b[0m", message, token.literal);
    Expr::ParseErr
}