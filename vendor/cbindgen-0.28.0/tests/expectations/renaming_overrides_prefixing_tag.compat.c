#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct StyleA;

struct B {
  int32_t x;
  float y;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const struct StyleA *a, struct B b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
