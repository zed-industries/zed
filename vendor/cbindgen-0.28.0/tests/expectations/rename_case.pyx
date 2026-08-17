from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  void test_camel_case(int32_t fooBar);

  void test_pascal_case(int32_t FooBar);

  void test_snake_case(int32_t foo_bar);

  void test_screaming_snake_case(int32_t FOO_BAR);

  void test_gecko_case(int32_t aFooBar);

  void test_prefix(int32_t prefix_foo_bar);
