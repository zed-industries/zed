#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque Opaque;

typedef struct Normal {
  int32_t x;
  float y;
} Normal;

typedef struct NormalWithZST {
  int32_t x;
  float y;
} NormalWithZST;

typedef struct TupleRenamed {
  int32_t m0;
  float m1;
} TupleRenamed;

typedef struct TupleNamed {
  int32_t x;
  float y;
} TupleNamed;

void root(struct Opaque *a,
          struct Normal b,
          struct NormalWithZST c,
          struct TupleRenamed d,
          struct TupleNamed e);
