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

typedef struct NotReprC_Point Foo;

typedef struct Point {
  int32_t x;
  int32_t y;
} Point;

typedef struct MyStruct {
  struct Point point;
} MyStruct;

void root(const Foo *a, const struct MyStruct *with_manual_drop);

void take(struct Point with_manual_drop);
