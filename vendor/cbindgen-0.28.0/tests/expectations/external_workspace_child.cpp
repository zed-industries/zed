#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct ExtType {
  uint32_t data;
};

extern "C" {

void consume_ext(ExtType _ext);

}  // extern "C"
