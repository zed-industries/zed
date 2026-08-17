#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  X = 2,
  Y,
};
#ifndef __cplusplus
typedef uint32_t C;
#endif // __cplusplus

typedef struct A {
  int32_t _0;
} A;

typedef struct B {
  int32_t x;
  float y;
} B;

typedef struct D {
  uint8_t List;
  uintptr_t Of;
  struct B Things;
} D;

enum F_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Foo,
  Bar,
  Baz,
};
#ifndef __cplusplus
typedef uint8_t F_Tag;
#endif // __cplusplus

typedef struct Bar_Body {
  F_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef union F {
  F_Tag tag;
  struct {
    F_Tag foo_tag;
    int16_t foo;
  };
  Bar_Body bar;
} F;

enum H_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Hello,
  There,
  Everyone,
};
#ifndef __cplusplus
typedef uint8_t H_Tag;
#endif // __cplusplus

typedef struct There_Body {
  uint8_t x;
  int16_t y;
} There_Body;

typedef struct H {
  H_Tag tag;
  union {
    struct {
      int16_t hello;
    };
    There_Body there;
  };
} H;

enum I_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  ThereAgain,
  SomethingElse,
};
#ifndef __cplusplus
typedef uint8_t I_Tag;
#endif // __cplusplus

typedef struct ThereAgain_Body {
  uint8_t x;
  int16_t y;
} ThereAgain_Body;

typedef struct I {
  I_Tag tag;
  union {
    ThereAgain_Body there_again;
  };
} I;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A a, struct B b, C c, struct D d, union F f, struct H h, struct I i);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
