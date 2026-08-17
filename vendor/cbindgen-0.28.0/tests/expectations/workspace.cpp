#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const int32_t EXT_CONST = 0;

struct ExtType {
  uint32_t data;
};

extern "C" {

void consume_ext(ExtType _ext);

}  // extern "C"
