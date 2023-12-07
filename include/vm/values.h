#ifndef _VALUE_H
#define _VALUE_H

#include <stdbool.h>

#include "common.h"

typedef const char* string;
typedef char* mut_string;

typedef struct object object;
typedef struct  object_string object_string;

typedef enum {
    VAL_BOOL,
    VAL_NONE, 
    VAL_NUMBER,
    VAL_OBJ,
} value_type;

typedef struct {
    value_type type;
    union {
        bool boolean;
        double number;
        object* obj;
    } as;
} value;

#define NUMBER_VAL(val) ((value){VAL_NUMBER, {.number = val}})
#define BOOL_VAL(val)   ((value){VAL_BOOL, {.boolean = val}})
#define NONE_VAL           ((value){VAL_NONE, {.number = 0}})
#define OBJ_VAL(object)   ((Value){VAL_OBJ, {.obj = (object*)obj}})

#define AS_BOOL(val)    ((val).as.boolean)
#define AS_NUMBER(val)  ((val).as.number)
#define AS_OBJ(val)     ((val).as.obj)

#define IS_BOOL(val)    ((val).type == VAL_BOOL)
#define IS_NONE(val)     ((val).type == VAL_NONE)
#define IS_NUMBER(val)  ((val).type == VAL_NUMBER)
#define IS_OBJ(value)     ((value).type == VAL_OBJ)

#endif
