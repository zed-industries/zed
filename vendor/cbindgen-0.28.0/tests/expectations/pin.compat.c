#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using Pin = T;
template <typename T>
using Box = T*;
#endif

#if 0
' '''
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t *pinned_box;
  int32_t *pinned_ref;
} PinTest;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(int32_t *s, PinTest p);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
