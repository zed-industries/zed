from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  #With doc attr, each attr contribute to one line of document
  #like this one with a new line character at its end
  #and this one as well. So they are in the same paragraph
  #
  #Line ends with one new line should not break
  #
  #Line ends with two spaces and a new line
  #should break to next line
  #
  #Line ends with two new lines
  #
  #Should break to next paragraph
  void root();
