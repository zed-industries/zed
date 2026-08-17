#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct NotReprC_RefCell_i32;

typedef struct NotReprC_RefCell_i32 Foo;

struct MyStruct {
  int32_t number;
};

void root(const Foo *a, const struct MyStruct *with_cell);
