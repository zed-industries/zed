#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum Bar {
  BarSome,
  BarThing,
} Bar;

typedef struct FooU8 {
  uint8_t a;
} FooU8;

typedef struct FooU8 Boo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Boo x, enum Bar y);

void unsafe_root(Boo x, enum Bar y);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
