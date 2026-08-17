#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const uint32_t FIRST;

extern const uint32_t RENAMED;

void first(void);

void renamed(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
