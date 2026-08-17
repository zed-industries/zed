#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Inner_1 {
  uint8_t bytes[1];
} Inner_1;

typedef struct Outer_1 {
  struct Inner_1 inner;
} Outer_1;

typedef struct Inner_2 {
  uint8_t bytes[2];
} Inner_2;

typedef struct Outer_2 {
  struct Inner_2 inner;
} Outer_2;

struct Outer_1 one(void);

struct Outer_2 two(void);
