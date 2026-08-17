from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct ArrayVec_____u8__100:
    uint8_t *xs[100];
    uint32_t len;

  int32_t push(ArrayVec_____u8__100 *v, uint8_t *elem);
