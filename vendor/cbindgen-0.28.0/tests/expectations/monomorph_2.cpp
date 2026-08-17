#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct A;

struct B;

template<typename T>
struct List {
  T *members;
  uintptr_t count;
};

extern "C" {

void foo(List<A> a);

void bar(List<B> b);

}  // extern "C"
