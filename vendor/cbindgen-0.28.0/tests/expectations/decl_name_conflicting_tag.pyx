from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    Buffer # = 0,
    NotBuffer # = 1,
  ctypedef uint32_t BindingType;

  cdef struct BindGroupLayoutEntry:
    BindingType ty;

  void root(BindGroupLayoutEntry entry);
