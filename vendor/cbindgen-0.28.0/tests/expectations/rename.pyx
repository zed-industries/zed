from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const int32_t C_H # = 10

  cdef enum:
    x # = 0,
    y # = 1,
  ctypedef uint8_t C_E;

  ctypedef struct C_A:
    pass

  ctypedef struct C_C:
    pass

  ctypedef struct C_AwesomeB:
    int32_t x;
    float y;

  ctypedef union C_D:
    int32_t x;
    float y;

  ctypedef C_A C_F;

  const intptr_t C_I # = <intptr_t><C_F*>10

  extern const int32_t G;

  void root(const C_A *a, C_AwesomeB b, C_C c, C_D d, C_E e, C_F f);
