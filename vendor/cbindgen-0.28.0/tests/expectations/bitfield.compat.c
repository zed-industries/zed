#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint64_t foo: 8;
  uint64_t bar: 56;
} HasBitfields;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const HasBitfields*);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
