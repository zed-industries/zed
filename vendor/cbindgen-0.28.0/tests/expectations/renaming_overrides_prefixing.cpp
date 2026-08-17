#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct StyleA;

struct B {
  int32_t x;
  float y;
};

extern "C" {

void root(const StyleA *a, B b);

}  // extern "C"
