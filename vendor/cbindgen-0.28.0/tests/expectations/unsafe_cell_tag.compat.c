#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct NotReprC_i32;

typedef struct NotReprC_i32 Foo;

struct MyStruct {
  int32_t number;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Foo *a, const struct MyStruct *with_cell);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
