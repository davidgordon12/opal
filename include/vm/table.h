#ifndef _TABLE_H
#define _TABLE_H

#include "vm/values.h"
#include <stdbool.h>

#define TABLE_MAX_LOAD 0.75

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
void free_table(table* tbl);
void table_copy(table* src, table* dest);

bool table_add(table* tbl, object_string* key, value val);
bool table_remove(table* tbl, object_string* key);
bool table_get(table* tbl, object_string* key, value* val);

#endif
