#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef union Normal {
  int32_t x;
  float y;
} Normal;

typedef union NormalWithZST {
  int32_t x;
  float y;
} NormalWithZST;

void root(struct Opaque *a, union Normal b, union NormalWithZST c);
