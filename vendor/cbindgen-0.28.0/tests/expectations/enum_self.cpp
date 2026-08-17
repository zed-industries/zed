#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct Foo {
  const int32_t *something;
};

union Bar {
  enum class Tag : uint8_t {
    Min,
    Max,
    Other,
  };

  struct Min_Body {
    Tag tag;
    Foo<Bar> _0;
  };

  struct Max_Body {
    Tag tag;
    Foo<Bar> _0;
  };

  struct {
    Tag tag;
  };
  Min_Body min;
  Max_Body max;
};

extern "C" {

void root(Bar b);

}  // extern "C"
