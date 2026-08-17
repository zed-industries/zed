#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct TakeUntil_0 {
  const uint8_t *start;
  uintptr_t len;
  uintptr_t point;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct TakeUntil_0 until_nul(const uint8_t *start, uintptr_t len);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
