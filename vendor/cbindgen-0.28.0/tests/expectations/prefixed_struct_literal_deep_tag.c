#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct PREFIXBar {
  int32_t a;
};

struct PREFIXFoo {
  int32_t a;
  uint32_t b;
  struct PREFIXBar bar;
};

#define PREFIXVAL (PREFIXFoo){ .a = 42, .b = 1337, .bar = (PREFIXBar){ .a = 323 } }

void root(struct PREFIXFoo x);
