#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenLeftBracket, TokenRightBracket,
    TokenComma, TokenDot, TokenMinus, TokenPlus, TokenPower,
    TokenSemicolon, TokenSlash, TokenStar, TokenPound, TokenModulo,
    
    // One or two character tokens.
    TokenBang, TokenBangEqual,
    TokenEqual, TokenEqualEqual,
    TokenGreater, TokenGreaterEqual,
    TokenLess, TokenLessEqual,
    
    // Literals.
    TokenIdentifier, TokenString, TokenNumber,
    
    // Keywords.
    TokenAnd, TokenElse, TokenFalse,
    TokenFor, TokenProc, TokenIf, TokenNull, TokenOr,
    TokenReturn, TokenTrue, TokenLet, TokenNot,
    
    TokenEof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: i64,
    pub literal: String,
    pub token_type: TokenType,
}

pub fn reserved_keyword(keyword: &str) -> TokenType {
    match keyword {
        "and" => TokenType::TokenAnd,
        "proc" => TokenType::TokenProc,
        "if" => TokenType::TokenIf,
        "else" => TokenType::TokenElse,
        "or" => TokenType::TokenOr,
        "for" => TokenType::TokenFor,
        "true" => TokenType::TokenTrue,
        "false" => TokenType::TokenFalse,
        "let" => TokenType::TokenLet,
        "null" => TokenType::TokenNull,
        "not" => TokenType::TokenNot,
        "return" => TokenType::TokenReturn,
        _ => TokenType::TokenIdentifier,
    }
}
