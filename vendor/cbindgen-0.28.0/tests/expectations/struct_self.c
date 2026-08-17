#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const int32_t *something;
} Foo_Bar;

typedef struct {
  int32_t something;
  Foo_Bar subexpressions;
} Bar;

void root(Bar b);
