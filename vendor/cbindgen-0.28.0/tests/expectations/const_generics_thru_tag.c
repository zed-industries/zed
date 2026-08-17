#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Inner_1 {
  uint8_t bytes[1];
};

struct Outer_1 {
  struct Inner_1 inner;
};

struct Inner_2 {
  uint8_t bytes[2];
};

struct Outer_2 {
  struct Inner_2 inner;
};

struct Outer_1 one(void);

struct Outer_2 two(void);
