from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    Ok,
    Err,
  ctypedef uint32_t Status;

  ctypedef struct Dep:
    int32_t a;
    float b;

  ctypedef struct Foo_i32:
    int32_t a;
    int32_t b;
    Dep c;

  ctypedef Foo_i32 IntFoo;

  ctypedef struct Foo_f64:
    double a;
    double b;
    Dep c;

  ctypedef Foo_f64 DoubleFoo;

  ctypedef int32_t Unit;

  ctypedef Status SpecialStatus;

  void root(IntFoo x, DoubleFoo y, Unit z, SpecialStatus w);
