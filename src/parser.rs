use crate::error::error;
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

    pub fn create_ast(&mut self) -> Program {
        let mut program: Program = Program::new();

        for t in self.tokens.clone() {
            if t.token_type == TokenType::TokenEof 
                || t.token_type == TokenType::TokenError
            {
                break;
            }

            let expression: Expr = self.parse_statment();
            match expression {
                Expr::ParseErr => break,
                _ => program.body.push(expression),
            }
        }

        program
    }

    fn get_token(&mut self, index: usize) -> Token {
        let token: Token = self.tokens[index].clone();
        self.index += 1;
        token
    }

    fn peek(&mut self, index: usize) -> Token {
        self.tokens[index].clone()
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
        self.parse_primary_expression()
    }

    fn parse_additive_expression(&mut self) -> Expr {
        // This is an expensive operation but it is a workaround for
        // initializing an empty BinaryExpr.
        let mut b_expr = vec![];
        let left = self.parse_primary_expression();
        
        while self.peek(0).literal == "plus"
            || self.peek(0).literal == "minus"
        {
            let operator_token = self.get_token(self.index as usize);
            let right = self.parse_primary_expression();
            b_expr[0] = BinaryExpr::new(Box::new(left.clone()), 
                Box::new(right.clone()), 
                operator_token.literal);
        }

        Expr::BinaryExpr(b_expr[0].clone())
    }

    fn parse_primary_expression(&mut self) -> Expr {
        let token: Token = self.get_token(self.index as usize).clone();

        match token.token_type {
            TokenType::TokenIdentifier => return Expr::Identifier(Identifier::new(token.literal)),
            TokenType::TokenNumber => return Expr::Number(Number::new(token.literal.parse::<f32>().unwrap())),
            _ => error("Failed to parse token", None, Some(&token.literal)),
        }

        unreachable!()
    }
}