from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Inner_1:
    uint8_t bytes[1];

  cdef struct Outer_1:
    Inner_1 inner;

  cdef struct Inner_2:
    uint8_t bytes[2];

  cdef struct Outer_2:
    Inner_2 inner;

  Outer_1 one();

  Outer_2 two();
