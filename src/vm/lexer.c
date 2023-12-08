#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "common.h"
#include "debug.h"
#include "vm/lexer.h"
#include "vm/token_type.h"
#include "vm/values.h"

lexer _lexer; /* Static instance of lexer. Rewrite to use pointers instead. */

void init_lexer(string source) {
    _lexer.left = source;
    _lexer.right = source;
    _lexer.line = 1;
}

/**
 * Compares provided input against the expected token type and returns the type if they're equal,
 * otherwise returns a identifier token
 * @param start the starting index of the string
 * @param length the length of the remaining string after the starting character
 * @param remainder the substring of the keyword excluding the start character
 * @param type the expected token_type
 * 
 * @returns The expected token type if memcmp returns 1
*/
static token_type check_keyword(uint8_t start, uint8_t length, string remainder,
                                token_type type) {
    if ((_lexer.right - _lexer.left == start + length) &&
        memcmp(_lexer.left + start, remainder, length) == 0)
        return type;

    return TOKEN_IDENTIFIER;
}

static char peek() { return *_lexer.right; }

/*static char peek_next() { return _lexer.right[1]; }*/

static bool eof() { return *_lexer.right == '\0'; }

static char advance() {
    _lexer.right++;
    return _lexer.right[-1];
}

static void skip_whitespace() {
    for (;;) {
    char c = peek();
    switch (c) {
      case ' ':
      case '\r':
      case '\t':
        advance();
        break;
      default:
        return;
    }
  }
}

static token make_token(token_type type) {
    token tkn;
    tkn.type = type;
    tkn.line = _lexer.line;
    tkn.length = (int)(_lexer.right - _lexer.left);
    tkn.start = _lexer.left;
    return tkn;
}

static token error_token(string message) {
    token tkn;
    tkn.type = TOKEN_ERROR;
    tkn.start = _lexer.left;
    tkn.length = strlen(message);
    tkn.line = _lexer.line;
    return tkn;
}

static bool match(char expected) {
    if (eof())
        return false;
    if (*_lexer.right != expected)
        return false;
    _lexer.right++;
    return true;
}

static token str() {
    while (peek() != '"' && !eof()) {
        if (peek() == '\n')
            _lexer.line++;
        advance();
    }

    if (eof())
        return error_token("Unterminated string");

    advance(); // Eat the closing quotation

    return make_token(TOKEN_STRING);
}

static bool is_digit(char c) { return c >= '0' && c <= '9'; }

static bool is_alpha(char c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_');
}

static token num() {
    while (is_digit(peek()) || peek() == '.')
        advance();

    return make_token(TOKEN_NUMBER);
}

static token ident() {
    while (is_alpha(peek()) || is_digit(peek()))
        advance();

    token_type type;

    switch (_lexer.left[0]) {
    case 'a':
        type = check_keyword(1, 2, "nd", TOKEN_AND);
        break;
    case 'c':
        type = check_keyword(1, 4, "lass", TOKEN_CLASS);
        break;
    case 'e':
        type = check_keyword(1, 3, "lse", TOKEN_ELSE);
        break;
    case 'f':
        if (_lexer.right - _lexer.left > 1) {
            switch (_lexer.left[1]) {
            case 'a':
                type = check_keyword(2, 3, "lse", TOKEN_FALSE);
                break;
            case 'o':
                break;
            case 'u':
                break;
            }
        }
        break;
    case 'i':
        type = check_keyword(1, 1, "f", TOKEN_IF);
        break;
    case 'n':
        if (_lexer.right - _lexer.left > 1)
            switch (_lexer.left[1]) {
            case 'o':
                type = check_keyword(2, 2, "ne", TOKEN_NONE);
                break;
            case 't':
                type = check_keyword(2, 1, "t", TOKEN_NOT);
                break;
            }
        break;
    case 'o':
        type = check_keyword(1, 1, "r", TOKEN_OR);
        break;
    case 'p':
        if (_lexer.right - _lexer.left > 2) {
            switch (_lexer.left[2]) {
            case 'i':
                type = check_keyword(3, 2, "nt", TOKEN_PRINT);
                break;
            case 'o':
                type = check_keyword(3, 1, "c", TOKEN_PROC);
                break;
            }
        }
        break;
    case 'r':
        type = check_keyword(1, 5, "eturn", TOKEN_RETURN);
        break;
    case 's':
        type = check_keyword(1, 4, "uper", TOKEN_SUPER);
        break;
    case 'l':
        type = check_keyword(1, 2, "et", TOKEN_LET);
        break;
    case 'w':
        type = check_keyword(1, 4, "hile", TOKEN_WHILE);
        break;
    case 't':
        type = check_keyword(1, 3, "rue", TOKEN_TRUE);
        break;
    default:
        type = TOKEN_IDENTIFIER;
        break;
    }

    return make_token(type);
}

token scan_token() {
    skip_whitespace();

    _lexer.left = _lexer.right;

    if (eof())
        return make_token(TOKEN_EOF);

    char c = advance();

    if(is_alpha(c)) return ident();
    if(is_digit(c)) return num();

    switch (c) {
    case '(':
        return make_token(TOKEN_LEFT_PAREN);
    case ')':
        return make_token(TOKEN_RIGHT_PAREN);
    case '{':
        return make_token(TOKEN_LEFT_BRACE);
    case '}':
        return make_token(TOKEN_RIGHT_BRACE);
    case ';':
        return make_token(TOKEN_SEMICOLON);
    case ',':
        return make_token(TOKEN_COMMA);
    case '.':
        return make_token(TOKEN_DOT);
    case '-':
        return make_token(TOKEN_MINUS);
    case '+':
        return make_token(TOKEN_PLUS);
    case '/':
        return make_token(TOKEN_SLASH);
    case '*':
        return make_token(TOKEN_STAR);
    case '"':
        return str();
    case '!':
        return make_token(match('=') ? TOKEN_BANG_EQUAL : TOKEN_BANG);
    case '=':
        return make_token(match('=') ? TOKEN_EQUAL_EQUAL : TOKEN_EQUAL);
    case '<':
        return make_token(match('=') ? TOKEN_LESS_EQUAL : TOKEN_LESS);
    case '>':
        return make_token(match('=') ? TOKEN_GREATER_EQUAL : TOKEN_GREATER);
    case '#':
        while (peek() != '\n' && !eof())
            advance();
        return make_token(TOKEN_POUND);
    }

    return error_token("Unexpected token");
}