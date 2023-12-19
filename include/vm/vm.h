#ifndef _VM_H
#define _VM_H

#include "chunk.h"
#include "values.h"
#include "result.h"
#include "table.h"

#define STACK_MAX 512

typedef struct {
    chunk* chunk;
    uint8_t* ip;
    value stack[STACK_MAX];
    value* sp;
    object* objs;
    table strings;
    table globals;
} vm;

extern vm dvm;

void init_vm();
void free_vm();

result interpret(string source);

#endif