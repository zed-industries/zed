/* Package version: 0.1.0 */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Foo {
  uint64_t bar;
};

extern "C" {

void doit(const Foo*);

}  // extern "C"
