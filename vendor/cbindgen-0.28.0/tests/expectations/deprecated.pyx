#define DEPRECATED_FUNC __attribute__((deprecated))
#define DEPRECATED_STRUCT __attribute__((deprecated))
#define DEPRECATED_ENUM __attribute__((deprecated))
#define DEPRECATED_ENUM_VARIANT __attribute__((deprecated))
#define DEPRECATED_FUNC_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_STRUCT_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_ENUM_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_ENUM_VARIANT_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    A # = 0,
  ctypedef int32_t DeprecatedEnum;

  cdef enum:
    B # = 0,
  ctypedef int32_t DeprecatedEnumWithNote;

  cdef enum:
    C # = 0,
    D # = 1,
    E # = 2,
    F # = 3,
  ctypedef int32_t EnumWithDeprecatedVariants;

  ctypedef struct DeprecatedStruct:
    int32_t a;

  ctypedef struct DeprecatedStructWithNote:
    int32_t a;

  cdef enum:
    Foo,
    Bar,
    Baz,
  ctypedef uint8_t EnumWithDeprecatedStructVariants_Tag;

  ctypedef struct Bar_Body:
    EnumWithDeprecatedStructVariants_Tag tag;
    uint8_t x;
    int16_t y;

  ctypedef struct Baz_Body:
    EnumWithDeprecatedStructVariants_Tag tag;
    uint8_t x;
    uint8_t y;

  ctypedef union EnumWithDeprecatedStructVariants:
    EnumWithDeprecatedStructVariants_Tag tag;
    int16_t foo;
    Bar_Body bar;
    Baz_Body baz;

  void deprecated_without_note();

  void deprecated_without_bracket();

  void deprecated_with_note();

  void deprecated_with_note_and_since();

  void deprecated_with_note_which_requires_to_be_escaped();

  void dummy(DeprecatedEnum a,
             DeprecatedEnumWithNote b,
             EnumWithDeprecatedVariants c,
             DeprecatedStruct d,
             DeprecatedStructWithNote e,
             EnumWithDeprecatedStructVariants f);
