#define MY_ASSERT(...) do { } while (0)
#define MY_ATTRS __attribute((noinline))


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct I:
    pass

  cdef enum:
    H_Foo,
    H_Bar,
    H_Baz,
  ctypedef uint8_t H_Tag;

  ctypedef struct H_Bar_Body:
    uint8_t x;
    int16_t y;

  ctypedef struct H:
    H_Tag tag;
    int16_t foo;
    H_Bar_Body bar;

  cdef enum:
    J_Foo,
    J_Bar,
    J_Baz,
  ctypedef uint8_t J_Tag;

  ctypedef struct J_Bar_Body:
    uint8_t x;
    int16_t y;

  ctypedef struct J:
    J_Tag tag;
    int16_t foo;
    J_Bar_Body bar;

  cdef enum:
    K_Foo,
    K_Bar,
    K_Baz,
  ctypedef uint8_t K_Tag;

  ctypedef struct K_Bar_Body:
    K_Tag tag;
    uint8_t x;
    int16_t y;

  ctypedef union K:
    K_Tag tag;
    int16_t foo;
    K_Bar_Body bar;

  void foo(H h, I i, J j, K k);
