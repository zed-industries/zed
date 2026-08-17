from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list
#ifndef NO_RETURN_ATTR
  #ifdef __GNUC__
    #define NO_RETURN_ATTR __attribute__ ((noreturn))
  #else // __GNUC__
    #define NO_RETURN_ATTR
  #endif // __GNUC__
#endif // NO_RETURN_ATTR


cdef extern from *:

  cdef struct Example:
    void (*f)(uintptr_t, uintptr_t);

  void loop_forever();

  uint8_t normal_return(Example arg, void (*other)(uint8_t));
