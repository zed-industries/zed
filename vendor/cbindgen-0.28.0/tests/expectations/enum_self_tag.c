#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo_Bar {
  const int32_t *something;
};

enum Bar_Tag {
  Min,
  Max,
  Other,
};
typedef uint8_t Bar_Tag;

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

void root(union Bar b);
