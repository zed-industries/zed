#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Bar;

struct Foo {

};

extern "C" {

extern const int32_t NUMBER;

extern Foo FOO;

extern const Bar BAR;

void root();

}  // extern "C"
