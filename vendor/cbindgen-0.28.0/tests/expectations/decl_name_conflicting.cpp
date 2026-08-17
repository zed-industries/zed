#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class BindingType : uint32_t {
  Buffer = 0,
  NotBuffer = 1,
};

struct BindGroupLayoutEntry {
  BindingType ty;
};

extern "C" {

void root(BindGroupLayoutEntry entry);

}  // extern "C"
