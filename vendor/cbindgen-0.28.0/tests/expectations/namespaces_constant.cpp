#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace constants {
namespace test {

constexpr static const int32_t FOO = 10;

constexpr static const float ZOM = 3.14;

struct Foo {
  int32_t x[FOO];
};

extern "C" {

void root(Foo x);

}  // extern "C"

}  // namespace test
}  // namespace constants
