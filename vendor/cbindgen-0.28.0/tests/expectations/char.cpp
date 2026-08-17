#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Foo {
  uint32_t a;
};

extern "C" {

void root(Foo a);

}  // extern "C"
