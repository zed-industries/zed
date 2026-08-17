#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo_i32__i32 {
  int32_t x;
  int32_t y;
};

typedef struct Foo_i32__i32 IntFoo_i32;

void root(IntFoo_i32 a);
