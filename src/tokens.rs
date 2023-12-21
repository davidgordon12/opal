#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenLeftBracket, TokenRightBracket,
    TokenComma, TokenDot, TokenMinus, TokenPlus,
    TokenSemicolon, TokenSlash, TokenStar, TokenPound,
    
    // One or two character tokens.
    TokenBang, TokenBangEqual,
    TokenEqual, TokenEqualEqual,
    TokenGreater, TokenGreaterEqual,
    TokenLess, TokenLessEqual,
    
    // Literals.
    TokenIdentifier, TokenString, TokenNumber,
    
    // Keywords.
    TokenAnd, TokenClass, TokenElse, TokenFalse,
    TokenFor, TokenProc, TokenIf, TokenNone, TokenOr,
    TokenPrint, TokenReturn, TokenSuper, TokenThis,
    TokenTrue, TokenLet, TokenWhile, TokenNot,
    
    TokenError, TokenEof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: i32,
    pub literal: String,
    pub token_type: TokenType,
}

pub fn reserved_keyword(keyword: &str) -> TokenType {
    match keyword {
        "and" => TokenType::TokenAnd,
        "proc" => TokenType::TokenProc,
        "if" => TokenType::TokenIf,
        _ => TokenType::TokenIdentifier,
    }
}