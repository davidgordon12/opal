#ifndef _CHUNK_H
#define _CHUNK_H

#include "common.h"
#include "array.h"
#include "values.h"

typedef enum {
    OP_CONSTANT,
    OP_NONE,
    OP_TRUE,
    OP_FALSE,
    OP_EQUAL,
    OP_GREATER,
    OP_LESS,
    OP_NOT,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_NEGATE,
    OP_RETURN,
} opcode;

typedef struct chunk {
    int count;
    int capacity;
    uint8_t* bytes;
    uint32_t* lines;
    value_array constants;
} chunk;

void init_chunk(chunk* chunk);
void write_chunk(chunk* chunk, uint8_t byte, uint32_t line);
void grow_chunk(chunk* chunk);
int add_constant(chunk* chunk, value value);
void free_chunk(chunk* chunk);

#endif
