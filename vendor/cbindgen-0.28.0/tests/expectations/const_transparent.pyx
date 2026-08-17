from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef uint8_t TransparentStruct;
  const int64_t TransparentStruct_ASSOC_STRUCT_FOO # = 1
  const TransparentStruct TransparentStruct_ASSOC_STRUCT_BAR # = 2


  ctypedef uint8_t TransparentTupleStruct;

  const TransparentStruct STRUCT_FOO # = 4

  const TransparentTupleStruct STRUCT_BAR # = 5




