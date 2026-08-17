#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Constants shared by multiple CSS Box Alignment properties
///
/// These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
struct AlignFlags {
  uint8_t bits;

  constexpr explicit operator bool() const {
    return !!bits;
  }
  constexpr AlignFlags operator~() const {
    return AlignFlags { static_cast<decltype(bits)>(~bits) };
  }
  constexpr AlignFlags operator|(const AlignFlags& other) const {
    return AlignFlags { static_cast<decltype(bits)>(this->bits | other.bits) };
  }
  AlignFlags& operator|=(const AlignFlags& other) {
    *this = (*this | other);
    return *this;
  }
  constexpr AlignFlags operator&(const AlignFlags& other) const {
    return AlignFlags { static_cast<decltype(bits)>(this->bits & other.bits) };
  }
  AlignFlags& operator&=(const AlignFlags& other) {
    *this = (*this & other);
    return *this;
  }
  constexpr AlignFlags operator^(const AlignFlags& other) const {
    return AlignFlags { static_cast<decltype(bits)>(this->bits ^ other.bits) };
  }
  AlignFlags& operator^=(const AlignFlags& other) {
    *this = (*this ^ other);
    return *this;
  }
};
/// 'auto'
constexpr static const AlignFlags AlignFlags_AUTO = AlignFlags{
  /* .bits = */ (uint8_t)0
};
/// 'normal'
constexpr static const AlignFlags AlignFlags_NORMAL = AlignFlags{
  /* .bits = */ (uint8_t)1
};
/// 'start'
constexpr static const AlignFlags AlignFlags_START = AlignFlags{
  /* .bits = */ (uint8_t)(1 << 1)
};
/// 'end'
constexpr static const AlignFlags AlignFlags_END = AlignFlags{
  /* .bits = */ (uint8_t)(1 << 2)
};
constexpr static const AlignFlags AlignFlags_ALIAS = AlignFlags{
  /* .bits = */ (uint8_t)(AlignFlags_END).bits
};
/// 'flex-start'
constexpr static const AlignFlags AlignFlags_FLEX_START = AlignFlags{
  /* .bits = */ (uint8_t)(1 << 3)
};
constexpr static const AlignFlags AlignFlags_MIXED = AlignFlags{
  /* .bits = */ (uint8_t)(((1 << 4) | (AlignFlags_FLEX_START).bits) | (AlignFlags_END).bits)
};
constexpr static const AlignFlags AlignFlags_MIXED_SELF = AlignFlags{
  /* .bits = */ (uint8_t)(((1 << 5) | (AlignFlags_FLEX_START).bits) | (AlignFlags_END).bits)
};

struct DebugFlags {
  uint32_t bits;

  constexpr explicit operator bool() const {
    return !!bits;
  }
  constexpr DebugFlags operator~() const {
    return DebugFlags { static_cast<decltype(bits)>(~bits) };
  }
  constexpr DebugFlags operator|(const DebugFlags& other) const {
    return DebugFlags { static_cast<decltype(bits)>(this->bits | other.bits) };
  }
  DebugFlags& operator|=(const DebugFlags& other) {
    *this = (*this | other);
    return *this;
  }
  constexpr DebugFlags operator&(const DebugFlags& other) const {
    return DebugFlags { static_cast<decltype(bits)>(this->bits & other.bits) };
  }
  DebugFlags& operator&=(const DebugFlags& other) {
    *this = (*this & other);
    return *this;
  }
  constexpr DebugFlags operator^(const DebugFlags& other) const {
    return DebugFlags { static_cast<decltype(bits)>(this->bits ^ other.bits) };
  }
  DebugFlags& operator^=(const DebugFlags& other) {
    *this = (*this ^ other);
    return *this;
  }
};
/// Flag with the topmost bit set of the u32
constexpr static const DebugFlags DebugFlags_BIGGEST_ALLOWED = DebugFlags{
  /* .bits = */ (uint32_t)(1 << 31)
};

struct LargeFlags {
  uint64_t bits;

  constexpr explicit operator bool() const {
    return !!bits;
  }
  constexpr LargeFlags operator~() const {
    return LargeFlags { static_cast<decltype(bits)>(~bits) };
  }
  constexpr LargeFlags operator|(const LargeFlags& other) const {
    return LargeFlags { static_cast<decltype(bits)>(this->bits | other.bits) };
  }
  LargeFlags& operator|=(const LargeFlags& other) {
    *this = (*this | other);
    return *this;
  }
  constexpr LargeFlags operator&(const LargeFlags& other) const {
    return LargeFlags { static_cast<decltype(bits)>(this->bits & other.bits) };
  }
  LargeFlags& operator&=(const LargeFlags& other) {
    *this = (*this & other);
    return *this;
  }
  constexpr LargeFlags operator^(const LargeFlags& other) const {
    return LargeFlags { static_cast<decltype(bits)>(this->bits ^ other.bits) };
  }
  LargeFlags& operator^=(const LargeFlags& other) {
    *this = (*this ^ other);
    return *this;
  }
};
/// Flag with a very large shift that usually would be narrowed.
constexpr static const LargeFlags LargeFlags_LARGE_SHIFT = LargeFlags{
  /* .bits = */ (uint64_t)(1ull << 44)
};
constexpr static const LargeFlags LargeFlags_INVERTED = LargeFlags{
  /* .bits = */ (uint64_t)~(LargeFlags_LARGE_SHIFT).bits
};

struct OutOfLine {
  uint32_t _0;

  constexpr explicit operator bool() const {
    return !!_0;
  }
  constexpr OutOfLine operator~() const {
    return OutOfLine { static_cast<decltype(_0)>(~_0) };
  }
  constexpr OutOfLine operator|(const OutOfLine& other) const {
    return OutOfLine { static_cast<decltype(_0)>(this->_0 | other._0) };
  }
  OutOfLine& operator|=(const OutOfLine& other) {
    *this = (*this | other);
    return *this;
  }
  constexpr OutOfLine operator&(const OutOfLine& other) const {
    return OutOfLine { static_cast<decltype(_0)>(this->_0 & other._0) };
  }
  OutOfLine& operator&=(const OutOfLine& other) {
    *this = (*this & other);
    return *this;
  }
  constexpr OutOfLine operator^(const OutOfLine& other) const {
    return OutOfLine { static_cast<decltype(_0)>(this->_0 ^ other._0) };
  }
  OutOfLine& operator^=(const OutOfLine& other) {
    *this = (*this ^ other);
    return *this;
  }
};
constexpr static const OutOfLine OutOfLine_A = OutOfLine{
  /* ._0 = */ (uint32_t)1
};
constexpr static const OutOfLine OutOfLine_B = OutOfLine{
  /* ._0 = */ (uint32_t)2
};
constexpr static const OutOfLine OutOfLine_AB = OutOfLine{
  /* ._0 = */ (uint32_t)((OutOfLine_A)._0 | (OutOfLine_B)._0)
};

extern "C" {

void root(AlignFlags flags,
          DebugFlags bigger_flags,
          LargeFlags largest_flags,
          OutOfLine out_of_line);

}  // extern "C"
