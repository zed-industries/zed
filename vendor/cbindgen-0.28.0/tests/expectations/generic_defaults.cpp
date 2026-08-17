#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T, typename P = void>
using Foo = T;

template<typename T, typename P>
struct Bar {
  Foo<T> f;
  P p;
};

template<typename T>
using Baz = Foo<T>;

extern "C" {

void foo_root(Foo<int16_t> f, Bar<int32_t, uint32_t> b, Baz<int64_t> z);

}  // extern "C"
