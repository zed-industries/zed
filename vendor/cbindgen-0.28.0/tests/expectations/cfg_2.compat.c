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
typedef struct {
  int32_t x;
} Foo;
#endif

#if defined(NOT_DEFINED)
typedef struct {
  Foo y;
} Bar;
#endif

#if defined(DEFINED)
typedef struct {
  Foo z;
} Bar;
#endif

typedef struct {
  Bar w;
} Root;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Root a);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
