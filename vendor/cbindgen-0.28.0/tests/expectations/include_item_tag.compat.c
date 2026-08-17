#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct A {
  int32_t x;
  float y;
};

struct B {
  struct A data;
};
