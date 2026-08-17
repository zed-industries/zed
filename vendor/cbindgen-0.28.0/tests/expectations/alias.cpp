#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class Status : uint32_t {
  Ok,
  Err,
};

struct Dep {
  int32_t a;
  float b;
};

template<typename X>
struct Foo {
  X a;
  X b;
  Dep c;
};

using IntFoo = Foo<int32_t>;

using DoubleFoo = Foo<double>;

using Unit = int32_t;

using SpecialStatus = Status;

extern "C" {

void root(IntFoo x, DoubleFoo y, Unit z, SpecialStatus w);

}  // extern "C"
