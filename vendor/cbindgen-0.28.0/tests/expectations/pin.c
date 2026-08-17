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

void root(int32_t *s, PinTest p);
