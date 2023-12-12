#include <stdbool.h>
#include <stdlib.h>
#include <stdint.h>

#include "vm/table.h"
#include "vm/memory.h"
#include "vm/object.h"
#include "vm/values.h"

void init_table(table* tbl) {
    tbl->count = 0;
    tbl->capacity = 0;
    tbl->entries = NULL;
}

void free_table(table* tbl) {
    FREE_ARRAY(entry, tbl->entries, tbl->capacity);
    init_table(tbl);
}

static entry* find_entry(entry* entries, size_t capacity, object_string* key) {
    uint32_t index = key->hash % capacity;
    entry* tombstone = NULL;
    for(;;) {
        entry* ent = &entries[index];

        if (ent->key == NULL) {
            if (IS_NONE(ent->val)) {
                // Empty entry.
                return tombstone != NULL ? tombstone : ent;
            } else {
                // We found a tombstone.
                if (tombstone == NULL) tombstone = ent;
            }
        } else if (ent->key == key) {
            // We found the key.
            return ent;
        }
        index = (index + 1) % capacity;
    }
}

static void resize_table(table* tbl, int capacity) {
    entry* entries = ALLOCATE(entry, capacity);

    for(int i = 0; i < capacity; i++) {
        entries[i].key = NULL;
        entries[i].val = NONE_VAL;
    }
    
    tbl->count = 0;
    for(int i = 0; i < tbl->capacity; i++) {
        entry* ent = &tbl->entries[i];
        if(ent->key == NULL) continue;

        entry* dest = find_entry(entries, capacity, ent->key);
        dest->key = ent->key;
        dest->val = ent->val;
        tbl->count++;
    }

    FREE_ARRAY(entry, tbl->entries, tbl->capacity);
    tbl->entries = entries;
    tbl->capacity = capacity;
}

bool table_add(table* tbl, object_string* key, value val) {
    if(tbl->count + 1 > tbl->capacity * TABLE_MAX_LOAD) {
        int capacity = GROW_CAPACITY(tbl->capacity);
        resize_table(tbl, capacity);
    }

    entry* ent = find_entry(tbl->entries, tbl->capacity, key);
    bool exists = ent->key != NULL;

    if(!exists && IS_NONE(ent->val)) tbl->count++;

    ent->key = key;
    ent->val = val;

    return !exists;
}

bool table_remove(table *tbl, object_string *key) {
    if(tbl->count == 0) return false;

    entry* ent = find_entry(tbl->entries, tbl->capacity, key);
    if(ent->key == NULL) return false;

    ent->key = NULL;
    ent->val = BOOL_VAL(true);

    return true;
}

void table_copy(table *src, table *dest) {
    for(int i = 0; i < src->capacity; i++) {
        entry* ent = &src->entries[i];
        if(ent->key == NULL) continue;
        table_add(dest, ent->key, ent->val);
    }
}

bool table_get(table* tbl, object_string* key, value* val) {
    if(tbl->count == 0) return 0;

    entry* ent = find_entry(tbl->entries, tbl->capacity, key);
    if(ent->key == NULL) return false;

    *val = ent->val;
    return true;
}
