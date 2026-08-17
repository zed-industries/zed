#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct NotReprC_i32 NotReprC_i32;

typedef NotReprC_i32 Foo;

typedef struct {
  int32_t number;
} MyStruct;

void root(const Foo *a, const MyStruct *with_cell);
