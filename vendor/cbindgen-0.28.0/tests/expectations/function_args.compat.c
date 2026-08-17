#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void unnamed(const uint64_t*);

void pointer_test(const uint64_t *a);

void print_from_rust(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
