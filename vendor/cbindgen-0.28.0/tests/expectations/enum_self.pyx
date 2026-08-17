from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Foo_Bar:
    const int32_t *something;

  cdef enum:
    Min,
    Max,
    Other,
  ctypedef uint8_t Bar_Tag;

  ctypedef union Bar:
    Bar_Tag tag;
    Foo_Bar min;
    Foo_Bar max;

  void root(Bar b);
