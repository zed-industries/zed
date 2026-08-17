#include <stdint.h>

#if 0
''' '
#endif

typedef uint64_t Option_Foo;

#if 0
' '''
#endif

#if 0
from libc.stdint cimport uint64_t
ctypedef uint64_t Option_Foo
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Bar {
  Option_Foo foo;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Bar f);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
