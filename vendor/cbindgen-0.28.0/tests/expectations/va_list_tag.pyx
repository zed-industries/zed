from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef int32_t (*VaListFnPtr)(int32_t count, ...);

  ctypedef int32_t (*VaListFnPtr2)(int32_t count, ...);

  cdef struct Interface_______i32_______i32_______va_list:
    int32_t (*fn1)(int32_t count, ...);

  int32_t va_list_test(int32_t count, ...);

  int32_t va_list_test2(int32_t count, ...);

  void va_list_fn_ptrs(int32_t (*fn1)(int32_t count, ...),
                       int32_t (*fn2)(int32_t count, ...),
                       VaListFnPtr fn3,
                       VaListFnPtr2 fn4,
                       Interface_______i32_______i32_______va_list fn5,
                       Interface_______i32_______i32_______va_list fn6);
