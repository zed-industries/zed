#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  bool a;
  int32_t b;
} Foo;

enum Bar_Tag {
  Baz,
  Bazz,
  FooNamed,
  FooParen,
};
typedef uint8_t Bar_Tag;

typedef struct {
  Bar_Tag tag;
  Foo named;
} Bazz_Body;

typedef struct {
  Bar_Tag tag;
  int32_t different;
  uint32_t fields;
} FooNamed_Body;

typedef struct {
  Bar_Tag tag;
  int32_t _0;
  Foo _1;
} FooParen_Body;

typedef union {
  Bar_Tag tag;
  Bazz_Body bazz;
  FooNamed_Body foo_named;
  FooParen_Body foo_paren;
} Bar;

Foo root(Bar aBar);
