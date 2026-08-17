from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    A_A1,
    A_A2,
    A_A3,
    # Must be last for serialization purposes
    A_Sentinel,
  ctypedef uint8_t A;

  cdef enum:
    B_B1,
    B_B2,
    B_B3,
    # Must be last for serialization purposes
    B_Sentinel,
  ctypedef uint8_t B;

  cdef enum:
    C_C1,
    C_C2,
    C_C3,
    # Must be last for serialization purposes
    C_Sentinel,
  ctypedef uint8_t C_Tag;

  cdef struct C_C1_Body:
    C_Tag tag;
    uint32_t a;

  cdef struct C_C2_Body:
    C_Tag tag;
    uint32_t b;

  cdef union C:
    C_Tag tag;
    C_C1_Body c1;
    C_C2_Body c2;

  void root(A a, B b, C c);
