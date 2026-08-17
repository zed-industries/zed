#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct Foo {
  const int32_t *something;
};

union Bar {
  int32_t something;
  Foo<Bar> subexpressions;
};

extern "C" {

void root(Bar b);

}  // extern "C"
