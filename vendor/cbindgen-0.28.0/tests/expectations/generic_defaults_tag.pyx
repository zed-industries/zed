from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef int16_t Foo_i16;

  ctypedef int32_t Foo_i32;

  cdef struct Bar_i32__u32:
    Foo_i32 f;
    uint32_t p;

  ctypedef int64_t Foo_i64;

  ctypedef Foo_i64 Baz_i64;

  void foo_root(Foo_i16 f, Bar_i32__u32 b, Baz_i64 z);
