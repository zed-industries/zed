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


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Bar:
    Option_Foo foo;

  void root(Bar f);
