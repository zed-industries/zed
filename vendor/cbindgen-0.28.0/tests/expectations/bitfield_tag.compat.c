#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct HasBitfields {
  uint64_t foo: 8;
  uint64_t bar: 56;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const struct HasBitfields*);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
