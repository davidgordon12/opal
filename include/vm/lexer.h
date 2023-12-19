#ifndef _LEXER_H
#define _LEXER_H

#include "common.h"
#include "token.h"
#include "values.h"

#define MAX_BUFFER 1024

/* 
let identifier: i32 = 15;
    ^        ^
    |       |
    L       R
Our lexer would have a left and right 'pointer', that keeps track of the current token.
L will set the token.column, R-L will be used to check if the token is reserved, then sets the literal.
*/
typedef struct {
    string left;
    string right;
    int line;
} lexer;

void init_lexer(string source);
token scan_token();

#endif
