from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct A:
    pass

  ctypedef struct B:
    pass

  ctypedef struct List_A:
    A *members;
    uintptr_t count;

  ctypedef struct List_B:
    B *members;
    uintptr_t count;

  void foo(List_A a);

  void bar(List_B b);
