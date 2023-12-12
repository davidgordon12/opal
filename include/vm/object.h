#ifndef _OBJECT_H
#define _OBJECT_H

#include "values.h"

#define OBJ_TYPE(val) (AS_OBJ(val)->type)

#define IS_STRING(val) (is_obj_type(val, OBJ_STRING))

#define AS_STRING(val) ((object_string*)AS_OBJ(val))
#define AS_CSTRING(val) (((object_string*)AS_OBJ(val))->chars)

object_string* copy_string(string chars, uint64_t length);
object_string* get_string(mut_string chars, uint64_t length);
void print_object(value val);

typedef enum {
    OBJ_STRING,
} object_type;

struct object {
    object_type type;
    struct object* next;
};

struct object_string {
    object obj;
    uint64_t length;
    mut_string chars;
    uint32_t hash;
};

static inline bool is_obj_type(value val, object_type type) {
    return IS_OBJ(val) && AS_OBJ(val)->type == type;
}

#endif
