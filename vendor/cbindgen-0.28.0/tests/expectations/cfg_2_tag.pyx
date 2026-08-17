#if 0
DEF DEFINED = 1
DEF NOT_DEFINED = 0
#endif


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  IF NOT_DEFINED:
    const int32_t DEFAULT_X # = 8

  IF DEFINED:
    const int32_t DEFAULT_X # = 42

  IF (NOT_DEFINED or DEFINED):
    cdef struct Foo:
      int32_t x;

  IF NOT_DEFINED:
    cdef struct Bar:
      Foo y;

  IF DEFINED:
    cdef struct Bar:
      Foo z;

  cdef struct Root:
    Bar w;

  void root(Root a);
