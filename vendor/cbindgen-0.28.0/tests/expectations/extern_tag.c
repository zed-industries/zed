#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Normal {
  int32_t x;
  float y;
};

extern int32_t foo(void);

extern void bar(struct Normal a);

extern int32_t baz(void);
