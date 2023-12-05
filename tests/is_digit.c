#include <stdio.h>
#include <stdbool.h>

static bool is_digit(char c);

int main() {
    char a = 4;
    printf("%d", is_digit(a));

}

static bool is_digit(char c) {
    return c >= '0' && c <= '9';
}