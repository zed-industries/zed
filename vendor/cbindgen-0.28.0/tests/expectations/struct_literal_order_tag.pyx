from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct ABC:
    float a;
    uint32_t b;
    uint32_t c;
  const ABC ABC_abc # = <ABC>{ 1.0, 2, 3 }
  const ABC ABC_bac # = <ABC>{ 1.0, 2, 3 }
  const ABC ABC_cba # = <ABC>{ 1.0, 2, 3 }

  cdef struct BAC:
    uint32_t b;
    float a;
    int32_t c;
  const BAC BAC_abc # = <BAC>{ 1, 2.0, 3 }
  const BAC BAC_bac # = <BAC>{ 1, 2.0, 3 }
  const BAC BAC_cba # = <BAC>{ 1, 2.0, 3 }

  void root(ABC a1, BAC a2);
