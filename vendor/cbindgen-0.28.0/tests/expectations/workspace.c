#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define EXT_CONST 0

typedef struct {
  uint32_t data;
} ExtType;

void consume_ext(ExtType _ext);
