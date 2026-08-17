#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Opaque;

template<typename T = void>
struct Option;

struct Foo {
  const Opaque *x;
  Opaque *y;
  void (*z)();
  void (**zz)();
};

union Bar {
  const Opaque *x;
  Opaque *y;
  void (*z)();
  void (**zz)();
};

extern "C" {

void root(const Opaque *a, Opaque *b, Foo c, Bar d, Option<Opaque*> *e, void (*f)(const Opaque*));

}  // extern "C"
