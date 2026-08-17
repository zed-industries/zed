#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t x;
  int32_t y;
} Foo_i32__i32;

typedef Foo_i32__i32 IntFoo_i32;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(IntFoo_i32 a);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
