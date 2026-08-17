from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct A:
    int32_t namespace_;
    float float_;

  cdef struct B:
    int32_t namespace_;
    float float_;

  cdef enum:
    D,
  ctypedef uint8_t C_Tag;

  cdef struct D_Body:
    int32_t namespace_;
    float float_;

  cdef struct C:
    C_Tag tag;
    D_Body d;

  cdef enum:
    Double,
    Float,
  ctypedef uint8_t E_Tag;

  cdef struct E:
    E_Tag tag;
    double double_;
    float float_;

  cdef enum:
    double_,
    float_,
  ctypedef uint8_t F_Tag;

  cdef struct F:
    F_Tag tag;
    double double_;
    float float_;

  void root(A a, B b, C c, E e, F f, int32_t namespace_, float float_);
