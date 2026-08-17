#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum E {
  V,
};

struct S {
  uint8_t field;
};

typedef uint8_t A;

#define C1 (S){ .field = 0 }

#define C2 V

#define C3 0
