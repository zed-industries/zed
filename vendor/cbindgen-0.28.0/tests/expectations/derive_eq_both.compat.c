#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo {
  bool a;
  int32_t b;
} Foo;

enum Bar_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Baz,
  Bazz,
  FooNamed,
  FooParen,
};
#ifndef __cplusplus
typedef uint8_t Bar_Tag;
#endif // __cplusplus

typedef struct Bazz_Body {
  Bar_Tag tag;
  struct Foo named;
} Bazz_Body;

typedef struct FooNamed_Body {
  Bar_Tag tag;
  int32_t different;
  uint32_t fields;
} FooNamed_Body;

typedef struct FooParen_Body {
  Bar_Tag tag;
  int32_t _0;
  struct Foo _1;
} FooParen_Body;

typedef union Bar {
  Bar_Tag tag;
  Bazz_Body bazz;
  FooNamed_Body foo_named;
  FooParen_Body foo_paren;
} Bar;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct Foo root(union Bar aBar);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
