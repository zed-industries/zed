#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint8_t *a;
} Foo_____u8;

typedef Foo_____u8 Boo;

typedef struct {
  uint8_t a[4];
} Foo__________u8__________4;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Boo x);

void my_function(Foo__________u8__________4 x);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
