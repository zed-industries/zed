#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct PREFIXFoo {
  int32_t a;
  uint32_t b;
};
constexpr static const PREFIXFoo PREFIXFoo_FOO = PREFIXFoo{
  /* .a = */ 42,
  /* .b = */ 47
};

constexpr static const PREFIXFoo PREFIXBAR = PREFIXFoo{
  /* .a = */ 42,
  /* .b = */ 1337
};

extern "C" {

void root(PREFIXFoo x);

}  // extern "C"
