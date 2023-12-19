#ifndef _ARRAY_H
#define _ARRAY_H

#include "common.h"
#include "memory.h"
#include "values.h"

typedef struct {
    int count;
    int capacity;
    value* values;
} value_array;

void init_array(value_array* value);
void write_array(value_array* values, value value);
void grow_array(value_array* value);
void free_array(value_array* value);

#endif
