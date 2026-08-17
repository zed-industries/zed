from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Option_i32:
    pass

  ctypedef struct Result_i32__String:
    pass

  ctypedef struct Vec_String:
    pass

  void root(const Vec_String *a, const Option_i32 *b, const Result_i32__String *c);
