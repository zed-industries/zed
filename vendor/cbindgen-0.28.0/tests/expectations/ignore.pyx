from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const uint32_t NO_IGNORE_CONST # = 0

  const uint32_t NoIgnoreStructWithImpl_NO_IGNORE_INNER_CONST # = 0

  void no_ignore_root();

  void no_ignore_associated_method();
