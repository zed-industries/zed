#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Foo_Tag {
  A,
};

struct Foo {
  enum Foo_Tag tag;
  union {
    struct {
      float a[20];
    };
  };
};

void root(struct Foo a);
