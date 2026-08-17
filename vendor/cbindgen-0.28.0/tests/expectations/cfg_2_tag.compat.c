#if 0
DEF DEFINED = 1
DEF NOT_DEFINED = 0
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if defined(NOT_DEFINED)
#define DEFAULT_X 8
#endif

#if defined(DEFINED)
#define DEFAULT_X 42
#endif

#if (defined(NOT_DEFINED) || defined(DEFINED))
struct Foo {
  int32_t x;
};
#endif

#if defined(NOT_DEFINED)
struct Bar {
  struct Foo y;
};
#endif

#if defined(DEFINED)
struct Bar {
  struct Foo z;
};
#endif

struct Root {
  struct Bar w;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Root a);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
