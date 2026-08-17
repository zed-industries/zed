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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(int32_t *arg, struct Foo_u64 *foo, struct Opaque **d);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
