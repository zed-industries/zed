#if 0
DEF DEFINED = 1
DEF NOT_DEFINED = 0
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

#if defined(NOT_DEFINED)
constexpr static const int32_t DEFAULT_X = 8;
#endif

#if defined(DEFINED)
constexpr static const int32_t DEFAULT_X = 42;
#endif

#if (defined(NOT_DEFINED) || defined(DEFINED))
struct Foo {
  int32_t x;
};
#endif

#if defined(NOT_DEFINED)
struct Bar {
  Foo y;
};
#endif

#if defined(DEFINED)
struct Bar {
  Foo z;
};
#endif

struct Root {
  Bar w;
};

extern "C" {

void root(Root a);

}  // extern "C"
