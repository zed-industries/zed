#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  BarSome,
  BarThing,
} Bar;

typedef struct {
  uint8_t a;
} FooU8;

typedef FooU8 Boo;

void root(Boo x, Bar y);

void unsafe_root(Boo x, Bar y);
