#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo {
  bool a;
  int32_t b;
};

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

struct Bazz_Body {
  Bar_Tag tag;
  struct Foo named;
};

struct FooNamed_Body {
  Bar_Tag tag;
  int32_t different;
  uint32_t fields;
};

struct FooParen_Body {
  Bar_Tag tag;
  int32_t _0;
  struct Foo _1;
};

union Bar {
  Bar_Tag tag;
  struct Bazz_Body bazz;
  struct FooNamed_Body foo_named;
  struct FooParen_Body foo_paren;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct Foo root(union Bar aBar);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
