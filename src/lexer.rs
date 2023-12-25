use crate::tokens::*;
use crate::error::error;

pub struct Lexer {
    source: String,
    line: i32,
    current: i32,
    start: i32,
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
            _ => error("Invalid character on line", Some(&self.line.to_string()), None, None),
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
            TokenType::TokenLeftParen => literal = "left_paren",
            TokenType::TokenRightParen => literal = "right_paren",
            TokenType::TokenLeftBrace => literal = "left_brace",
            TokenType::TokenRightBrace => literal = "right_brace",
            TokenType::TokenLeftBracket => literal = "left_bracket",
            TokenType::TokenRightBracket => literal = "right_bracket",
            TokenType::TokenDot => literal = "dot",
            TokenType::TokenComma => literal = "comma",
            TokenType::TokenSemicolon => literal = "semicolon",
            TokenType::TokenPlus => literal = "plus",
            TokenType::TokenMinus => literal = "minus",
            TokenType::TokenStar => literal = "star",
            TokenType::TokenSlash => literal = "slash",
            TokenType::TokenPower => literal = "power",
            TokenType::TokenPound => literal = "pound",
            TokenType::TokenEqual => literal = "equal",
            TokenType::TokenEqualEqual => literal = "equal_equal",
            TokenType::TokenBang => literal = "bang",
            TokenType::TokenBangEqual => literal = "bang_equal",
            TokenType::TokenLess => literal = "less",
            TokenType::TokenLessEqual => literal = "less_equal",
            TokenType::TokenGreater => literal = "greater",
            TokenType::TokenGreaterEqual => literal = "greater_equal",
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

        while self.ch.is_numeric() || self.ch == '.' {
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
        self.ch == '\0' || self.current == self.source.len() as i32
    }
}
