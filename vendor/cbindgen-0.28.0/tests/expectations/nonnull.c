#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct {
  float *a;
  uint64_t *b;
  Opaque *c;
  uint64_t **d;
  float **e;
  Opaque **f;
  uint64_t *g;
  int32_t *h;
  int32_t **i;
} Foo_u64;

void root(int32_t *arg, Foo_u64 *foo, Opaque **d);
