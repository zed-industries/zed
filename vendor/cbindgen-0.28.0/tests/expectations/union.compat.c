#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef union {
  int32_t x;
  float y;
} Normal;

typedef union {
  int32_t x;
  float y;
} NormalWithZST;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Opaque *a, Normal b, NormalWithZST c);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
