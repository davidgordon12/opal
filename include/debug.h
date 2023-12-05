#ifndef _DEBUG_H
#define _DEBUG_H

#include "common.h"
#include "compiler/token_type.h"
#include "lib/chunk.h"
#include "lib/values.h"

// #define DEBUG_PRINT_TOKEN
// #define DEBUG_PRINT_CODE
#define DEBUG_TRACE_EXECUTION

void disassemble_chunk(chunk* chunk, string name);
int disassemble_instruction(chunk* chunk, int offset);
void print_value(value val);
string get_token_name(token_type type);

#endif
