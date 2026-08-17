#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo_Bar {
  const int32_t *something;
};

struct Bar {
  int32_t something;
  struct Foo_Bar subexpressions;
};

void root(struct Bar b);
