#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*MyCallback)(uintptr_t a, uintptr_t b);

typedef void (*MyOtherCallback)(uintptr_t a,
                                uintptr_t lot,
                                uintptr_t of,
                                uintptr_t args,
                                uintptr_t and_then_some);

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void my_function(MyCallback a, MyOtherCallback b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
