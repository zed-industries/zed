#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo_Bar {
  const int32_t *something;
};

enum Bar_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Min,
  Max,
  Other,
};
#ifndef __cplusplus
typedef uint8_t Bar_Tag;
#endif // __cplusplus

union Bar {
  Bar_Tag tag;
  struct {
    Bar_Tag min_tag;
    struct Foo_Bar min;
  };
  struct {
    Bar_Tag max_tag;
    struct Foo_Bar max;
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(union Bar b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
