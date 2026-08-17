#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<uintptr_t N>
struct Inner {
  uint8_t bytes[N];
};

template<uintptr_t N>
struct Outer {
  Inner<N> inner;
};

extern "C" {

Outer<1> one();

Outer<2> two();

}  // extern "C"
