#define CBINDGEN_PACKED        __attribute__ ((packed))
#define CBINDGEN_ALIGNED(n)    __attribute__ ((aligned(n)))


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct OpaqueAlign16Union:
    pass

  cdef struct OpaqueAlign1Struct:
    pass

  cdef struct OpaqueAlign1Union:
    pass

  cdef struct OpaqueAlign2Struct:
    pass

  cdef struct OpaqueAlign32Struct:
    pass

  cdef struct OpaqueAlign4Struct:
    pass

  cdef struct OpaqueAlign4Union:
    pass

  cdef struct OpaqueAlign8Struct:
    pass

  cdef packed struct PackedStruct:
    uintptr_t arg1;
    uint8_t *arg2;

  cdef union PackedUnion:
    uintptr_t variant1;
    uint8_t *variant2;
