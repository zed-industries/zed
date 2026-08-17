#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C {
  X = 2,
  Y,
};
typedef uint32_t C;

typedef struct {
  int32_t _0;
} A;

typedef struct {
  int32_t x;
  float y;
} B;

typedef struct {
  uint8_t List;
  uintptr_t Of;
  B Things;
} D;

enum F_Tag {
  Foo,
  Bar,
  Baz,
};
typedef uint8_t F_Tag;

typedef struct {
  F_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef union {
  F_Tag tag;
  struct {
    F_Tag foo_tag;
    int16_t foo;
  };
  Bar_Body bar;
} F;

enum H_Tag {
  Hello,
  There,
  Everyone,
};
typedef uint8_t H_Tag;

typedef struct {
  uint8_t x;
  int16_t y;
} There_Body;

typedef struct {
  H_Tag tag;
  union {
    struct {
      int16_t hello;
    };
    There_Body there;
  };
} H;

enum I_Tag {
  ThereAgain,
  SomethingElse,
};
typedef uint8_t I_Tag;

typedef struct {
  uint8_t x;
  int16_t y;
} ThereAgain_Body;

typedef struct {
  I_Tag tag;
  union {
    ThereAgain_Body there_again;
  };
} I;

void root(A a, B b, C c, D d, F f, H h, I i);
