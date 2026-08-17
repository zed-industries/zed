#if 0
''' '
#endif

#ifdef __cplusplus
struct NonZeroI64;
#endif

#if 0
' '''
#endif


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Option_i64:
    pass

  cdef struct NonZeroAliases:
    uint8_t a;
    uint16_t b;
    uint32_t c;
    uint64_t d;
    int8_t e;
    int16_t f;
    int32_t g;
    int64_t h;
    int64_t i;
    const Option_i64 *j;

  cdef struct NonZeroGenerics:
    uint8_t a;
    uint16_t b;
    uint32_t c;
    uint64_t d;
    int8_t e;
    int16_t f;
    int32_t g;
    int64_t h;
    int64_t i;
    const Option_i64 *j;

  void root_nonzero_aliases(NonZeroAliases test,
                            uint8_t a,
                            uint16_t b,
                            uint32_t c,
                            uint64_t d,
                            int8_t e,
                            int16_t f,
                            int32_t g,
                            int64_t h,
                            int64_t i,
                            const Option_i64 *j);

  void root_nonzero_generics(NonZeroGenerics test,
                             uint8_t a,
                             uint16_t b,
                             uint32_t c,
                             uint64_t d,
                             int8_t e,
                             int16_t f,
                             int32_t g,
                             int64_t h,
                             int64_t i,
                             const Option_i64 *j);
