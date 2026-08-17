from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const int32_t FOO # = 10

  const uint32_t DELIMITER # = ':'

  const uint32_t LEFTCURLY # = '{'

  const uint32_t QUOTE # = '\''

  const uint32_t TAB # = '\t'

  const uint32_t NEWLINE # = '\n'

  const uint32_t HEART # = U'\U00002764'

  const uint32_t EQUID # = U'\U00010083'

  const float ZOM # = 3.14

  # A single-line doc comment.
  const int8_t POS_ONE # = 1

  # A
  # multi-line
  # doc
  # comment.
  const int8_t NEG_ONE # = -1

  const int64_t SHIFT # = 3

  const int64_t XBOOL # = 1

  const int64_t XFALSE # = ((0 << SHIFT) | XBOOL)

  const int64_t XTRUE # = (1 << (SHIFT | XBOOL))

  const uint8_t CAST # = <uint8_t>'A'

  const uint32_t DOUBLE_CAST # = <uint32_t><float>1

  cdef struct Foo:
    int32_t x[FOO];

  void root(Foo x);
