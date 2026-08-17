from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const uint64_t UNSIGNED_NEEDS_ULL_SUFFIX # = 9223372036854775808ull

  const uint64_t UNSIGNED_DOESNT_NEED_ULL_SUFFIX # = 8070450532247928832

  const int64_t SIGNED_NEEDS_ULL_SUFFIX # = -9223372036854775808ull

  const int64_t SIGNED_DOESNT_NEED_ULL_SUFFIX # = -9223372036854775807
