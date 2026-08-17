#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T, typename U>
struct Foo {
  T x;
  U y;
};

template<typename T>
using IntFoo = Foo<int32_t, T>;

extern "C" {

void root(IntFoo<int32_t> a);

}  // extern "C"
