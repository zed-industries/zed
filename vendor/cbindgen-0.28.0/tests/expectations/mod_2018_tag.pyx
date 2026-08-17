from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const uint8_t EXPORT_ME_TOO # = 42

  cdef struct ExportMe:
    uint64_t val;

  cdef struct ExportMe2:
    uint64_t val;

  void export_me(ExportMe *val);

  void export_me_2(ExportMe2*);

  void from_really_nested_mod();
