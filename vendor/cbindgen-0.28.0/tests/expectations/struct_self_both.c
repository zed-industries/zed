#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo_Bar {
  const int32_t *something;
} Foo_Bar;

typedef struct Bar {
  int32_t something;
  struct Foo_Bar subexpressions;
} Bar;

void root(struct Bar b);
