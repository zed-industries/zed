#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Opaque;

union Normal {
  int32_t x;
  float y;
};

union NormalWithZST {
  int32_t x;
  float y;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Opaque *a, union Normal b, union NormalWithZST c);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
