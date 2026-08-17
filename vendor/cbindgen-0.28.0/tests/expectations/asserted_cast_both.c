#define MY_ASSERT(...) do { } while (0)
#define MY_ATTRS __attribute((noinline))


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct I I;

enum H_Tag {
  H_Foo,
  H_Bar,
  H_Baz,
};
typedef uint8_t H_Tag;

typedef struct H_Bar_Body {
  uint8_t x;
  int16_t y;
} H_Bar_Body;

typedef struct H {
  H_Tag tag;
  union {
    struct {
      int16_t foo;
    };
    H_Bar_Body bar;
  };
} H;

enum J_Tag {
  J_Foo,
  J_Bar,
  J_Baz,
};
typedef uint8_t J_Tag;

typedef struct J_Bar_Body {
  uint8_t x;
  int16_t y;
} J_Bar_Body;

typedef struct J {
  J_Tag tag;
  union {
    struct {
      int16_t foo;
    };
    J_Bar_Body bar;
  };
} J;

enum K_Tag {
  K_Foo,
  K_Bar,
  K_Baz,
};
typedef uint8_t K_Tag;

typedef struct K_Bar_Body {
  K_Tag tag;
  uint8_t x;
  int16_t y;
} K_Bar_Body;

typedef union K {
  K_Tag tag;
  struct {
    K_Tag foo_tag;
    int16_t foo;
  };
  K_Bar_Body bar;
} K;

void foo(struct H h, struct I i, struct J j, union K k);
