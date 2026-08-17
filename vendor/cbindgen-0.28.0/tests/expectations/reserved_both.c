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

enum C_Tag {
  D,
};
typedef uint8_t C_Tag;

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

enum E_Tag {
  Double,
  Float,
};
typedef uint8_t E_Tag;

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

enum F_Tag {
  double_,
  float_,
};
typedef uint8_t F_Tag;

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

void root(struct A a,
          struct B b,
          struct C c,
          struct E e,
          struct F f,
          int32_t namespace_,
          float float_);
