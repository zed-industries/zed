#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T, uintptr_t CAP>
struct ArrayVec {
  T xs[CAP];
  uint32_t len;
};

extern "C" {

int32_t push(ArrayVec<uint8_t*, 100> *v, uint8_t *elem);

}  // extern "C"
