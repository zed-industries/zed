#if 0
DEF FOO = 0
DEF BAR = 0
#endif


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  IF FOO:
    const int32_t FOO # = 1

  IF BAR:
    const int32_t BAR # = 2

  IF FOO:
    ctypedef struct Foo:
      pass

  IF BAR:
    ctypedef struct Bar:
      pass

  IF FOO:
    void foo(const Foo *foo);

  IF BAR:
    void bar(const Bar *bar);
