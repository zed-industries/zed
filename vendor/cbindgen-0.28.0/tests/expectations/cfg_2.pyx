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
    ctypedef struct Foo:
      int32_t x;

  IF NOT_DEFINED:
    ctypedef struct Bar:
      Foo y;

  IF DEFINED:
    ctypedef struct Bar:
      Foo z;

  ctypedef struct Root:
    Bar w;

  void root(Root a);
