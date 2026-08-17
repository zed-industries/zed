#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct TraitObject {
  void *data;
  void *vtable;
} TraitObject;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void *root(const void *ptr, struct TraitObject t);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
