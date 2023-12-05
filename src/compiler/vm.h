#ifndef _VM_H
#define _VM_H

#include "lib/chunk.h"
#include "lib/values.h"
#include "result.h"

#define STACK_MAX 512

typedef struct {
    chunk* chunk;
    uint8_t* ip;
    value stack[STACK_MAX];
    value* sp;
} vm;

void init_vm();
void free_vm();

result interpret(string source);

#endif