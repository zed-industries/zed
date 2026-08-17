#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using ManuallyDrop = T;
#endif

#if 0
' '''
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct NotReprC_Point NotReprC_Point;

typedef NotReprC_Point Foo;

typedef struct {
  int32_t x;
  int32_t y;
} Point;

typedef struct {
  Point point;
} MyStruct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Foo *a, const MyStruct *with_manual_drop);

void take(Point with_manual_drop);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
