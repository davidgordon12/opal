#include <stdio.h>

#include "debug.h"
#include "vm/token_type.h"
#include "vm/chunk.h"
#include "vm/values.h"
#include "vm/object.h"

static int simple_instruction(string name, int offset);
static int const_instruction(string name, chunk* chunk, int offset);

void disassemble_chunk(chunk* chunk, string name) {
    printf("\n==== %s ====\n", name);
    printf("OFFSET | LINE | INSTRUCTION | CONST_INDEX | CONST_VALUE\n\n");
    for (int offset = 0; offset < chunk->count;) {
        offset = disassemble_instruction(chunk, offset);
    }
}

int disassemble_instruction(chunk* chunk, int offset) {
    printf("%04d ", offset);
    printf("%d ", chunk->lines[offset]);
    uint8_t instruction = chunk->bytes[offset];
    switch (instruction) {
    case OP_CONSTANT:
        return const_instruction("OP_CONSTANT", chunk, offset);
    case OP_ADD:
        return simple_instruction("OP_ADD", offset);
    case OP_SUBTRACT:
        return simple_instruction("OP_SUBTRACT", offset);
    case OP_MULTIPLY:
        return simple_instruction("OP_MULTIPLY", offset);
    case OP_DIVIDE:
        return simple_instruction("OP_DIVIDE", offset);
    case OP_NEGATE:
        return simple_instruction("OP_NEGATE", offset);
    case OP_NONE:
        return simple_instruction("OP_NONE", offset);
    case OP_TRUE:
        return simple_instruction("OP_TRUE", offset);
    case OP_FALSE:
        return simple_instruction("OP_FALSE", offset);
    case OP_RETURN:
        return simple_instruction("OP_RETURN", offset);
    case OP_NOT:
        return simple_instruction("OP_NOT", offset);
    case OP_EQUAL:
        return simple_instruction("OP_EQUAL", offset);
    case OP_GREATER:
        return simple_instruction("OP_GREATEER", offset);
    case OP_LESS:
        return simple_instruction("OP_LESS", offset);
    default:
        printf("Unknown opcode %d\n", instruction);
        return offset + 1;
    }
}

string get_token_name(token_type type) {
    switch(type) {
    case TOKEN_NUMBER: return "TOKEN_NUMBER";
    case TOKEN_PLUS: return "TOKEN_PLUS";
    case TOKEN_MINUS: return "TOKEN_MINUS";
    case TOKEN_STAR: return "TOKEN_STAR";
    case TOKEN_SLASH: return "TOKEN_SLASH";
    case TOKEN_LEFT_PAREN: return "TOKEN_LEFT_PAREN";
    case TOKEN_RIGHT_PAREN: return "TOKEN_RIGHT_PAREN";
    case TOKEN_BANG: return "TOKEN_BANG";
    case TOKEN_EQUAL: return "TOKEN_EQUAL";
    case TOKEN_BANG_EQUAL: return "TOKEN_BANG_EQUAL";
    case TOKEN_EQUAL_EQUAL: return "TOKEN_EQUAL_EQUAL";
    case TOKEN_NONE: return "TOKEN_NONE";
    case TOKEN_AND: return "TOKEN_AND";
    case TOKEN_PROC: return "TOKEN_PROC";
    case TOKEN_EOF: return "TOKEN_EOF";
    default: return "UNIMPLEMENTED TOKEN";
    }
}

static int simple_instruction(string name, int offset) {
    printf("%s\n", name);
    return offset + 1;
}

static int const_instruction(string name, chunk* chunk, int offset) {
    uint8_t idx = chunk->bytes[offset + 1];
    printf("%-16s %4d '", name, idx);
    print_value(chunk->constants.values[idx]);
    printf("'\n");
    return offset + 2;
}

void print_value(value val) {

    switch (val.type) {
    case VAL_OBJ:
        print_object(val);
        break;
    case VAL_NUMBER:
        printf("%g", AS_NUMBER(val));
        break;
    case VAL_BOOL:
        printf(AS_BOOL(val) ? "true" : "false");
        break;
    case VAL_NONE:
        printf("none");
        break;
    default:
        break;
    }
}