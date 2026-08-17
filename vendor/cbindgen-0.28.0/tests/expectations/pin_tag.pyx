#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using Pin = T;
template <typename T>
using Box = T*;
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

  cdef struct PinTest:
    int32_t *pinned_box;
    int32_t *pinned_ref;

  void root(int32_t *s, PinTest p);
