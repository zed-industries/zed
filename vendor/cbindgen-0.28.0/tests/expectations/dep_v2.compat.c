#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint32_t x;
  double y;
} dep_struct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

uint32_t get_x(const dep_struct *dep_struct);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
