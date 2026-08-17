#if 0
''' '
#endif

#ifdef __cplusplus
// These could be added as opaque types I guess.
template <typename T>
struct BuildHasherDefault;

struct DefaultHasher;
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

  cdef struct HashMap_i32__i32__BuildHasherDefault_DefaultHasher:
    pass

  cdef struct Result_Foo:
    pass

  # Fast hash map used internally.
  ctypedef HashMap_i32__i32__BuildHasherDefault_DefaultHasher FastHashMap_i32__i32;

  ctypedef FastHashMap_i32__i32 Foo;

  ctypedef Result_Foo Bar;

  void root(const Foo *a, const Bar *b);
