#if 0
''' '
#endif

#ifdef __cplusplus
struct NonZeroI64;
#endif

#if 0
' '''
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Option_i64;

struct NonZeroAliases {
  uint8_t a;
  uint16_t b;
  uint32_t c;
  uint64_t d;
  int8_t e;
  int16_t f;
  int32_t g;
  int64_t h;
  int64_t i;
  const struct Option_i64 *j;
};

struct NonZeroGenerics {
  uint8_t a;
  uint16_t b;
  uint32_t c;
  uint64_t d;
  int8_t e;
  int16_t f;
  int32_t g;
  int64_t h;
  int64_t i;
  const struct Option_i64 *j;
};

void root_nonzero_aliases(struct NonZeroAliases test,
                          uint8_t a,
                          uint16_t b,
                          uint32_t c,
                          uint64_t d,
                          int8_t e,
                          int16_t f,
                          int32_t g,
                          int64_t h,
                          int64_t i,
                          const struct Option_i64 *j);

void root_nonzero_generics(struct NonZeroGenerics test,
                           uint8_t a,
                           uint16_t b,
                           uint32_t c,
                           uint64_t d,
                           int8_t e,
                           int16_t f,
                           int32_t g,
                           int64_t h,
                           int64_t i,
                           const struct Option_i64 *j);
