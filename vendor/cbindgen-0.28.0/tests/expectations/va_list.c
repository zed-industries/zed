#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int32_t (*VaListFnPtr)(int32_t count, ...);

typedef int32_t (*VaListFnPtr2)(int32_t count, ...);

typedef struct {
  int32_t (*fn1)(int32_t count, ...);
} Interface_______i32_______i32_______va_list;

int32_t va_list_test(int32_t count, ...);

int32_t va_list_test2(int32_t count, ...);

void va_list_fn_ptrs(int32_t (*fn1)(int32_t count, ...),
                     int32_t (*fn2)(int32_t count, ...),
                     VaListFnPtr fn3,
                     VaListFnPtr2 fn4,
                     Interface_______i32_______i32_______va_list fn5,
                     Interface_______i32_______i32_______va_list fn6);
