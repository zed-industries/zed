from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Bar_Bar_f32:
    pass

  cdef struct Bar_Foo_f32:
    pass

  cdef struct Bar_f32:
    pass

  cdef struct Foo_i32:
    const int32_t *data;

  cdef struct Foo_f32:
    const float *data;

  cdef struct Foo_Bar_f32:
    const Bar_f32 *data;

  cdef struct Tuple_Foo_f32_____f32:
    const Foo_f32 *a;
    const float *b;

  cdef struct Tuple_f32__f32:
    const float *a;
    const float *b;

  ctypedef Tuple_f32__f32 Indirection_f32;

  void root(Foo_i32 a,
            Foo_f32 b,
            Bar_f32 c,
            Foo_Bar_f32 d,
            Bar_Foo_f32 e,
            Bar_Bar_f32 f,
            Tuple_Foo_f32_____f32 g,
            Indirection_f32 h);
