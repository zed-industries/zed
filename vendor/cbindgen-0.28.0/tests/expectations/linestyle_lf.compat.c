#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t x;
  float y;
} Dummy;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Dummy d);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
