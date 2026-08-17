from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Foo:
    bool a;
    int32_t b;

  cdef enum:
    Baz,
    Bazz,
    FooNamed,
    FooParen,
  ctypedef uint8_t Bar_Tag;

  cdef struct Bazz_Body:
    Bar_Tag tag;
    Foo named;

  cdef struct FooNamed_Body:
    Bar_Tag tag;
    int32_t different;
    uint32_t fields;

  cdef struct FooParen_Body:
    Bar_Tag tag;
    int32_t _0;
    Foo _1;

  cdef union Bar:
    Bar_Tag tag;
    Bazz_Body bazz;
    FooNamed_Body foo_named;
    FooParen_Body foo_paren;

  Foo root(Bar aBar);
