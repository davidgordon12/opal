#ifndef _MEMORY_H
#define _MEMORY_H

#include "common.h"

#define FREE(type, pointer) reallocate(pointer, sizeof(type), 0)

#define GROW_CAPACITY(capacity) \
    ((capacity) < 8 ? 8 : (capacity) * 2)

#define GROW_ARRAY(type, pointer, old_capacity, new_capacity) \
    (type*)reallocate(pointer, sizeof(type) * (old_capacity), \
        sizeof(type) * (new_capacity))

#define FREE_ARRAY(type, pointer, old_capacity) \
    reallocate(pointer, sizeof(type) * old_capacity, 0)

#define ALLOCATE(type, count) \
    (type*)reallocate(NULL, 0, sizeof(type) * count)

void* reallocate(void* pointer, size_t old_capacity, size_t new_capacity);
void free_objects();

#endif
