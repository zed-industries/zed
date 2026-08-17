#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint8_t *xs[100];
  uint32_t len;
} ArrayVec_____u8__100;

int32_t push(ArrayVec_____u8__100 *v, uint8_t *elem);
