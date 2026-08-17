from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const int32_t FOO # = 10

  const float ZOM # = 3.14

  ctypedef struct Foo:
    int32_t x[FOO];

  void root(Foo x);
