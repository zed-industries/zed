#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int32_t (*VaListFnPtr)(int32_t count, ...);

typedef int32_t (*VaListFnPtr2)(int32_t count, ...);

struct Interface_______i32_______i32_______va_list {
  int32_t (*fn1)(int32_t count, ...);
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int32_t va_list_test(int32_t count, ...);

int32_t va_list_test2(int32_t count, ...);

void va_list_fn_ptrs(int32_t (*fn1)(int32_t count, ...),
                     int32_t (*fn2)(int32_t count, ...),
                     VaListFnPtr fn3,
                     VaListFnPtr2 fn4,
                     struct Interface_______i32_______i32_______va_list fn5,
                     struct Interface_______i32_______i32_______va_list fn6);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
