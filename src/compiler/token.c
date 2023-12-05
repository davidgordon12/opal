#include "compiler/token.h"
#include "compiler/token_type.h"

const token_type reserved[8] = { 
  TOKEN_PROC, 
  TOKEN_LET, 
  TOKEN_TRUE, 
  TOKEN_FALSE, 
  TOKEN_IF, 
  TOKEN_ELSE, 
  TOKEN_WHILE, 
  TOKEN_RETURN 
};

bool is_reserved(token_type type) {
    for(int i = 0; i < 8; ++i) {
        if(type == reserved[i]) return true;
    }

    return false;
}