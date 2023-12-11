#ifndef _TABLE_H
#define _TABLE_H

#include "common.h"
#include "vm/values.h"

typedef struct {
    object_string* key;
    value val;
} entry;

typedef struct {
    uint64_t count;
    uint64_t capacity;
    entry* entries;
} table;

void init_table(table* tbl);

#endif
