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

struct PinTest {
  int32_t *pinned_box;
  int32_t *pinned_ref;
};

void root(int32_t *s, struct PinTest p);
