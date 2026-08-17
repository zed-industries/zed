from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct StylePoint_i32:
    int32_t x;
    int32_t y;

  cdef struct StylePoint_f32:
    float x;
    float y;

  cdef enum:
    Foo_i32,
    Bar_i32,
    Baz_i32,
    Bazz_i32,
  ctypedef uint8_t StyleFoo_i32_Tag;

  cdef struct StyleFoo_Body_i32:
    StyleFoo_i32_Tag tag;
    int32_t x;
    StylePoint_i32 y;
    StylePoint_f32 z;

  cdef union StyleFoo_i32:
    StyleFoo_i32_Tag tag;
    StyleFoo_Body_i32 foo;
    int32_t bar;
    StylePoint_i32 baz;

  cdef enum StyleBar_i32_Tag:
    Bar1_i32,
    Bar2_i32,
    Bar3_i32,
    Bar4_i32,

  cdef struct StyleBar1_Body_i32:
    int32_t x;
    StylePoint_i32 y;
    StylePoint_f32 z;
    int32_t (*u)(int32_t);

  cdef struct StyleBar_i32:
    StyleBar_i32_Tag tag;
    StyleBar1_Body_i32 bar1;
    int32_t bar2;
    StylePoint_i32 bar3;

  cdef struct StylePoint_u32:
    uint32_t x;
    uint32_t y;

  cdef enum StyleBar_u32_Tag:
    Bar1_u32,
    Bar2_u32,
    Bar3_u32,
    Bar4_u32,

  cdef struct StyleBar1_Body_u32:
    int32_t x;
    StylePoint_u32 y;
    StylePoint_f32 z;
    int32_t (*u)(int32_t);

  cdef struct StyleBar_u32:
    StyleBar_u32_Tag tag;
    StyleBar1_Body_u32 bar1;
    uint32_t bar2;
    StylePoint_u32 bar3;

  cdef enum:
    Baz1,
    Baz2,
    Baz3,
  ctypedef uint8_t StyleBaz_Tag;

  cdef union StyleBaz:
    StyleBaz_Tag tag;
    StyleBar_u32 baz1;
    StylePoint_i32 baz2;

  cdef enum:
    Taz1,
    Taz2,
    Taz3,
  ctypedef uint8_t StyleTaz_Tag;

  cdef struct StyleTaz:
    StyleTaz_Tag tag;
    StyleBar_u32 taz1;
    StyleBaz taz2;

  void foo(const StyleFoo_i32 *foo,
           const StyleBar_i32 *bar,
           const StyleBaz *baz,
           const StyleTaz *taz);
