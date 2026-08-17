#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using MaybeUninit = T;
#endif

#if 0
' '''
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct NotReprC______i32;

typedef struct NotReprC______i32 Foo;

struct MyStruct {
  const int32_t *number;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Foo *a, const struct MyStruct *with_maybe_uninit);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
