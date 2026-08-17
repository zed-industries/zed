#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Constants shared by multiple CSS Box Alignment properties
///
/// These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
struct StyleAlignFlags {
  uint8_t bits;

  constexpr explicit operator bool() const {
    return !!bits;
  }
  constexpr StyleAlignFlags operator~() const {
    return StyleAlignFlags { static_cast<decltype(bits)>(~bits) };
  }
  constexpr StyleAlignFlags operator|(const StyleAlignFlags& other) const {
    return StyleAlignFlags { static_cast<decltype(bits)>(this->bits | other.bits) };
  }
  StyleAlignFlags& operator|=(const StyleAlignFlags& other) {
    *this = (*this | other);
    return *this;
  }
  constexpr StyleAlignFlags operator&(const StyleAlignFlags& other) const {
    return StyleAlignFlags { static_cast<decltype(bits)>(this->bits & other.bits) };
  }
  StyleAlignFlags& operator&=(const StyleAlignFlags& other) {
    *this = (*this & other);
    return *this;
  }
  constexpr StyleAlignFlags operator^(const StyleAlignFlags& other) const {
    return StyleAlignFlags { static_cast<decltype(bits)>(this->bits ^ other.bits) };
  }
  StyleAlignFlags& operator^=(const StyleAlignFlags& other) {
    *this = (*this ^ other);
    return *this;
  }
  static const StyleAlignFlags AUTO;
  static const StyleAlignFlags NORMAL;
  static const StyleAlignFlags START;
  static const StyleAlignFlags END;
  static const StyleAlignFlags ALIAS;
  static const StyleAlignFlags FLEX_START;
  static const StyleAlignFlags MIXED;
  static const StyleAlignFlags MIXED_SELF;
};
/// 'auto'
constexpr inline const StyleAlignFlags StyleAlignFlags::AUTO = StyleAlignFlags{
  /* .bits = */ (uint8_t)0
};
/// 'normal'
constexpr inline const StyleAlignFlags StyleAlignFlags::NORMAL = StyleAlignFlags{
  /* .bits = */ (uint8_t)1
};
/// 'start'
constexpr inline const StyleAlignFlags StyleAlignFlags::START = StyleAlignFlags{
  /* .bits = */ (uint8_t)(1 << 1)
};
/// 'end'
constexpr inline const StyleAlignFlags StyleAlignFlags::END = StyleAlignFlags{
  /* .bits = */ (uint8_t)(1 << 2)
};
constexpr inline const StyleAlignFlags StyleAlignFlags::ALIAS = StyleAlignFlags{
  /* .bits = */ (uint8_t)(StyleAlignFlags::END).bits
};
/// 'flex-start'
constexpr inline const StyleAlignFlags StyleAlignFlags::FLEX_START = StyleAlignFlags{
  /* .bits = */ (uint8_t)(1 << 3)
};
constexpr inline const StyleAlignFlags StyleAlignFlags::MIXED = StyleAlignFlags{
  /* .bits = */ (uint8_t)(((1 << 4) | (StyleAlignFlags::FLEX_START).bits) | (StyleAlignFlags::END).bits)
};
constexpr inline const StyleAlignFlags StyleAlignFlags::MIXED_SELF = StyleAlignFlags{
  /* .bits = */ (uint8_t)(((1 << 5) | (StyleAlignFlags::FLEX_START).bits) | (StyleAlignFlags::END).bits)
};

/// An arbitrary identifier for a native (OS compositor) surface
struct StyleNativeSurfaceId {
  uint64_t _0;
  static const StyleNativeSurfaceId DEBUG_OVERLAY;
};
/// A special id for the native surface that is used for debug / profiler overlays.
constexpr inline const StyleNativeSurfaceId StyleNativeSurfaceId::DEBUG_OVERLAY = StyleNativeSurfaceId{
  /* ._0 = */ UINT64_MAX
};

struct StyleNativeTileId {
  StyleNativeSurfaceId surface_id;
  int32_t x;
  int32_t y;
  static const StyleNativeTileId DEBUG_OVERLAY;
};
/// A special id for the native surface that is used for debug / profiler overlays.
constexpr inline const StyleNativeTileId StyleNativeTileId::DEBUG_OVERLAY = StyleNativeTileId{
  /* .surface_id = */ StyleNativeSurfaceId::DEBUG_OVERLAY,
  /* .x = */ 0,
  /* .y = */ 0
};

extern "C" {

void root(StyleAlignFlags flags, StyleNativeTileId tile);

}  // extern "C"
