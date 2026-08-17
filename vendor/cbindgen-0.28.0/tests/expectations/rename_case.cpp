#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void test_camel_case(int32_t fooBar);

void test_pascal_case(int32_t FooBar);

void test_snake_case(int32_t foo_bar);

void test_screaming_snake_case(int32_t FOO_BAR);

void test_gecko_case(int32_t aFooBar);

void test_prefix(int32_t prefix_foo_bar);

}  // extern "C"
