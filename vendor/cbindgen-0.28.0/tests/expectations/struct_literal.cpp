#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Bar;

struct Foo {
  int32_t a;
  uint32_t b;
};
constexpr static const Foo Foo_FOO = Foo{
  /* .a = */ 42,
  /* .b = */ 47
};
constexpr static const Foo Foo_FOO2 = Foo{
  /* .a = */ 42,
  /* .b = */ 47
};
constexpr static const Foo Foo_FOO3 = Foo{
  /* .a = */ 42,
  /* .b = */ 47
};


constexpr static const Foo BAR = Foo{
  /* .a = */ 42,
  /* .b = */ 1337
};



extern "C" {

void root(Foo x, Bar bar);

}  // extern "C"
