#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Foo {

};

extern "C" {

void extra_debug_fn();

void cbindgen();

void root(Foo a);

}  // extern "C"
