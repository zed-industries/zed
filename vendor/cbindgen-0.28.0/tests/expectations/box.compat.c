#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using Box = T*;
#endif

#if 0
' '''
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct NotReprC_____i32 NotReprC_____i32;

typedef NotReprC_____i32 Foo;

typedef struct {
  int32_t *number;
} MyStruct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Foo *a, const MyStruct *with_box);

void drop_box(int32_t *x);

void drop_box_opt(int32_t *x);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
