#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint16_t FONT_WEIGHT_FRACTION_BITS = 6;

template<uint16_t FRACTION_BITS>
struct FixedPoint {
  uint16_t value;
};

using FontWeightFixedPoint = FixedPoint<FONT_WEIGHT_FRACTION_BITS>;

struct FontWeight {
  FontWeightFixedPoint _0;
};
constexpr static const FontWeight FontWeight_NORMAL = FontWeight{
  /* ._0 = */ FontWeightFixedPoint{
    /* .value = */ (400 << FONT_WEIGHT_FRACTION_BITS)
  }
};

extern "C" {

void root(FontWeight w);

}  // extern "C"
