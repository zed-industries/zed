from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const int64_t CONSTANT_I64 # = 216

  const float CONSTANT_FLOAT32 # = 312.292

  const uint32_t DELIMITER # = ':'

  const uint32_t LEFTCURLY # = '{'

  ctypedef struct Foo:
    int32_t x;
  const int64_t Foo_CONSTANT_I64_BODY # = 216

  const Foo SomeFoo # = <Foo>{ 99 }
