#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const int32_t *something;
} Foo_Bar;

enum Bar_Tag {
  Min,
  Max,
  Other,
};
typedef uint8_t Bar_Tag;

typedef union {
  Bar_Tag tag;
  struct {
    Bar_Tag min_tag;
    Foo_Bar min;
  };
  struct {
    Bar_Tag max_tag;
    Foo_Bar max;
  };
} Bar;

void root(Bar b);
