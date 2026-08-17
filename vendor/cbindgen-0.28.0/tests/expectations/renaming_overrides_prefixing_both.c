#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct StyleA StyleA;

typedef struct B {
  int32_t x;
  float y;
} B;

void root(const struct StyleA *a, struct B b);
