#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct TakeUntil_0 {
  const uint8_t *start;
  uintptr_t len;
  uintptr_t point;
} TakeUntil_0;

struct TakeUntil_0 until_nul(const uint8_t *start, uintptr_t len);
