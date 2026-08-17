#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define EXT_CONST 0

struct ExtType {
  uint32_t data;
};

void consume_ext(struct ExtType _ext);
