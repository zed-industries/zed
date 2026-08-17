#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const uint8_t *start;
  uintptr_t len;
  uintptr_t point;
} TakeUntil_0;

TakeUntil_0 until_nul(const uint8_t *start, uintptr_t len);
