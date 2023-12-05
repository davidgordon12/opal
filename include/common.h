#ifndef _COMMON_H
#define _COMMON_H

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define PANIC(message)                                                         \
    printf(message);                                                           \
    printf("\n");                                                              \
    exit(1);

#endif
