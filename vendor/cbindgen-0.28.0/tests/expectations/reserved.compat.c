#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t namespace_;
  float float_;
} A;

typedef struct {
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

typedef struct {
  int32_t namespace_;
  float float_;
} D_Body;

typedef struct {
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

typedef struct {
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

typedef struct {
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

void root(A a, B b, C c, E e, F f, int32_t namespace_, float float_);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
