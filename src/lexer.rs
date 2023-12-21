use std::{error::Error, slice};
use crate::tokens::*;

pub struct Lexer {
    source: String,
    line: i32,
    current: i32,
    start: i32,
    ch: char,
    error: bool,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source,
            line: 1,
            current: 0,
            start: 0,
            ch: '0',
            error: false,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            let token: Token = self.next_token();
            tokens.push(token.clone());

            if token.token_type == TokenType::TokenEof || 
                token.token_type == TokenType::TokenError 
            { 
                break;
            }
        } 

        tokens
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.eof() {
            return self.make_token(TokenType::TokenEof);
        }

        self.read_char();

        if self.ch.is_alphabetic() {
            return self.read_ident()
        }

        if self.ch.is_numeric() {
            return self.read_number()
        }

        match self.ch {
            '(' => return self.make_token(TokenType::TokenLeftParen),
            ')' => return self.make_token(TokenType::TokenRightParen),
            '{' => return self.make_token(TokenType::TokenLeftBrace),
            '}' => return self.make_token(TokenType::TokenRightBrace),
            '[' => return self.make_token(TokenType::TokenLeftBracket),
            ']' => return self.make_token(TokenType::TokenRightBracket),
            ',' => return self.make_token(TokenType::TokenComma),
            '"' => return self.make_token(TokenType::TokenString),
            ';' => return self.make_token(TokenType::TokenSemicolon),
            '+' => return self.make_token(TokenType::TokenPlus),
            '-' => return self.make_token(TokenType::TokenMinus),
            '*' => return self.make_token(TokenType::TokenStar),
            '/' => return self.make_token(TokenType::TokenSlash),
            '=' => {
                match self.next_char('=') {
                    true => return self.make_token(TokenType::TokenEqualEqual),
                    false => return self.make_token(TokenType::TokenEqual),
                };
            },
            '!' => return self.make_token(TokenType::TokenBang),
            '>' => return self.make_token(TokenType::TokenGreater),
            '<' => return self.make_token(TokenType::TokenLess),
            '#' => return self.make_token(TokenType::TokenPound),
            _ => return self.error("Invalid character"),
        };
    }
    
    fn read_char(&mut self) {
        let index: usize = self.current.try_into().unwrap();
        self.ch = self.source.as_bytes()[index] as char;
        self.current += 1;
    }
    
    fn make_token(&mut self, token_type: TokenType) -> Token {
        let mut literal = "";            

        match token_type {
            TokenType::TokenLeftParen => literal = "left_paren",
            TokenType::TokenRightParen => literal = "right_paren",
            TokenType::TokenLeftBrace => literal = "left_brace",
            TokenType::TokenRightBrace => literal = "right_brace",
            TokenType::TokenLeftBracket => literal = "left_bracket",
            TokenType::TokenRightBracket => literal = "right_bracket",
            TokenType::TokenComma => literal = "comma",
            TokenType::TokenSemicolon => literal = "semicolon",
            TokenType::TokenPlus => literal = "plus",
            TokenType::TokenMinus => literal = "minus",
            TokenType::TokenStar => literal = "star",
            TokenType::TokenSlash => literal = "slash",
            TokenType::TokenEqual => literal = "equal",
            TokenType::TokenBang => literal = "bang",
            TokenType::TokenPound => literal = "pound",
            TokenType::TokenEqualEqual => literal = "equal_equal",
            TokenType::TokenAnd => literal = "and",
            TokenType::TokenProc => literal = "proc",
            TokenType::TokenIf => literal = "if",
            TokenType::TokenElse => literal = "else",
            TokenType::TokenOr => literal = "or",
            TokenType::TokenFor => literal = "for",
            TokenType::TokenTrue => literal = "true",
            TokenType::TokenFalse => literal = "false",
            TokenType::TokenLet => literal = "let",
            TokenType::TokenNone => literal = "none",
            TokenType::TokenNot => literal = "not",
            TokenType::TokenReturn => literal = "return",
            TokenType::TokenError => literal = "error",
            TokenType::TokenEof => literal = "eof",
            _ => literal = "error",            
        }

        Token {
            token_type: token_type,
            line: self.line,
            literal: literal.to_string(),
        }
    }
    
    fn peek(&mut self) -> char {
        if self.eof() {
            return '\0'
        }
        let index: usize = self.current.try_into().unwrap();
        self.source.as_bytes()[index] as char
    }

    fn next_char(&mut self, ch: char) -> bool {
        if self.eof() {
            return false
        }

        let index: usize = self.current.try_into().unwrap();
        if self.source.as_bytes()[index] as char == ch {
            self.ch = ch;
            self.current += 1;
            return true
        }

        false
    }
    
    fn read_ident(&mut self) -> Token {
        self.start = self.current - 1;
        while self.ch.is_alphabetic() {
            self.read_char();
        }

        let ident: String = self.source
            .chars()
            .skip(self.start as usize)
            .take(((self.current-self.start) - 1) as usize)
            .collect();

    
        let token_type = reserved_keyword(&ident.to_lowercase());

        if token_type != TokenType::TokenIdentifier {
            return self.make_token(token_type)
        }

        Token {
            line: self.line,
            literal: ident,
            token_type: token_type
        }
    }
    
    fn read_number(&mut self) -> Token {
        self.start = self.current - 1;
        while self.ch.is_numeric() {
            self.read_char();
        }

        let ident: String = self.source
            .chars()
            .skip(self.start as usize)
            .take(((self.current-self.start) - 1) as usize)
            .collect();

        Token {
            line: self.line,
            literal: ident,
            token_type: TokenType::TokenNumber,
        }
    }
    
    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' => self.read_char(),
                '\t' => { 
                    self.read_char() 
                },
                '\r' => {
                    self.read_char() 
                }
                '\n' => { 
                    self.line += 1; 
                    self.read_char() 
                },
                _ => return,
            };
        }
    }

    fn eof(&self) -> bool {
        self.ch == '\0' || self.current == self.source.len() as i32
    }

    fn error(&mut self, message: &str) -> Token {
        println!("Opal: \x1b[91mSyntax Error\x1b[0m");
        println!("{} on line: \x1b[93m{}\x1b[0m", message, self.line);
        self.error = true;
        self.make_token(TokenType::TokenError)
    }
}