#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A {
  int32_t namespace_;
  float float_;
} A;

typedef struct B {
  int32_t namespace_;
  float float_;
} B;

enum C_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  D,
};
#ifndef __cplusplus
typedef uint8_t C_Tag;
#endif // __cplusplus

typedef struct D_Body {
  int32_t namespace_;
  float float_;
} D_Body;

typedef struct C {
  C_Tag tag;
  union {
    D_Body d;
  };
} C;

enum E_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Double,
  Float,
};
#ifndef __cplusplus
typedef uint8_t E_Tag;
#endif // __cplusplus

typedef struct E {
  E_Tag tag;
  union {
    struct {
      double double_;
    };
    struct {
      float float_;
    };
  };
} E;

enum F_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  double_,
  float_,
};
#ifndef __cplusplus
typedef uint8_t F_Tag;
#endif // __cplusplus

typedef struct F {
  F_Tag tag;
  union {
    struct {
      double double_;
    };
    struct {
      float float_;
    };
  };
} F;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A a,
          struct B b,
          struct C c,
          struct E e,
          struct F f,
          int32_t namespace_,
          float float_);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
