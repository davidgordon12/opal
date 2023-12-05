#ifndef _COMPILER_H
#define _COMPILER_H

#include "lib/values.h"
#include "lib/chunk.h"

bool compile(string source, chunk* chunk);

#endif