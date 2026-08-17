#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Opaque;

union Normal {
  int32_t x;
  float y;
};

union NormalWithZST {
  int32_t x;
  float y;
};

extern "C" {

void root(Opaque *a, Normal b, NormalWithZST c);

}  // extern "C"
