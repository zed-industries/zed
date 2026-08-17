#if 0
DEF DEFINE_FREEBSD = 0
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Foo {
  int32_t x;
};

struct RenamedTy {
  uint64_t y;
};

#if !defined(DEFINE_FREEBSD)
struct NoExternTy {
  uint8_t field;
};
#endif

#if !defined(DEFINE_FREEBSD)
struct ContainsNoExternTy {
  NoExternTy field;
};
#endif

#if defined(DEFINE_FREEBSD)
struct ContainsNoExternTy {
  uint64_t field;
};
#endif

extern "C" {

void root(Foo a);

void renamed_func(RenamedTy a);

void no_extern_func(ContainsNoExternTy a);

}  // extern "C"
