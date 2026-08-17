from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Fns:
    void (*noArgs)();
    void (*anonymousArg)(int32_t);
    int32_t (*returnsNumber)();
    int8_t (*namedArgs)(int32_t first, int16_t snd);
    int8_t (*namedArgsWildcards)(int32_t _, int16_t named, int64_t _1);

  void root(Fns _fns);

  void no_return();
