#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Opaque;

struct Foo_u64 {
  float *a;
  uint64_t *b;
  struct Opaque *c;
  uint64_t **d;
  float **e;
  struct Opaque **f;
  uint64_t *g;
  int32_t *h;
  int32_t **i;
};

void root(int32_t *arg, struct Foo_u64 *foo, struct Opaque **d);
