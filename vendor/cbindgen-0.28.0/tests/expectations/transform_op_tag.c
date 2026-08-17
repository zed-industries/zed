#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct StylePoint_i32 {
  int32_t x;
  int32_t y;
};

struct StylePoint_f32 {
  float x;
  float y;
};

enum StyleFoo_i32_Tag {
  Foo_i32,
  Bar_i32,
  Baz_i32,
  Bazz_i32,
};
typedef uint8_t StyleFoo_i32_Tag;

struct StyleFoo_Body_i32 {
  StyleFoo_i32_Tag tag;
  int32_t x;
  struct StylePoint_i32 y;
  struct StylePoint_f32 z;
};

union StyleFoo_i32 {
  StyleFoo_i32_Tag tag;
  struct StyleFoo_Body_i32 foo;
  struct {
    StyleFoo_i32_Tag bar_tag;
    int32_t bar;
  };
  struct {
    StyleFoo_i32_Tag baz_tag;
    struct StylePoint_i32 baz;
  };
};

enum StyleBar_i32_Tag {
  Bar1_i32,
  Bar2_i32,
  Bar3_i32,
  Bar4_i32,
};

struct StyleBar1_Body_i32 {
  int32_t x;
  struct StylePoint_i32 y;
  struct StylePoint_f32 z;
  int32_t (*u)(int32_t);
};

struct StyleBar_i32 {
  enum StyleBar_i32_Tag tag;
  union {
    struct StyleBar1_Body_i32 bar1;
    struct {
      int32_t bar2;
    };
    struct {
      struct StylePoint_i32 bar3;
    };
  };
};

struct StylePoint_u32 {
  uint32_t x;
  uint32_t y;
};

enum StyleBar_u32_Tag {
  Bar1_u32,
  Bar2_u32,
  Bar3_u32,
  Bar4_u32,
};

struct StyleBar1_Body_u32 {
  int32_t x;
  struct StylePoint_u32 y;
  struct StylePoint_f32 z;
  int32_t (*u)(int32_t);
};

struct StyleBar_u32 {
  enum StyleBar_u32_Tag tag;
  union {
    struct StyleBar1_Body_u32 bar1;
    struct {
      uint32_t bar2;
    };
    struct {
      struct StylePoint_u32 bar3;
    };
  };
};

enum StyleBaz_Tag {
  Baz1,
  Baz2,
  Baz3,
};
typedef uint8_t StyleBaz_Tag;

union StyleBaz {
  StyleBaz_Tag tag;
  struct {
    StyleBaz_Tag baz1_tag;
    struct StyleBar_u32 baz1;
  };
  struct {
    StyleBaz_Tag baz2_tag;
    struct StylePoint_i32 baz2;
  };
};

enum StyleTaz_Tag {
  Taz1,
  Taz2,
  Taz3,
};
typedef uint8_t StyleTaz_Tag;

struct StyleTaz {
  StyleTaz_Tag tag;
  union {
    struct {
      struct StyleBar_u32 taz1;
    };
    struct {
      union StyleBaz taz2;
    };
  };
};

void foo(const union StyleFoo_i32 *foo,
         const struct StyleBar_i32 *bar,
         const union StyleBaz *baz,
         const struct StyleTaz *taz);
