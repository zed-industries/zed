#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo_Bar {
  const int32_t *something;
};

struct Bar {
  int32_t something;
  struct Foo_Bar subexpressions;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Bar b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
