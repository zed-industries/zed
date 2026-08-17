from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Bar:
    pass

  cdef struct Foo:
    int32_t a;
    uint32_t b;
  const Foo Foo_FOO # = <Foo>{ 42, 47 }
  const Foo Foo_FOO2 # = <Foo>{ 42, 47 }
  const Foo Foo_FOO3 # = <Foo>{ 42, 47 }


  const Foo BAR # = <Foo>{ 42, 1337 }



  void root(Foo x, Bar bar);
