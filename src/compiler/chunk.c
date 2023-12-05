#include "lib/chunk.h"
#include "lib/array.h"
#include "lib/memory.h"
#include "lib/values.h"

void init_chunk(chunk* chunk) {
    chunk->count = 0;
    chunk->capacity = 0;
    chunk->bytes = NULL;
    chunk->lines = NULL;
    init_array(&chunk->constants);
}

void write_chunk(chunk* chunk, uint8_t byte, uint32_t line) {
    if(chunk->capacity < chunk->count + 1) {
        grow_chunk(chunk);
    }

    chunk->bytes[chunk->count] = byte;
    chunk->lines[chunk->count] = line;
    chunk->count++;
}

void grow_chunk(chunk* chunk) {
    int old_capacity = chunk->capacity;
    chunk->capacity = GROW_CAPACITY(old_capacity);
    chunk->bytes = GROW_ARRAY(uint8_t, chunk->bytes, old_capacity, chunk->capacity);
    chunk->lines = GROW_ARRAY(uint32_t, chunk->lines, old_capacity, chunk->capacity);
}

int add_constant(chunk *chunk, value value) {
    write_array(&chunk->constants, value);
    return chunk->constants.count - 1;
}

void free_chunk(chunk* chunk) {
    FREE_ARRAY(uint8_t, chunk->bytes, chunk->capacity);
    FREE_ARRAY(uint32_t, chunk->lines, chunk->capacity);
    free_array(&chunk->constants);
    init_chunk(chunk);
}
