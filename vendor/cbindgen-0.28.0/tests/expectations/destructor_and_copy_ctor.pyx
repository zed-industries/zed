#define NOINLINE __attribute__((noinline))
#define NODISCARD [[nodiscard]]


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    A,
    B,
  ctypedef uint8_t FillRule;

  # This will have a destructor manually implemented via variant_body, and
  # similarly a Drop impl in Rust.
  ctypedef struct OwnedSlice_u32:
    uintptr_t len;
    uint32_t *ptr;

  ctypedef struct Polygon_u32:
    FillRule fill;
    OwnedSlice_u32 coordinates;

  # This will have a destructor manually implemented via variant_body, and
  # similarly a Drop impl in Rust.
  ctypedef struct OwnedSlice_i32:
    uintptr_t len;
    int32_t *ptr;

  cdef enum:
    Bar_u32,
    Polygon1_u32,
    Slice1_u32,
    Slice2_u32,
    Slice3_u32,
    Slice4_u32,
  ctypedef uint8_t Foo_u32_Tag;

  ctypedef struct Slice3_Body_u32:
    FillRule fill;
    OwnedSlice_u32 coords;

  ctypedef struct Slice4_Body_u32:
    FillRule fill;
    OwnedSlice_i32 coords;

  ctypedef struct Foo_u32:
    Foo_u32_Tag tag;
    Polygon_u32 polygon1;
    OwnedSlice_u32 slice1;
    OwnedSlice_i32 slice2;
    Slice3_Body_u32 slice3;
    Slice4_Body_u32 slice4;

  ctypedef struct Polygon_i32:
    FillRule fill;
    OwnedSlice_i32 coordinates;

  cdef enum:
    Bar2_i32,
    Polygon21_i32,
    Slice21_i32,
    Slice22_i32,
    Slice23_i32,
    Slice24_i32,
  ctypedef uint8_t Baz_i32_Tag;

  ctypedef struct Slice23_Body_i32:
    Baz_i32_Tag tag;
    FillRule fill;
    OwnedSlice_i32 coords;

  ctypedef struct Slice24_Body_i32:
    Baz_i32_Tag tag;
    FillRule fill;
    OwnedSlice_i32 coords;

  ctypedef union Baz_i32:
    Baz_i32_Tag tag;
    Polygon_i32 polygon21;
    OwnedSlice_i32 slice21;
    OwnedSlice_i32 slice22;
    Slice23_Body_i32 slice23;
    Slice24_Body_i32 slice24;

  cdef enum:
    Bar3,
    Taz1,
    Taz3,
  ctypedef uint8_t Taz_Tag;

  ctypedef union Taz:
    Taz_Tag tag;
    int32_t taz1;
    OwnedSlice_i32 taz3;

  cdef enum:
    Bar4,
    Taz2,
  ctypedef uint8_t Tazz_Tag;

  ctypedef union Tazz:
    Tazz_Tag tag;
    int32_t taz2;

  cdef enum:
    Bar5,
    Taz5,
  ctypedef uint8_t Tazzz_Tag;

  ctypedef union Tazzz:
    Tazzz_Tag tag;
    int32_t taz5;

  cdef enum:
    Taz6,
    Taz7,
  ctypedef uint8_t Tazzzz_Tag;

  ctypedef union Tazzzz:
    Tazzzz_Tag tag;
    int32_t taz6;
    uint32_t taz7;

  cdef enum:
    Qux1,
    Qux2,
  ctypedef uint8_t Qux_Tag;

  ctypedef union Qux:
    Qux_Tag tag;
    int32_t qux1;
    uint32_t qux2;

  void root(const Foo_u32 *a,
            const Baz_i32 *b,
            const Taz *c,
            Tazz d,
            const Tazzz *e,
            const Tazzzz *f,
            const Qux *g);
