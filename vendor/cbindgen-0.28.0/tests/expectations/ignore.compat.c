#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define NO_IGNORE_CONST 0

#define NoIgnoreStructWithImpl_NO_IGNORE_INNER_CONST 0

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void no_ignore_root(void);

void no_ignore_associated_method(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
