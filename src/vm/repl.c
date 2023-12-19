#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "vm/repl.h"
#include "vm/vm.h"

void repl() {
    char line[1024];

    for(;;) {
        printf("\n$ ");

        if(!fgets(line, sizeof(line), stdin)) {
            break;
        }
        
        interpret(line);
    }
}
