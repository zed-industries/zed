from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const int8_t FOURTY_FOUR # = 4

  cdef enum:
    A # = 1,
    B # = -1,
    C # = (1 + 2),
    D # = FOURTY_FOUR,
    F # = 5,
    G # = <int8_t>54,
    H # = <int8_t>False,
  ctypedef int8_t E;

  void root(const E*);
