#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define EXT_CONST 0

typedef struct {
  uint32_t data;
} ExtType;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void consume_ext(ExtType _ext);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
