#include "lib/array.h"

void init_array(value_array* values) {
    values->count = 0;
    values->capacity = 0;
    values->values = NULL;
}

void write_array(value_array* values, value value) {
    if(values->capacity < values->count + 1) {
        grow_array(values);
    }

    values->values[values->count] = value;
    values->count++;
}

void grow_array(value_array* values) {
    int old_capacity = values->capacity;
    values->capacity = GROW_CAPACITY(old_capacity);
    values->values = GROW_ARRAY(value, values->values, old_capacity, values->capacity);
}

void free_array(value_array* values) {
    FREE_ARRAY(value, values->values, values->capacity);
    init_array(values);
}
