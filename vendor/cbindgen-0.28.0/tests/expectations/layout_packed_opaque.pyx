#define CBINDGEN_PACKED        __attribute__ ((packed))
#define CBINDGEN_ALIGNED(n)    __attribute__ ((aligned(n)))


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct OpaquePackedStruct:
    pass

  ctypedef struct OpaquePackedUnion:
    pass

  ctypedef union Align1Union:
    uintptr_t variant1;
    uint8_t *variant2;

  ctypedef union Align4Union:
    uintptr_t variant1;
    uint8_t *variant2;

  ctypedef union Align16Union:
    uintptr_t variant1;
    uint8_t *variant2;

  ctypedef struct Align1Struct:
    uintptr_t arg1;
    uint8_t *arg2;

  ctypedef struct Align2Struct:
    uintptr_t arg1;
    uint8_t *arg2;

  ctypedef struct Align4Struct:
    uintptr_t arg1;
    uint8_t *arg2;

  ctypedef struct Align8Struct:
    uintptr_t arg1;
    uint8_t *arg2;

  ctypedef struct Align32Struct:
    uintptr_t arg1;
    uint8_t *arg2;
