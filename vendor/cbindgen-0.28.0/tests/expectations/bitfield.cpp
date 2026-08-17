#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct HasBitfields {
  uint64_t foo: 8;
  uint64_t bar: 56;
};

extern "C" {

void root(const HasBitfields*);

}  // extern "C"
