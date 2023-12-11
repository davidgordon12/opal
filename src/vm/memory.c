#include <stdlib.h>

#include "vm/memory.h"

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