from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct A:
    const int32_t *data;

  cdef enum E_Tag:
    V,
    U,

  cdef struct E:
    E_Tag tag;
    const uint8_t *u;

  void root(A _a, E _e);
