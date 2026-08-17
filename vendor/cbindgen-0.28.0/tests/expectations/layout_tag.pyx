#define CBINDGEN_PACKED     __attribute__ ((packed))
#define CBINDGEN_ALIGNED(n) __attribute__ ((aligned(n)))


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct RustAlign4Struct:
    pass

  cdef struct RustAlign4Union:
    pass

  cdef struct RustPackedStruct:
    pass

  cdef struct RustPackedUnion:
    pass

  cdef struct UnsupportedAlign4Enum:
    pass

  cdef struct UnsupportedPacked4Struct:
    pass

  cdef struct UnsupportedPacked4Union:
    pass

  cdef struct Align1Struct:
    uintptr_t arg1;
    uint8_t *arg2;

  cdef struct Align2Struct:
    uintptr_t arg1;
    uint8_t *arg2;

  cdef struct Align4Struct:
    uintptr_t arg1;
    uint8_t *arg2;

  cdef struct Align8Struct:
    uintptr_t arg1;
    uint8_t *arg2;

  cdef struct Align32Struct:
    uintptr_t arg1;
    uint8_t *arg2;

  cdef packed struct PackedStruct:
    uintptr_t arg1;
    uint8_t *arg2;

  cdef union Align1Union:
    uintptr_t variant1;
    uint8_t *variant2;

  cdef union Align4Union:
    uintptr_t variant1;
    uint8_t *variant2;

  cdef union Align16Union:
    uintptr_t variant1;
    uint8_t *variant2;

  cdef union PackedUnion:
    uintptr_t variant1;
    uint8_t *variant2;
