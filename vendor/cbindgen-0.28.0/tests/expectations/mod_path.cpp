#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint8_t EXPORT_ME_TOO = 42;

struct ExportMe {
  uint64_t val;
};

extern "C" {

void export_me(ExportMe *val);

}  // extern "C"
