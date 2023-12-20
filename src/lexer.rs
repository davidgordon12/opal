use crate::tokens::*;

pub struct Lexer {
    source: String,
    current: i32,
    ch: char,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source,
            current: 0,
            ch: '0',
        }
    }

    pub fn tokenize(&mut self) {
        while self.next_token().token_type != TokenType::TokenEof {
            println!("READING");
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        self.read_char();

        match self.ch {
            _ => "not yet implemented",
        };

        self.make_token(TokenType::TokenEof)
    }

    fn make_token(&mut self, token_type: TokenType, line: i32) -> Token {
        let mut literal = "";
        
        match token_type {
            TokenType::TokenEof => { literal = "eof" }
            _ => { literal = "eof" }
        }

        Token {
            token_type: token_type,
            line: line,
            literal: literal.to_string(),
        }
    }

    fn read_char(&mut self) {
        let index: usize = self.current.try_into().unwrap();
        self.ch = self.source.as_bytes()[index] as char;
        self.current += 1;
    }

    fn peek(&mut self) -> char {
        let index: usize = self.current.try_into().unwrap();
        self.source.as_bytes()[index + 1] as char
    }
    
    fn read_ident(&mut self) {
        
    }
    
    fn read_number(&mut self) {
        
    }
    
    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' => self.read_char(),
                '\r' => self.read_char(),
                '\t' => self.read_char(),
                '\n' => self.read_char(),
                _ => return,
            };
        }
    }

    fn is_alpha(&self) -> bool {
        todo!()
    }

    fn is_numeric(&self) -> bool {
        todo!()
    }

    fn new_token(&mut self) -> Token {
        todo!()
    }

    fn eof(&self) -> bool {
        self.ch == '\0' || self.current >= self.source.len() as i32
    }
}