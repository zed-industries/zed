from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef void (*MyCallback)(uintptr_t a, uintptr_t b);

  ctypedef void (*MyOtherCallback)(uintptr_t a,
                                   uintptr_t lot,
                                   uintptr_t of,
                                   uintptr_t args,
                                   uintptr_t and_then_some);

  void my_function(MyCallback a, MyOtherCallback b);
