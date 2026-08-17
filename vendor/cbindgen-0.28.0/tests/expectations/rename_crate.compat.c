#if 0
DEF DEFINE_FREEBSD = 0
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t x;
} Foo;

typedef struct {
  uint64_t y;
} RenamedTy;

#if !defined(DEFINE_FREEBSD)
typedef struct {
  uint8_t field;
} NoExternTy;
#endif

#if !defined(DEFINE_FREEBSD)
typedef struct {
  NoExternTy field;
} ContainsNoExternTy;
#endif

#if defined(DEFINE_FREEBSD)
typedef struct {
  uint64_t field;
} ContainsNoExternTy;
#endif

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Foo a);

void renamed_func(RenamedTy a);

void no_extern_func(ContainsNoExternTy a);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
