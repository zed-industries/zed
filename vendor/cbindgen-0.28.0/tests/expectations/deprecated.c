#define DEPRECATED_FUNC __attribute__((deprecated))
#define DEPRECATED_STRUCT __attribute__((deprecated))
#define DEPRECATED_ENUM __attribute__((deprecated))
#define DEPRECATED_ENUM_VARIANT __attribute__((deprecated))
#define DEPRECATED_FUNC_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_STRUCT_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_ENUM_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_ENUM_VARIANT_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum DEPRECATED_ENUM DeprecatedEnum {
  A = 0,
};
typedef int32_t DeprecatedEnum;

enum DEPRECATED_ENUM_WITH_NOTE("This is a note") DeprecatedEnumWithNote {
  B = 0,
};
typedef int32_t DeprecatedEnumWithNote;

enum EnumWithDeprecatedVariants {
  C = 0,
  D DEPRECATED_ENUM_VARIANT = 1,
  E DEPRECATED_ENUM_VARIANT_WITH_NOTE("This is a note") = 2,
  F DEPRECATED_ENUM_VARIANT_WITH_NOTE("This is a note") = 3,
};
typedef int32_t EnumWithDeprecatedVariants;

typedef struct DEPRECATED_STRUCT {
  int32_t a;
} DeprecatedStruct;

typedef struct DEPRECATED_STRUCT_WITH_NOTE("This is a note") {
  int32_t a;
} DeprecatedStructWithNote;

enum EnumWithDeprecatedStructVariants_Tag {
  Foo,
  Bar DEPRECATED_ENUM_VARIANT,
  Baz DEPRECATED_ENUM_VARIANT_WITH_NOTE("This is a note"),
};
typedef uint8_t EnumWithDeprecatedStructVariants_Tag;

typedef struct DEPRECATED_STRUCT {
  EnumWithDeprecatedStructVariants_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef struct DEPRECATED_STRUCT_WITH_NOTE("This is a note") {
  EnumWithDeprecatedStructVariants_Tag tag;
  uint8_t x;
  uint8_t y;
} Baz_Body;

typedef union {
  EnumWithDeprecatedStructVariants_Tag tag;
  struct {
    EnumWithDeprecatedStructVariants_Tag foo_tag;
    int16_t foo;
  };
  Bar_Body bar;
  Baz_Body baz;
} EnumWithDeprecatedStructVariants;

DEPRECATED_FUNC void deprecated_without_note(void);

DEPRECATED_FUNC_WITH_NOTE("This is a note") void deprecated_without_bracket(void);

DEPRECATED_FUNC_WITH_NOTE("This is a note") void deprecated_with_note(void);

DEPRECATED_FUNC_WITH_NOTE("This is a note") void deprecated_with_note_and_since(void);

DEPRECATED_FUNC_WITH_NOTE("This quote \" requires to be quoted, and this [\n] requires to be escaped")
void deprecated_with_note_which_requires_to_be_escaped(void);

void dummy(DeprecatedEnum a,
           DeprecatedEnumWithNote b,
           EnumWithDeprecatedVariants c,
           DeprecatedStruct d,
           DeprecatedStructWithNote e,
           EnumWithDeprecatedStructVariants f);
