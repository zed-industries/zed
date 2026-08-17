#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A A;

typedef struct B B;

typedef struct {
  A *members;
  uintptr_t count;
} List_A;

typedef struct {
  B *members;
  uintptr_t count;
} List_B;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void foo(List_A a);

void bar(List_B b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
