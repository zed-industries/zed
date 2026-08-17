#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<uint32_t V>
struct TakeUntil {
  const uint8_t *start;
  uintptr_t len;
  uintptr_t point;
};

extern "C" {

TakeUntil<0> until_nul(const uint8_t *start, uintptr_t len);

}  // extern "C"
