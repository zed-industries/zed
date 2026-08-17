// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PHYSICAL_SIZE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PHYSICAL_SIZE_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/size.h"
#include "ui/gfx/geometry/size_f.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

enum AspectRatioFit { kAspectRatioFitShrink, kAspectRatioFitGrow };

// PhysicalSize is the size of a rect (typically a fragment) in the physical
// coordinate system.
// For more information about physical and logical coordinate systems, see:
// https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/core/layout/README.md#coordinate-spaces
struct PLATFORM_EXPORT PhysicalSize {
  constexpr PhysicalSize() = default;
  constexpr PhysicalSize(LayoutUnit width, LayoutUnit height)
      : width(width), height(height) {}

  // This is deleted to avoid unwanted lossy conversion from float or double to
  // LayoutUnit or int. Use explicit LayoutUnit constructor for each parameter,
  // or use FromSizeF*() instead.
  PhysicalSize(double, double) = delete;

  // For testing only. It's defined in core/testing/core_unit_test_helper.h.
  // 'constexpr' is to let compiler detect usage from production code.
  constexpr PhysicalSize(int width, int height);

  LayoutUnit width;
  LayoutUnit height;

  constexpr bool operator==(const PhysicalSize& other) const = default;

  PhysicalSize operator+(const PhysicalSize& other) const {
    return PhysicalSize{this->width + other.width, this->height + other.height};
  }
  PhysicalSize& operator+=(const PhysicalSize& other) {
    *this = *this + other;
    return *this;
  }

  PhysicalSize operator-() const {
    return PhysicalSize{-this->width, -this->height};
  }
  PhysicalSize operator-(const PhysicalSize& other) const {
    return PhysicalSize{this->width - other.width, this->height - other.height};
  }
  PhysicalSize& operator-=(const PhysicalSize& other) {
    *this = *this - other;
    return *this;
  }

  // Returns a new PhysicalSize scaling `this` by `scale`.
  PhysicalSize operator*(float scale) const {
    return PhysicalSize(LayoutUnit(this->width * scale),
                        LayoutUnit(this->height * scale));
  }

  constexpr bool IsEmpty() const {
    return width == LayoutUnit() || height == LayoutUnit();
  }
  constexpr bool IsZero() const {
    return width == LayoutUnit() && height == LayoutUnit();
  }
  constexpr bool HasFraction() const {
    return width.HasFraction() || height.HasFraction();
  }

  void Scale(float s) {
    width *= s;
    height *= s;
  }
  void Scale(LayoutUnit s) {
    width *= s;
    height *= s;
  }

  // Returns a new PhysicalSize with the maximum width of `this` and `other`,
  // and the maximum height of `this` and `other`.
  PhysicalSize ExpandedTo(const PhysicalSize& other) const {
    return {std::max(this->width, other.width),
            std::max(this->height, other.height)};
  }

  // Returns a new PhysicalSize with the minimum width of `this` and `other`,
  // and the minimum height of `this` and `other`.
  PhysicalSize ShrunkTo(const PhysicalSize& other) const {
    return {std::min(this->width, other.width),
            std::min(this->height, other.height)};
  }

  void ClampNegativeToZero() {
    width = std::max(width, LayoutUnit());
    height = std::max(height, LayoutUnit());
  }

  PhysicalSize FitToAspectRatio(const PhysicalSize& aspect_ratio,
                                AspectRatioFit fit) const;

  constexpr explicit operator gfx::SizeF() const { return {width, height}; }

  static PhysicalSize FromSizeFRound(const gfx::SizeF& size) {
    return {LayoutUnit::FromFloatRound(size.width()),
            LayoutUnit::FromFloatRound(size.height())};
  }
  static PhysicalSize FromSizeFFloor(const gfx::SizeF& size) {
    return {LayoutUnit::FromFloatFloor(size.width()),
            LayoutUnit::FromFloatFloor(size.height())};
  }

  explicit PhysicalSize(const gfx::Size& size)
      : width(size.width()), height(size.height()) {}

  WTF::String ToString() const;
};

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const PhysicalSize&);

// Convert an aspect ratio represented as a gfx::SizeF (width:height) to the
// closest PhysicalSize representation.
PLATFORM_EXPORT PhysicalSize LayoutRatioFromSizeF(gfx::SizeF ratio);

// TODO(crbug.com/41458361): These functions should upgraded to force correct
// pixel snapping in a type-safe way.
inline gfx::Size ToRoundedSize(const PhysicalSize& s) {
  return {s.width.Round(), s.height.Round()};
}
inline gfx::Size ToFlooredSize(const PhysicalSize& s) {
  return {s.width.Floor(), s.height.Floor()};
}
inline gfx::Size ToCeiledSize(const PhysicalSize& s) {
  return {s.width.Ceil(), s.height.Ceil()};
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PHYSICAL_SIZE_H_
