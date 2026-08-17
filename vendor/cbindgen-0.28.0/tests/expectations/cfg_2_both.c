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
typedef struct Foo {
  int32_t x;
} Foo;
#endif

#if defined(NOT_DEFINED)
typedef struct Bar {
  struct Foo y;
} Bar;
#endif

#if defined(DEFINED)
typedef struct Bar {
  struct Foo z;
} Bar;
#endif

typedef struct Root {
  struct Bar w;
} Root;

void root(struct Root a);
