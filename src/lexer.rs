use crate::tokens::*;
use crate::error::{parse_token_error, operation_error};

pub struct Lexer {
    source: String,
    line: i64,
    current: i64,
    start: i64,
    ch: char,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source,
            line: 1,
            current: 0,
            start: 0,
            ch: '0',
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.eof() {
            tokens.push(self.next_token())
        }

        tokens.push(self.make_token(TokenType::TokenEof));
        tokens
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        self.read_char();
        
        if self.eof() {
            return self.make_token(TokenType::TokenEof);
        }

        if self.ch.is_alphabetic() {
            return self.read_ident()
        }

        if self.ch.is_numeric() {
            return self.read_number()
        }

        if self.ch == '"' {
            return self.read_string()
        }

        match self.ch {
            '(' => return self.make_token(TokenType::TokenLeftParen),
            ')' => return self.make_token(TokenType::TokenRightParen),
            '{' => return self.make_token(TokenType::TokenLeftBrace),
            '}' => return self.make_token(TokenType::TokenRightBrace),
            '[' => return self.make_token(TokenType::TokenLeftBracket),
            ']' => return self.make_token(TokenType::TokenRightBracket),
            '.' => return self.make_token(TokenType::TokenDot),
            ',' => return self.make_token(TokenType::TokenComma),
            ';' => return self.make_token(TokenType::TokenSemicolon),
            '+' => return self.make_token(TokenType::TokenPlus),
            '-' => return self.make_token(TokenType::TokenMinus),
            '*' => return self.make_token(TokenType::TokenStar),
            '/' => return self.make_token(TokenType::TokenSlash),
            '^' => return self.make_token(TokenType::TokenPower),
            '%' => return self.make_token(TokenType::TokenModulo),
            '=' => {
                match self.next_char('=') {
                    true => return self.make_token(TokenType::TokenEqualEqual),
                    false => return self.make_token(TokenType::TokenEqual),
                };
            },
            '!' => {
                match self.next_char('=') {
                    true => return self.make_token(TokenType::TokenBangEqual),
                    false => return self.make_token(TokenType::TokenBang),
                }
            },
            '>' => {
                match self.next_char('=') {
                    true => return self.make_token(TokenType::TokenGreaterEqual),
                    false => return self.make_token(TokenType::TokenGreater),
                }
            },
            '<' => {
                match self.next_char('=') {
                    true => return self.make_token(TokenType::TokenLessEqual),
                    false => return self.make_token(TokenType::TokenLess),
                }
            },
            '#' => return self.make_token(TokenType::TokenPound),
            '&' => {
                match self.next_char('&') {
                    true => return self.make_token(TokenType::TokenAnd),
                    false => operation_error("Invalid AND operation. Use && instead.", &self.line.to_string()),
                }
            }
            '|' => {
                match self.next_char('|') {
                    true => return self.make_token(TokenType::TokenOr),
                    false => operation_error("Invalid OR operation. Use || instead.", &self.line.to_string()),
                }
            }
            _ => parse_token_error("Invalid character", &self.ch.to_string(), &self.line.to_string()),
        };

        unreachable!()
    }
    
    fn read_char(&mut self) {
        let index: usize = self.current.try_into().unwrap();
        self.ch = self.source.as_bytes()[index] as char;
        self.current += 1;
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        #[allow(unused_assignments)]
        let mut literal = "";            

        match token_type {
            TokenType::TokenLeftParen => literal = "(",
            TokenType::TokenRightParen => literal = ")",
            TokenType::TokenLeftBrace => literal = "{",
            TokenType::TokenRightBrace => literal = "}",
            TokenType::TokenLeftBracket => literal = "[",
            TokenType::TokenRightBracket => literal = "]",
            TokenType::TokenDot => literal = ".",
            TokenType::TokenComma => literal = ",",
            TokenType::TokenSemicolon => literal = ";",
            TokenType::TokenPlus => literal = "+",
            TokenType::TokenMinus => literal = "-",
            TokenType::TokenStar => literal = "*",
            TokenType::TokenSlash => literal = "/",
            TokenType::TokenPower => literal = "^",
            TokenType::TokenModulo => literal = "%",
            TokenType::TokenPound => literal = "#",
            TokenType::TokenEqual => literal = "=",
            TokenType::TokenEqualEqual => literal = "==",
            TokenType::TokenBang => literal = "!",
            TokenType::TokenBangEqual => literal = "!=",
            TokenType::TokenLess => literal = "<",
            TokenType::TokenLessEqual => literal = "<=",
            TokenType::TokenGreater => literal = ">",
            TokenType::TokenGreaterEqual => literal = ">=",
            TokenType::TokenAnd => literal = "&&",
            TokenType::TokenOr => literal = "||",
            TokenType::TokenProc => literal = "proc",
            TokenType::TokenIf => literal = "if",
            TokenType::TokenElse => literal = "else",
            TokenType::TokenFor => literal = "for",
            TokenType::TokenTrue => literal = "true",
            TokenType::TokenFalse => literal = "false",
            TokenType::TokenLet => literal = "let",
            TokenType::TokenNull => literal = "null",
            TokenType::TokenNot => literal = "not",
            TokenType::TokenReturn => literal = "return",
            TokenType::TokenEof => literal = "eof",
            _ => literal = "error",            
        }

        Token {
            token_type: token_type,
            line: self.line,
            literal: String::from(literal),
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
        while self.peek().is_alphabetic() {
            self.read_char();
        }

        let ident: String = self.source
            .chars()
            .skip(self.start as usize)
            .take((self.current-self.start) as usize)
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

        while self.peek().is_numeric() || self.peek() == '.' {
            self.read_char();
        }

        let ident: String = self.source
            .chars()
            .skip(self.start as usize)
            .take((self.current-self.start) as usize)
            .collect();

        Token {
            line: self.line,
            literal: ident,
            token_type: TokenType::TokenNumber,
        }
    }

    fn read_string(&mut self) -> Token {
        // Eat the first quotation mark
        self.read_char();

        self.start = self.current - 1;
        
        while self.peek() != '"' {
            self.read_char();
        }

        // Eat the last quotation mark
        self.read_char();

        let ident: String = self.source
            .chars()
            .skip(self.start as usize)
            .take(((self.current-self.start) - 1) as usize)
            .collect();

        Token {
            line: self.line,
            literal: ident,
            token_type: TokenType::TokenString,
        }
    }
    
    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' => self.read_char(),
                '\t' => { 
                    self.read_char() 
                }
                '\r' => {
                    self.read_char() 
                }
                '\n' => { 
                    self.line += 1; 
                    self.read_char() 
                }
                _ => return,
            };
        }
    }

    fn eof(&self) -> bool {
        self.ch == '\0' || self.current == self.source.len() as i64
    }
}
