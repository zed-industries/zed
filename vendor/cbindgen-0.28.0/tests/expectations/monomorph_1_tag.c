#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Bar_Bar_f32;

struct Bar_Foo_f32;

struct Bar_f32;

struct Foo_i32 {
  const int32_t *data;
};

struct Foo_f32 {
  const float *data;
};

struct Foo_Bar_f32 {
  const struct Bar_f32 *data;
};

struct Tuple_Foo_f32_____f32 {
  const struct Foo_f32 *a;
  const float *b;
};

struct Tuple_f32__f32 {
  const float *a;
  const float *b;
};

typedef struct Tuple_f32__f32 Indirection_f32;

void root(struct Foo_i32 a,
          struct Foo_f32 b,
          struct Bar_f32 c,
          struct Foo_Bar_f32 d,
          struct Bar_Foo_f32 e,
          struct Bar_Bar_f32 f,
          struct Tuple_Foo_f32_____f32 g,
          Indirection_f32 h);
