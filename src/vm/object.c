#include "vm/vm.h"
#include "vm/values.h"
#include "vm/object.h"
#include "vm/memory.h"
#include <stdint.h>

static object* allocate_object(size_t size, object_type type);
static object_string* allocate_string(mut_string chars, uint64_t length, uint32_t hash);

#define ALLOCATE_OBJ(type, obj_type) \
    (type*)allocate_object(sizeof(type), obj_type)

static object_string* allocate_string(mut_string chars, uint64_t length, uint32_t hash) {
    object_string* str = ALLOCATE_OBJ(object_string, OBJ_STRING);
    str->length = length;
    str->chars = chars;
    str->hash = hash;
    table_add(&dvm.strings, str, NONE_VAL);
    return str;
}

object_string* table_search(table* tbl, string chars, int length, uint32_t hash) {
    if(tbl->count == 0) return NULL;

    uint32_t index = hash % tbl->capacity;

    for(;;) {
        entry* ent = &tbl->entries[index];

        if(ent->key == NULL) {
            if(IS_NONE(ent->val)) return NULL;
        } else if(ent->key->length == length && 
            ent->key->hash == hash && 
            memcmp(ent->key->chars, chars, length) == 0) {
            return ent->key;
        }

        index = (index + 1) % tbl->capacity;
    }
}

static object* allocate_object(size_t size, object_type type) {
    object* obj = (object*)reallocate(NULL, 0, size);
    obj->type = type;
    obj->next = dvm.objs;
    return obj;
}

static uint32_t hash_string(string chars, int length) {
    uint32_t hash_int = 2166136261u;
    for(int i = 0; i < length; i++) {
        hash_int ^= (uint8_t)chars[i];
        hash_int *= 16777619;
    }
    return hash_int;
}

object_string* copy_string(string chars, uint64_t length) {
    uint32_t hash = hash_string(chars, length);
    object_string* interned = table_search(&dvm.strings, chars, length, hash);

    if(interned != NULL) {
        FREE_ARRAY(char, chars, length + 1);
        return interned;
    }

    char* heap_chars = ALLOCATE(char, length + 1);
    memcpy(heap_chars, chars, length);
    heap_chars[length] = '\0';
    return allocate_string(heap_chars, length, hash);
}

object_string* get_string(mut_string chars, uint64_t length) {
    uint32_t hash = hash_string(chars, length);
    return allocate_string(chars, length, hash);
}

void print_object(value val) {
    switch (OBJ_TYPE(val))
    {
    case OBJ_STRING:
        printf("%s", AS_CSTRING(val));
        break;
    
    default:
        break;
    }
}
