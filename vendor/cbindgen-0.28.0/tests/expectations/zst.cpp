#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct TraitObject {
  void *data;
  void *vtable;
};

extern "C" {

void *root(const void *ptr, TraitObject t);

}  // extern "C"
