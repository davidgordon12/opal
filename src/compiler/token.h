#ifndef _TOKEN_H
#define _TOKEN_H

#include <stdbool.h>

#include "token_type.h"
#include "lib/values.h"
#include "common.h"

typedef struct token {
    token_type type;
    string start;
    int length;
    int line; // Line number
} token;

bool is_reserved(token_type);

#endif
