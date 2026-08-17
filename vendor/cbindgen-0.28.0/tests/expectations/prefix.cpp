#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const int32_t PREFIX_LEN = 22;

constexpr static const int64_t PREFIX_X = (22 << 22);

constexpr static const int64_t PREFIX_Y = (PREFIX_X + PREFIX_X);

using PREFIX_NamedLenArray = int32_t[PREFIX_LEN];

using PREFIX_ValuedLenArray = int32_t[22];

union PREFIX_AbsoluteFontWeight {
  enum class Tag : uint8_t {
    Weight,
    Normal,
    Bold,
  };

  struct Weight_Body {
    Tag tag;
    float _0;
  };

  struct {
    Tag tag;
  };
  Weight_Body weight;
};

extern "C" {

void root(PREFIX_NamedLenArray x, PREFIX_ValuedLenArray y, PREFIX_AbsoluteFontWeight z);

}  // extern "C"
