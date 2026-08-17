#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint32_t NO_IGNORE_CONST = 0;

constexpr static const uint32_t NoIgnoreStructWithImpl_NO_IGNORE_INNER_CONST = 0;

extern "C" {

void no_ignore_root();

void no_ignore_associated_method();

}  // extern "C"
