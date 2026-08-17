from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Foo_____u8:
    uint8_t *a;

  ctypedef Foo_____u8 Boo;

  cdef struct Foo__________u8__________4:
    uint8_t a[4];

  void root(Boo x);

  void my_function(Foo__________u8__________4 x);
