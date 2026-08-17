from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Parser_40__41:
    uint8_t *buf;
    uintptr_t len;

  ctypedef struct Parser_123__125:
    uint8_t *buf;
    uintptr_t len;

  void init_parens_parser(Parser_40__41 *p, uint8_t *buf, uintptr_t len);

  void destroy_parens_parser(Parser_40__41 *p);

  void init_braces_parser(Parser_123__125 *p, uint8_t *buf, uintptr_t len);
