from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct PREFIXBar:
    int32_t a;

  cdef struct PREFIXFoo:
    int32_t a;
    uint32_t b;
    PREFIXBar bar;

  const PREFIXFoo PREFIXVAL # = <PREFIXFoo>{ 42, 1337, <PREFIXBar>{ 323 } }

  void root(PREFIXFoo x);
