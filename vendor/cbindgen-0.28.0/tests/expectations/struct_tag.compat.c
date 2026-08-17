#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Opaque;

struct Normal {
  int32_t x;
  float y;
};

struct NormalWithZST {
  int32_t x;
  float y;
};

struct TupleRenamed {
  int32_t m0;
  float m1;
};

struct TupleNamed {
  int32_t x;
  float y;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Opaque *a,
          struct Normal b,
          struct NormalWithZST c,
          struct TupleRenamed d,
          struct TupleNamed e);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
