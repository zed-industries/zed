#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct NotReprC_i32 NotReprC_i32;

typedef NotReprC_i32 Foo;

typedef struct {
  int32_t number;
} MyStruct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Foo *a, const MyStruct *with_cell);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
