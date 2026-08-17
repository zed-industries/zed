#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct ArrayVec_____u8__100 {
  uint8_t *xs[100];
  uint32_t len;
};

int32_t push(struct ArrayVec_____u8__100 *v, uint8_t *elem);
