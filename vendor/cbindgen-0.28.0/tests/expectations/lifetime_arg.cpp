#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct A {
  const int32_t *data;
};

struct E {
  enum class Tag {
    V,
    U,
  };

  struct U_Body {
    const uint8_t *_0;
  };

  Tag tag;
  union {
    U_Body u;
  };
};

extern "C" {

void root(A _a, E _e);

}  // extern "C"
