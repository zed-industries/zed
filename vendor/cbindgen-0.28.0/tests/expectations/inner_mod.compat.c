#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  float x;
} Foo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Foo a);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
