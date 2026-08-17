from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const uint16_t FONT_WEIGHT_FRACTION_BITS # = 6

  cdef struct FixedPoint_FONT_WEIGHT_FRACTION_BITS:
    uint16_t value;

  ctypedef FixedPoint_FONT_WEIGHT_FRACTION_BITS FontWeightFixedPoint;

  cdef struct FontWeight:
    FontWeightFixedPoint _0;
  const FontWeight FontWeight_NORMAL # = <FontWeight>{ <FontWeightFixedPoint>{ (400 << FONT_WEIGHT_FRACTION_BITS) } }

  void root(FontWeight w);
