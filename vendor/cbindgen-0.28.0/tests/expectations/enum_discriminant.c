#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define FOURTY_FOUR 4

enum E {
  A = 1,
  B = -1,
  C = (1 + 2),
  D = FOURTY_FOUR,
  F = 5,
  G = (int8_t)54,
  H = (int8_t)false,
};
typedef int8_t E;

void root(const E*);
