#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Normal {
  int32_t x;
  float y;
};

extern "C" {

extern int32_t foo();

extern void bar(Normal a);

extern int32_t baz();

}  // extern "C"
