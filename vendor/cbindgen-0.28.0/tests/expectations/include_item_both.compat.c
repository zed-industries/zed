#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A {
  int32_t x;
  float y;
} A;

typedef struct B {
  struct A data;
} B;
