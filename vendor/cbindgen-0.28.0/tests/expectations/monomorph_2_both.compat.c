#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A A;

typedef struct B B;

typedef struct List_A {
  struct A *members;
  uintptr_t count;
} List_A;

typedef struct List_B {
  struct B *members;
  uintptr_t count;
} List_B;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void foo(struct List_A a);

void bar(struct List_B b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
