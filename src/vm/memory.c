#include <stdlib.h>

#include "vm/memory.h"
#include "vm/vm.h"
#include "vm/object.h"

/*
  The two size arguments passed to reallocate() control which operation to perform:

  Old Capacity 	   New Capacity 	             Operation
  0 	             Non‑zero 	                 Allocate new block.
  Non‑zero 	       0 	                         Free allocation.
  Non‑zero 	       Smaller than oldSize 	     Shrink existing allocation.
  Non‑zero 	       Larger than oldSize 	       Grow existing allocation.
*/
void* reallocate(void* pointer, size_t old_capacity, size_t new_capacity) {
    if(new_capacity == 0) {
        free(pointer);
        return NULL;
    }

    void* result = NULL;
    
    result = realloc(pointer, new_capacity);

    if(result == NULL) { 
        PANIC("Allocation failed. Not enough memory");
    }

    return result;
}

static void free_object(object* obj) {
    switch(obj->type) {
    case OBJ_STRING: {
        object_string* str = (object_string*)obj;
        FREE_ARRAY(char, str->chars, str->length + 1);
        FREE(object_string, obj);
        break;
    }
    }
}

void free_objects() {
    object* obj = dvm.objs;
    while(obj != NULL) {
        object* next = obj->next;
        free_object(obj);
        obj = next;
    }
}