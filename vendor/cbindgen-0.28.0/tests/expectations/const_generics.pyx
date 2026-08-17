from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const uintptr_t TITLE_SIZE # = 80

  ctypedef int8_t CArrayString_TITLE_SIZE[TITLE_SIZE];

  ctypedef int8_t CArrayString_40[40];

  ctypedef struct Book:
    CArrayString_TITLE_SIZE title;
    CArrayString_40 author;

  void root(Book *a);
