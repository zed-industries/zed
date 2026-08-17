#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct PREFIXBar {
  int32_t a;
} PREFIXBar;

typedef struct PREFIXFoo {
  int32_t a;
  uint32_t b;
  struct PREFIXBar bar;
} PREFIXFoo;

#define PREFIXVAL (PREFIXFoo){ .a = 42, .b = 1337, .bar = (PREFIXBar){ .a = 323 } }

void root(struct PREFIXFoo x);
