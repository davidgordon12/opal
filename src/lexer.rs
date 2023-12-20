use crate::tokens::*;
pub struct Lexer {
    source: String,
    line: i32,
    current: i32,
    ch: char,
    error: bool,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source,
            line: 1,
            current: 0,
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

        /*
        if self.is_alpha() {
            todo!()
        }

        if self.is_numeric() {
            todo!()
        }
 */
        match self.ch {
            '(' => return self.make_token(TokenType::TokenLeftParen),
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
                TokenType::TokenProc => literal = "proc",
                TokenType::TokenError => literal = "error",
                TokenType::TokenEof => literal = "eof",
                TokenType::TokenLeftParen => literal = "left_paren",
                _ => literal = "error",
            }
    
            Token {
                token_type: token_type,
                line: self.line,
                literal: literal.to_string(),
            }
        }
    
    fn peek(&mut self) -> char {
        let index: usize = self.current.try_into().unwrap();
        if self.eof() {
            return '\0';
        }
        self.source.as_bytes()[index] as char
    }
    
    fn read_ident(&mut self) {
        
    }
    
    fn read_number(&mut self) {
        
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
        self.ch == '\0' || self.current == self.source.len() as i32
    }

    fn error(&mut self, message: &str) -> Token {
        println!("Opal: \x1b[91mSyntax Error\x1b[0m");
        println!("{} on line: \x1b[93m{}\x1b[0m", message, self.line);
        self.error = true;
        self.make_token(TokenType::TokenError)
    }
}