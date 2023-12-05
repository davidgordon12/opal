#include <stdio.h>
#include <stdlib.h>

#include "compiler/compiler.h"
#include "compiler/lexer.h"
#include "compiler/parser.h"
#include "compiler/token.h"
#include "compiler/token_type.h"
#include "lib/chunk.h"
#include "lib/values.h"
#include "debug.h"

typedef void (*parse_fn)();

typedef struct  {
    parse_fn prefix;
    parse_fn infix;
    precedence precedence;
} parse_rule;

parser _parser;
chunk* curr_chunk;

static void advance();
static void expression();
static void consume(token_type type, string message);
static void emit_byte(uint8_t byte);
static void emit_bytes(uint8_t byte1, uint8_t byte2);
static chunk* current_chunk();
static void end_compilation();
static void emit_return();
static void number();
static void unary();
static void literal();
static void parse_precedence(precedence precedence);
static void emit_constant(value value);
static uint8_t make_constant(value value);
static void grouping();
static parse_rule* get_rule(token_type type);

static void error_at_current(string message);
static void error_at(token* token, string message);
static void error(string message);


bool compile(string source, chunk* chunk) {
    init_lexer(source);
    curr_chunk = chunk;

    _parser.had_error = false;
    _parser.panic_mode = false;

    advance();

    expression();

    consume(TOKEN_EOF, "Expected end of expression");

    end_compilation();

    return !_parser.had_error;
}

void advance() {
    _parser.previous = _parser.current;

    for (;;) {
        _parser.current = scan_token();
#ifdef DEBUG_PRINT_TOKEN
        fprintf(stdout, "%s\n", get_token_name(_parser.current.type));
#endif
        if (_parser.current.type != TOKEN_ERROR)
            break;
        error_at_current(_parser.current.start);
    }
}

static void expression() {
    parse_precedence(PREC_ASSIGNMENT);
}

static void consume(token_type type, string message) {
    if (_parser.current.type == type) {
        advance();
        return;
    }

    error_at_current(message);
}

static void emit_byte(uint8_t byte) {
    write_chunk(current_chunk(), byte, _parser.previous.line);
}

static chunk* current_chunk() { return curr_chunk; }

static void end_compilation() { 
    emit_return();
#ifdef DEBUG_PRINT_CODE
    if(!_parser.had_error) {
        disassemble_chunk(current_chunk(), "CODE");
    }
#endif
}

static void emit_return() { emit_byte(OP_RETURN); }

static void number() {
    double val = strtod(_parser.previous.start, NULL);
    emit_constant(NUMBER_VAL(val));
}

static void unary() {
    token_type op_type = _parser.previous.type;

    parse_precedence(PREC_UNARY);

    switch (op_type) {
    case TOKEN_MINUS:
        emit_byte(OP_NEGATE);
        break;
    default:
        return;
    }
}

static void binary() {
  token_type operator_type = _parser.previous.type;
  parse_rule* rule = get_rule(operator_type);
  parse_precedence((precedence)(rule->precedence + 1));

  switch (operator_type) {
    case TOKEN_PLUS:          emit_byte(OP_ADD); break;
    case TOKEN_MINUS:         emit_byte(OP_SUBTRACT); break;
    case TOKEN_STAR:          emit_byte(OP_MULTIPLY); break;
    case TOKEN_SLASH:         emit_byte(OP_DIVIDE); break;
    default: return; // Unreachable.
  }
}

static void grouping() {
  expression();
  consume(TOKEN_RIGHT_PAREN, "Expected ')' after expression.");
}

static void literal() {
    switch (_parser.previous.type) {
    case TOKEN_FALSE:
        emit_byte(OP_FALSE);
        break;
    case TOKEN_TRUE:
        emit_byte(OP_TRUE);
        break;
    case TOKEN_NONE:
        emit_byte(OP_NONE);
        break;
    case TOKEN_BANG:
        emit_byte(OP_NOT);
        break;
    case TOKEN_BANG_EQUAL:
        emit_byte(OP_EQUAL);
        break;
    case TOKEN_GREATER:
        emit_byte(OP_GREATER);
        break;
    case TOKEN_GREATER_EQUAL:
        emit_bytes(OP_LESS, OP_NOT);
        break;
    case TOKEN_LESS:
        emit_byte(OP_LESS);
        break;
    case TOKEN_LESS_EQUAL:
        emit_bytes(OP_GREATER, OP_NOT);
        break;
    default:
        return;
    }
}

parse_rule rules[] = {
  [TOKEN_LEFT_PAREN]    = {grouping, NULL,   PREC_NONE},
  [TOKEN_RIGHT_PAREN]   = {NULL,     NULL,   PREC_NONE},
  [TOKEN_LEFT_BRACE]    = {NULL,     NULL,   PREC_NONE}, 
  [TOKEN_RIGHT_BRACE]   = {NULL,     NULL,   PREC_NONE},
  [TOKEN_COMMA]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_DOT]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_MINUS]         = {unary,    binary, PREC_TERM},
  [TOKEN_PLUS]          = {NULL,     binary, PREC_TERM},
  [TOKEN_SEMICOLON]     = {NULL,     NULL,   PREC_NONE},
  [TOKEN_SLASH]         = {NULL,     binary, PREC_FACTOR},
  [TOKEN_STAR]          = {NULL,     binary, PREC_FACTOR},
  [TOKEN_BANG]          = {unary,     NULL,   PREC_NONE},
  [TOKEN_BANG_EQUAL]    = {NULL,     binary,   PREC_EQUALITY},
  [TOKEN_EQUAL]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_EQUAL_EQUAL]   = {NULL,     binary,   PREC_EQUALITY},
  [TOKEN_GREATER]       = {NULL,     binary,   PREC_COMPARISON},
  [TOKEN_GREATER_EQUAL] = {NULL,     binary,   PREC_COMPARISON},
  [TOKEN_LESS]          = {NULL,     binary,   PREC_COMPARISON},
  [TOKEN_LESS_EQUAL]    = {NULL,     binary,   PREC_COMPARISON},
  [TOKEN_IDENTIFIER]    = {NULL,     NULL,   PREC_NONE},
  [TOKEN_STRING]        = {NULL,     NULL,   PREC_NONE},
  [TOKEN_NUMBER]        = {number,   NULL,   PREC_NONE},
  [TOKEN_AND]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_CLASS]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_ELSE]          = {NULL,     NULL,   PREC_NONE},
  [TOKEN_TRUE]          = {literal,     NULL,   PREC_NONE},
  [TOKEN_FALSE]         = {literal,     NULL,   PREC_NONE},
  [TOKEN_FOR]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_PROC]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_IF]            = {NULL,     NULL,   PREC_NONE},
  [TOKEN_NONE]           = {literal,     NULL,   PREC_NONE},
  [TOKEN_OR]            = {NULL,     NULL,   PREC_NONE},
  [TOKEN_PRINT]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_RETURN]        = {NULL,     NULL,   PREC_NONE},
  [TOKEN_SUPER]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_THIS]          = {NULL,     NULL,   PREC_NONE},
  [TOKEN_LET]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_WHILE]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_ERROR]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_EOF]           = {NULL,     NULL,   PREC_NONE},
};

static parse_rule* get_rule(token_type type) {
  return &rules[type];
}

static void parse_precedence(precedence precedence) {
    advance();
    parse_fn prefix_rule = get_rule(_parser.previous.type)->prefix;
    if (prefix_rule == NULL) {
        error("Expected expression.");
        return;
    }

    prefix_rule();

    while(precedence <= get_rule(_parser.current.type)->precedence) {
        advance();
        parse_fn infix_rule = get_rule(_parser.previous.type)->infix;
        infix_rule();
    }
}

static void emit_constant(value value) {
    emit_bytes(OP_CONSTANT, make_constant(value));
}

static uint8_t make_constant(value value) {
    int constant = add_constant(current_chunk(), value);
    if (constant > UINT8_MAX) {
        error("Too many constants in one chunk.");
        return 0;
    }

    return (uint8_t)constant;
}

static void emit_bytes(uint8_t byte1, uint8_t byte2) {
    emit_byte(byte1);
    emit_byte(byte2);
}

static void error_at_current(string message) {
    error_at(&_parser.current, message);
}

static void error(string message) { error_at(&_parser.current, message); }

static void error_at(token* token, string message) {
    if (_parser.panic_mode)
        return;

    _parser.panic_mode = true;
    fprintf(stderr, "[line %d] Error", token->line);

    if (token->type == TOKEN_EOF) {
        fprintf(stderr, " at end");
    } else if (token->type == TOKEN_ERROR) {
    } else {
        fprintf(stderr, " at '%.*s'", token->length, token->start);
    }

    fprintf(stderr, ": %s\n", message);
    _parser.had_error = true;
}
