#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "compiler/repl.h"
#include "compiler/vm.h"

void repl() {
    char line[1024];

    for(;;) {
        printf("\n$ ");

        if(!fgets(line, sizeof(line), stdin)) {
            printf("\n");
            break;
        }

        for(int i = 0; i < 1024; i++) {
            if(line[i] == '\n') line[i] = '\0';
        }
        
        interpret(line);
    }
}
