#if 0
DEF DEFINE_FREEBSD = 0
#endif


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Foo:
    int32_t x;

  ctypedef struct RenamedTy:
    uint64_t y;

  IF not DEFINE_FREEBSD:
    ctypedef struct NoExternTy:
      uint8_t field;

  IF not DEFINE_FREEBSD:
    ctypedef struct ContainsNoExternTy:
      NoExternTy field;

  IF DEFINE_FREEBSD:
    ctypedef struct ContainsNoExternTy:
      uint64_t field;

  void root(Foo a);

  void renamed_func(RenamedTy a);

  void no_extern_func(ContainsNoExternTy a);
