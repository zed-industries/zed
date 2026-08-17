#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct TraitObject {
  void *data;
  void *vtable;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void *root(const void *ptr, struct TraitObject t);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
