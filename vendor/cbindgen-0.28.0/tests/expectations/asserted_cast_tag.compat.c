#define MY_ASSERT(...) do { } while (0)
#define MY_ATTRS __attribute((noinline))


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct I;

enum H_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  H_Foo,
  H_Bar,
  H_Baz,
};
#ifndef __cplusplus
typedef uint8_t H_Tag;
#endif // __cplusplus

struct H_Bar_Body {
  uint8_t x;
  int16_t y;
};

struct H {
  H_Tag tag;
  union {
    struct {
      int16_t foo;
    };
    struct H_Bar_Body bar;
  };
};

enum J_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  J_Foo,
  J_Bar,
  J_Baz,
};
#ifndef __cplusplus
typedef uint8_t J_Tag;
#endif // __cplusplus

struct J_Bar_Body {
  uint8_t x;
  int16_t y;
};

struct J {
  J_Tag tag;
  union {
    struct {
      int16_t foo;
    };
    struct J_Bar_Body bar;
  };
};

enum K_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  K_Foo,
  K_Bar,
  K_Baz,
};
#ifndef __cplusplus
typedef uint8_t K_Tag;
#endif // __cplusplus

struct K_Bar_Body {
  K_Tag tag;
  uint8_t x;
  int16_t y;
};

union K {
  K_Tag tag;
  struct {
    K_Tag foo_tag;
    int16_t foo;
  };
  struct K_Bar_Body bar;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void foo(struct H h, struct I i, struct J j, union K k);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
