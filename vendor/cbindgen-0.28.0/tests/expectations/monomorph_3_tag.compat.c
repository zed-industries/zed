#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Bar_Bar_f32;

struct Bar_Foo_f32;

struct Bar_f32;

union Foo_i32 {
  const int32_t *data;
};

union Foo_f32 {
  const float *data;
};

union Foo_Bar_f32 {
  const struct Bar_f32 *data;
};

union Tuple_Foo_f32_____f32 {
  const union Foo_f32 *a;
  const float *b;
};

union Tuple_f32__f32 {
  const float *a;
  const float *b;
};

typedef union Tuple_f32__f32 Indirection_f32;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(union Foo_i32 a,
          union Foo_f32 b,
          struct Bar_f32 c,
          union Foo_Bar_f32 d,
          struct Bar_Foo_f32 e,
          struct Bar_Bar_f32 f,
          union Tuple_Foo_f32_____f32 g,
          Indirection_f32 h);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
