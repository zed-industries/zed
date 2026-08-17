from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Foo:
    bool a;
    int32_t b;

  cdef enum:
    Baz,
    Bazz,
    FooNamed,
    FooParen,
  ctypedef uint8_t Bar_Tag;

  ctypedef struct Bazz_Body:
    Bar_Tag tag;
    Foo named;

  ctypedef struct FooNamed_Body:
    Bar_Tag tag;
    int32_t different;
    uint32_t fields;

  ctypedef struct FooParen_Body:
    Bar_Tag tag;
    int32_t _0;
    Foo _1;

  ctypedef union Bar:
    Bar_Tag tag;
    Bazz_Body bazz;
    FooNamed_Body foo_named;
    FooParen_Body foo_paren;

  Foo root(Bar aBar);
