use core::panic;
use std::collections::VecDeque;

use crate::{
    ast::*,
    opal_error_parser_invalid_expr, opal_error_parser_oob, opal_error_parser_unexpected_token,
    tokens::{Token, TokenType},
};

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Parser { tokens: tokens }
    }

    pub fn create_ast(&mut self) -> Vec<Node> {
        let mut program = Vec::new();

        while !self.eof() {
            let node = self.parse_node();
            program.push(node);
        }

        program
    }

    fn _peek(&self, index: usize) -> &Token {
        if index < self.tokens.len() {
            return &self.tokens[index];
        }

        opal_error_parser_oob();
        unreachable!();
    }

    fn peek_next(&self) -> &Token {
        if self.tokens.len() > 0 {
            return &self.tokens[0];
        }

        opal_error_parser_oob();
        unreachable!();
    }

    fn get_token(&mut self) -> Token {
        if !self.eof() {
            return self.tokens.pop_front().unwrap();
        }

        opal_error_parser_oob();
        unreachable!();
    }

    fn eof(&self) -> bool {
        self.peek_next().token_type == TokenType::TokenEof
    }

    fn parse_node(&mut self) -> Node {
        match self.peek_next().token_type {
            TokenType::TokenProc => return Node::ProcDeclaration(self.parse_procedure()),
            TokenType::TokenLet => return Node::LetDeclaration(self.parse_let()),
            _ => return self.parse_statement(),
        }
    }

    fn parse_expression(&mut self) -> Node {
        match self.peek_next().token_type {
            _ => return self.parse_comparison(),
        }
    }

    fn parse_procedure(&mut self) -> ProcDeclaration {
        self.get_token();

        let ident = self.get_token();

        if ident.token_type != TokenType::TokenIdentifier {
            panic!()
        }

        let mut proc = ProcDeclaration::new(ident.literal, TokenType::TokenVoid);

        let mut tkn = self.get_token();
        if tkn.token_type != TokenType::TokenLeftParen {
            opal_error_parser_unexpected_token('('.to_string(), tkn.literal, tkn.line);
            unreachable!();
        }

        while self.get_token().token_type != TokenType::TokenRightParen {
            if self.peek_next().token_type == TokenType::TokenRightParen {
                self.get_token();
                break;
            }
        }

        if self.peek_next().token_type == TokenType::TokenArrow {
            self.get_token();
            let ret_value = self.get_token();
            match ret_value.token_type {
                TokenType::TokenNumber => proc.ret_value = ret_value.token_type,
                TokenType::TokenFloat => proc.ret_value = ret_value.token_type,
                TokenType::TokenString => proc.ret_value = ret_value.token_type,
                _ => {
                    opal_error_parser_unexpected_token(
                        "Number, Float, String or no return type at all".to_string(),
                        ret_value.literal,
                        ret_value.line,
                    );
                    unreachable!()
                }
            }
        }

        tkn = self.get_token();
        if tkn.token_type != TokenType::TokenLeftBrace {
            opal_error_parser_unexpected_token("{".to_string(), tkn.literal, tkn.line);
        }

        while self.peek_next().token_type != TokenType::TokenRightBrace {
            let node = self.parse_node();
            proc.body.push(node);
        }

        tkn = self.get_token();
        if tkn.token_type != TokenType::TokenRightBrace {
            opal_error_parser_unexpected_token("}".to_string(), tkn.literal, tkn.line);
        }

        proc
    }

    fn parse_let(&mut self) -> LetDeclaration {
        self.get_token();

        let ident = self.get_token();

        if ident.token_type != TokenType::TokenIdentifier {
            opal_error_parser_unexpected_token(
                "Identifier".to_string(),
                ident.literal.clone(),
                ident.line,
            );
        }

        let mut tkn = self.get_token();
        if tkn.token_type != TokenType::TokenEqual {
            opal_error_parser_unexpected_token("=".to_string(), tkn.literal, tkn.line);
        }

        let value = self.parse_node();

        tkn = self.get_token();
        if tkn.token_type != TokenType::TokenSemicolon {
            opal_error_parser_unexpected_token(";".to_string(), tkn.literal, tkn.line);
        }

        LetDeclaration::new(ident.literal, Box::from(value))
    }

    fn parse_statement(&mut self) -> Node {
        match self.peek_next().token_type {
            TokenType::TokenReturn => {
                self.get_token();
                if self.peek_next().token_type == TokenType::TokenSemicolon {
                    return Node::ReturnStatement(ReturnStatement::new(None));
                }
                let value = self.parse_expression();
                self.get_token();
                return Node::ReturnStatement(ReturnStatement::new(Some(Box::from(value))));
            }
            TokenType::TokenPrint => {
                self.get_token();
                let value = self.parse_expression();
                self.get_token();
                return Node::PrintStatement(PrintStatement::new(Box::from(value)));
            }
            TokenType::TokenIf => {
                self.get_token();
                self.parse_if()
            }
            _ => return self.parse_expression(),
        }
    }

    fn parse_comparison(&mut self) -> Node {
        let mut left = self.parse_addition();

        while self.peek_next().token_type == TokenType::TokenGreater
            || self.peek_next().token_type == TokenType::TokenGreaterEqual
            || self.peek_next().token_type == TokenType::TokenLess
            || self.peek_next().token_type == TokenType::TokenLessEqual
            || self.peek_next().token_type == TokenType::TokenEqualEqual
            || self.peek_next().token_type == TokenType::TokenBangEqual
        {
            let op = self.get_token();
            let right = self.parse_multiplication();
            left = Node::BinaryExpr(BinaryExpr::new(
                Box::from(left),
                Box::from(right),
                op.literal,
            ))
        }

        left
    }

    fn parse_addition(&mut self) -> Node {
        let mut left = self.parse_multiplication();

        while self.peek_next().token_type == TokenType::TokenPlus
            || self.peek_next().token_type == TokenType::TokenMinus
        {
            let op = self.get_token();
            let right = self.parse_multiplication();
            left = Node::BinaryExpr(BinaryExpr::new(
                Box::from(left),
                Box::from(right),
                op.literal,
            ))
        }

        left
    }

    fn parse_multiplication(&mut self) -> Node {
        let mut left = self.parse_exponent();

        while self.peek_next().token_type == TokenType::TokenStar
            || self.peek_next().token_type == TokenType::TokenSlash
            || self.peek_next().token_type == TokenType::TokenModulo
        {
            let op = self.get_token();
            let right = self.parse_exponent();
            left = Node::BinaryExpr(BinaryExpr::new(
                Box::from(left),
                Box::from(right),
                op.literal,
            ))
        }

        left
    }

    fn parse_exponent(&mut self) -> Node {
        let mut left = self.parse_primary();

        while self.peek_next().token_type == TokenType::TokenPower {
            let op = self.get_token();
            let right = self.parse_primary();
            left = Node::BinaryExpr(BinaryExpr::new(
                Box::from(left),
                Box::from(right),
                op.literal,
            ))
        }

        left
    }

    fn parse_primary(&mut self) -> Node {
        let token = self.get_token();
        match token.token_type {
            TokenType::TokenIf => {
                if self.peek_next().token_type == TokenType::TokenLeftParen {
                    return self.parse_if();
                }
                opal_error_parser_unexpected_token(
                    '{'.to_string(),
                    self.peek_next().literal.clone(),
                    self.peek_next().line,
                );
                unreachable!()
            }
            TokenType::TokenNumber => {
                return Node::Number(Number::new(token.literal.parse::<i64>().unwrap()))
            }
            TokenType::TokenFloat => {
                return Node::Float(Float::new(token.literal.parse::<f64>().unwrap()))
            }
            TokenType::TokenString => return Node::OString(OString::new(token.literal)),
            TokenType::TokenTrue => return Node::Boolean(Boolean::new(true)),
            TokenType::TokenFalse => return Node::Boolean(Boolean::new(false)),
            TokenType::TokenIdentifier => {
                let ident = Identifier::new(token.literal);

                if self.peek_next().token_type == TokenType::TokenLeftParen {
                    return self.parse_call(ident);
                }

                return Node::Identifier(ident);
            }
            TokenType::TokenLeftParen => {
                let val = self.parse_node();

                let tkn = self.get_token();
                if tkn.token_type != TokenType::TokenRightParen {
                    opal_error_parser_unexpected_token(")".to_string(), tkn.literal, tkn.line);
                }

                return val;
            }
            _ => {
                opal_error_parser_invalid_expr(token.line);
                unreachable!()
            }
        }
    }

    fn parse_if(&mut self) -> Node {
        let expr = self.parse_expression();
        let mut ifstmt = IfStatement::new(Box::from(expr));

        let mut tkn = self.get_token();
        if tkn.token_type != TokenType::TokenLeftBrace {
            opal_error_parser_unexpected_token("{".to_string(), tkn.literal, tkn.line);
        }

        while self.peek_next().token_type != TokenType::TokenRightBrace {
            let node = self.parse_node();
            ifstmt.body.push(node);
        }

        tkn = self.get_token();
        if tkn.token_type != TokenType::TokenRightBrace {
            opal_error_parser_unexpected_token("}".to_string(), tkn.literal, tkn.line);
        }

        Node::IfStatement(ifstmt)
    }

    fn parse_call(&mut self, caller: Identifier) -> Node {
        let mut args: Vec<Identifier> = Vec::new();

        self.get_token();

        while self.peek_next().token_type != TokenType::TokenRightParen {
            let token = self.get_token();

            match token.token_type {
                TokenType::TokenIdentifier => args.push(Identifier::new(token.literal)),
                _ => {
                    opal_error_parser_unexpected_token(")".to_string(), token.literal, token.line);
                    unreachable!()
                }
            }

            match self.peek_next().token_type {
                TokenType::TokenComma => {
                    self.get_token();
                }
                TokenType::TokenRightParen => break,
                _ => {
                    opal_error_parser_unexpected_token(
                        ")".to_string(),
                        self.peek_next().literal.clone(),
                        self.peek_next().line,
                    );
                    unreachable!()
                }
            }
        }

        let mut tkn = self.get_token();
        if tkn.token_type != TokenType::TokenRightParen {
            opal_error_parser_unexpected_token(")".to_string(), tkn.literal, tkn.line);
            unreachable!()
        }

        tkn = self.get_token();
        if tkn.token_type != TokenType::TokenSemicolon {
            opal_error_parser_unexpected_token(";".to_string(), tkn.literal, tkn.line);
            unreachable!()
        }

        return Node::ProcedureCall(ProcedureCall::new(args, caller));
    }
}
