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

        for(int i = 0; i < 1024; i++) {
            switch(line[i]) {
            case '\r':
            case '\t':
            case '\n':
                line[i] = '\0';
            }
        }

        interpret(line);
    }
}
