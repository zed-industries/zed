from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # Some docs.
  extern const uint32_t FOO;

  # The root of all evil.
  #
  # But at least it contains some more documentation as someone would expect
  # from a simple test case like this.
  #
  # # Hint
  #
  # Always ensure that everything is properly documented, even if you feel lazy.
  # **Sometimes** it is also helpful to include some markdown formatting.
  #
  # ////////////////////////////////////////////////////////////////////////////
  #
  # Attention:
  #
  #    Rust is going to trim all leading `/` symbols. If you want to use them as a
  #    marker you need to add at least a single whitespace inbetween the tripple
  #    slash doc-comment marker and the rest.
  #
  void root();
