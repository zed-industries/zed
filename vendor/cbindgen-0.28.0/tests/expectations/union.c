#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef union {
  int32_t x;
  float y;
} Normal;

typedef union {
  int32_t x;
  float y;
} NormalWithZST;

void root(Opaque *a, Normal b, NormalWithZST c);
