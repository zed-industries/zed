#ifndef Decomposed_D_H
#define Decomposed_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Decomposed {
  char32_t first;
  char32_t second;
} Decomposed;

typedef struct Decomposed_option {union { Decomposed ok; }; bool is_ok; } Decomposed_option;



#endif // Decomposed_D_H
