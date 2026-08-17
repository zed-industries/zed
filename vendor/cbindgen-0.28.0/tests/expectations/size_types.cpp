#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class IE : ptrdiff_t {
  IV,
};

enum class UE : size_t {
  UV,
};

using Usize = size_t;

using Isize = ptrdiff_t;

extern "C" {

void root(Usize, Isize, UE, IE);

}  // extern "C"
