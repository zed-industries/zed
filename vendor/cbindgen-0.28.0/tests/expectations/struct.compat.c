#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct {
  int32_t x;
  float y;
} Normal;

typedef struct {
  int32_t x;
  float y;
} NormalWithZST;

typedef struct {
  int32_t m0;
  float m1;
} TupleRenamed;

typedef struct {
  int32_t x;
  float y;
} TupleNamed;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Opaque *a, Normal b, NormalWithZST c, TupleRenamed d, TupleNamed e);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
