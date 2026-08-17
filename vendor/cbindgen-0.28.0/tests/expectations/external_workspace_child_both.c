#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct ExtType {
  uint32_t data;
} ExtType;

void consume_ext(struct ExtType _ext);
