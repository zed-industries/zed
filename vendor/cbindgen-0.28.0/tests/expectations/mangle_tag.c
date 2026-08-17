#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Bar {
  BarSome,
  BarThing,
};

struct FooU8 {
  uint8_t a;
};

typedef struct FooU8 Boo;

void root(Boo x, enum Bar y);

void unsafe_root(Boo x, enum Bar y);
