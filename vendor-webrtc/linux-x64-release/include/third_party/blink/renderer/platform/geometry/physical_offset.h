// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PHYSICAL_OFFSET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PHYSICAL_OFFSET_H_

#include "third_party/blink/renderer/platform/geometry/layout_point.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/point.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/vector2d.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// PhysicalOffset is the position of a rect (typically a fragment) relative to
// its parent rect in the physical coordinate system.
// For more information about physical and logical coordinate systems, see:
// https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/core/layout/README.md#coordinate-spaces
template <typename ValueType>
struct PLATFORM_EXPORT PhysicalFixedOffset {
  constexpr PhysicalFixedOffset() = default;
  constexpr PhysicalFixedOffset(ValueType left, ValueType top)
      : left(left), top(top) {}

  // This is deleted to avoid unwanted lossy conversion from float or double to
  // LayoutUnit or int. Use explicit LayoutUnit constructor for each parameter,
  // or use FromPointF*() instead.
  PhysicalFixedOffset(double, double) = delete;

  // For testing only. It's defined in core/testing/core_unit_test_helper.h.
  // 'constexpr' is to let compiler detect usage from production code.
  constexpr PhysicalFixedOffset(int left, int top);

  ValueType left;
  ValueType top;

  constexpr bool IsZero() const { return !left && !top; }
  constexpr bool HasFraction() const {
    return left.HasFraction() || top.HasFraction();
  }

  void ClampNegativeToZero() {
    left = std::max(left, ValueType());
    top = std::max(top, ValueType());
  }

  constexpr bool operator==(const PhysicalFixedOffset& other) const = default;

  PhysicalFixedOffset operator+(const PhysicalFixedOffset& other) const {
    return PhysicalFixedOffset{this->left + other.left, this->top + other.top};
  }
  PhysicalFixedOffset& operator+=(const PhysicalFixedOffset& other) {
    *this = *this + other;
    return *this;
  }

  PhysicalFixedOffset operator-() const {
    return PhysicalFixedOffset{-this->left, -this->top};
  }
  PhysicalFixedOffset operator-(const PhysicalFixedOffset& other) const {
    return PhysicalFixedOffset{this->left - other.left, this->top - other.top};
  }
  PhysicalFixedOffset& operator-=(const PhysicalFixedOffset& other) {
    *this = *this - other;
    return *this;
  }

  // Conversions from/to existing code. New code prefers type safety for
  // logical/physical distinctions.
  constexpr explicit PhysicalFixedOffset(const DeprecatedLayoutPoint& point)
      : left(point.X()), top(point.Y()) {}

  // Conversions from/to existing code. New code prefers type safety for
  // logical/physical distinctions.
  constexpr DeprecatedLayoutPoint FaultyToDeprecatedLayoutPoint() const {
    return {left, top};
  }

  explicit PhysicalFixedOffset(const gfx::Point& point)
      : left(point.x()), top(point.y()) {}
  explicit PhysicalFixedOffset(const gfx::Vector2d& vector)
      : left(vector.x()), top(vector.y()) {}

  static PhysicalFixedOffset FromPointFFloor(const gfx::PointF& point) {
    return {ValueType::FromFloatFloor(point.x()),
            ValueType::FromFloatFloor(point.y())};
  }
  static PhysicalFixedOffset FromPointFRound(const gfx::PointF& point) {
    return {ValueType::FromFloatRound(point.x()),
            ValueType::FromFloatRound(point.y())};
  }
  static PhysicalFixedOffset FromVector2dFFloor(const gfx::Vector2dF& vector) {
    return {ValueType::FromFloatFloor(vector.x()),
            ValueType::FromFloatFloor(vector.y())};
  }
  static PhysicalFixedOffset FromVector2dFRound(const gfx::Vector2dF& vector) {
    return {ValueType::FromFloatRound(vector.x()),
            ValueType::FromFloatRound(vector.y())};
  }

  void Scale(float s) {
    left *= s;
    top *= s;
  }

  constexpr explicit operator gfx::PointF() const { return {left, top}; }
  constexpr explicit operator gfx::Vector2dF() const { return {left, top}; }

  WTF::String ToString() const;
};

using PhysicalOffset = PhysicalFixedOffset<LayoutUnit>;

// TODO(crbug.com/41458361): These functions should upgraded to force correct
// pixel snapping in a type-safe way.
template <typename ValueType>
inline gfx::Point ToRoundedPoint(const PhysicalFixedOffset<ValueType>& o) {
  return {o.left.Round(), o.top.Round()};
}
template <typename ValueType>
inline gfx::Point ToFlooredPoint(const PhysicalFixedOffset<ValueType>& o) {
  return {o.left.Floor(), o.top.Floor()};
}
template <typename ValueType>
inline gfx::Point ToCeiledPoint(const PhysicalFixedOffset<ValueType>& o) {
  return {o.left.Ceil(), o.top.Ceil()};
}

template <typename ValueType>
inline gfx::Vector2d ToRoundedVector2d(
    const PhysicalFixedOffset<ValueType>& o) {
  return {o.left.Round(), o.top.Round()};
}
template <typename ValueType>
inline gfx::Vector2d ToFlooredVector2d(
    const PhysicalFixedOffset<ValueType>& o) {
  return {o.left.Floor(), o.top.Floor()};
}
template <typename ValueType>
inline gfx::Vector2d ToCeiledVector2d(const PhysicalFixedOffset<ValueType>& o) {
  return {o.left.Ceil(), o.top.Ceil()};
}

template <typename ValueType>
PLATFORM_EXPORT std::ostream& operator<<(std::ostream&,
                                         const PhysicalFixedOffset<ValueType>&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PHYSICAL_OFFSET_H_
