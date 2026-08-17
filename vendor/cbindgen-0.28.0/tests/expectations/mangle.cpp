#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class Bar {
  BarSome,
  BarThing,
};

template<typename T>
struct Foo {
  T a;
};

using Boo = Foo<uint8_t>;

extern "C" {

void root(Boo x, Bar y);

void unsafe_root(Boo x, Bar y);

}  // extern "C"
