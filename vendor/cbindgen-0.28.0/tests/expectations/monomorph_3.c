#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Bar_Bar_f32 Bar_Bar_f32;

typedef struct Bar_Foo_f32 Bar_Foo_f32;

typedef struct Bar_f32 Bar_f32;

typedef union {
  const int32_t *data;
} Foo_i32;

typedef union {
  const float *data;
} Foo_f32;

typedef union {
  const Bar_f32 *data;
} Foo_Bar_f32;

typedef union {
  const Foo_f32 *a;
  const float *b;
} Tuple_Foo_f32_____f32;

typedef union {
  const float *a;
  const float *b;
} Tuple_f32__f32;

typedef Tuple_f32__f32 Indirection_f32;

void root(Foo_i32 a,
          Foo_f32 b,
          Bar_f32 c,
          Foo_Bar_f32 d,
          Bar_Foo_f32 e,
          Bar_Bar_f32 f,
          Tuple_Foo_f32_____f32 g,
          Indirection_f32 h);
