#if 0
''' '
#endif

#ifdef __cplusplus
struct NonZeroI64;
#endif

#if 0
' '''
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct Option;

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
  const Option<int64_t> *j;
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
  const Option<int64_t> *j;
};

extern "C" {

void root_nonzero_aliases(NonZeroAliases test,
                          uint8_t a,
                          uint16_t b,
                          uint32_t c,
                          uint64_t d,
                          int8_t e,
                          int16_t f,
                          int32_t g,
                          int64_t h,
                          int64_t i,
                          const Option<int64_t> *j);

void root_nonzero_generics(NonZeroGenerics test,
                           uint8_t a,
                           uint16_t b,
                           uint32_t c,
                           uint64_t d,
                           int8_t e,
                           int16_t f,
                           int32_t g,
                           int64_t h,
                           int64_t i,
                           const Option<int64_t> *j);

}  // extern "C"
