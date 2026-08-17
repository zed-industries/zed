#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo_____u8 {
  uint8_t *a;
};

typedef struct Foo_____u8 Boo;

struct Foo__________u8__________4 {
  uint8_t a[4];
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Boo x);

void my_function(struct Foo__________u8__________4 x);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
