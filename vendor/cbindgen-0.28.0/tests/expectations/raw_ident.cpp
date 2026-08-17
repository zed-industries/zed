#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class Enum : uint8_t {
  a,
  b,
};

struct Struct {
  Enum field;
};

extern "C" {

extern const Enum STATIC;

void fn(Struct arg);

}  // extern "C"
