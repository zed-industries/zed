// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_PHYSICAL_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_PHYSICAL_RECT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect_f.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

struct PhysicalBoxStrut;

// PhysicalRect is the position and size of a rect (typically a fragment)
// relative to its parent rect in the physical coordinate system.
// For more information about physical and logical coordinate systems, see:
// https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/core/layout/README.md#coordinate-spaces
struct CORE_EXPORT PhysicalRect {
  constexpr PhysicalRect() = default;
  constexpr PhysicalRect(const PhysicalOffset& offset, const PhysicalSize& size)
      : offset(offset), size(size) {}
  constexpr PhysicalRect(LayoutUnit left,
                         LayoutUnit top,
                         LayoutUnit width,
                         LayoutUnit height)
      : offset(left, top), size(width, height) {}

  // This is deleted to avoid unwanted lossy conversion from float or double to
  // LayoutUnit or int. Use explicit LayoutUnit constructor for each parameter,
  // or use EnclosingRect() or FastAndLossyFromRectF() instead.
  PhysicalRect(double, double, double, double) = delete;

  // For testing only. It's defined in core/testing/core_unit_test_helper.h.
  // 'constexpr' is to let compiler detect usage from production code.
  constexpr PhysicalRect(int left, int top, int width, int height);

  PhysicalOffset offset;
  PhysicalSize size;

  constexpr bool IsEmpty() const { return size.IsEmpty(); }

  constexpr LayoutUnit X() const { return offset.left; }
  constexpr LayoutUnit Y() const { return offset.top; }
  constexpr LayoutUnit Width() const { return size.width; }
  constexpr LayoutUnit Height() const { return size.height; }
  LayoutUnit Right() const { return offset.left + size.width; }
  LayoutUnit Bottom() const { return offset.top + size.height; }

  void SetX(LayoutUnit x) { offset.left = x; }
  void SetY(LayoutUnit y) { offset.top = y; }
  void SetWidth(LayoutUnit w) { size.width = w; }
  void SetHeight(LayoutUnit h) { size.height = h; }

  PhysicalOffset MinXMinYCorner() const { return offset; }
  PhysicalOffset MaxXMinYCorner() const {
    return {offset.left + size.width, offset.top};
  }
  PhysicalOffset MinXMaxYCorner() const {
    return {offset.left, offset.top + size.height};
  }
  PhysicalOffset MaxXMaxYCorner() const {
    return {offset.left + size.width, offset.top + size.height};
  }

  constexpr bool operator==(const PhysicalRect& other) const = default;

  PhysicalRect operator+(const PhysicalOffset& other) const {
    return {offset + other, size};
  }

  PhysicalRect operator-(const PhysicalOffset& other) const {
    return {offset - other, size};
  }

  // Returns the distance to |target| in horizontal and vertical directions.
  // Each distance is zero if |this| contains |target| in that direction.
  PhysicalSize DistanceAsSize(PhysicalOffset target) const;

  // Returns square of the distance from |point| to the closest edge of |this|.
  // This function returns 0 if |this| contains |point|.
  LayoutUnit SquaredDistanceTo(const PhysicalOffset& point) const;

  bool Contains(const PhysicalRect&) const;
  bool Contains(LayoutUnit px, LayoutUnit py) const {
    return px >= offset.left && px < Right() && py >= offset.top &&
           py < Bottom();
  }
  bool Contains(const PhysicalOffset& point) const {
    return Contains(point.left, point.top);
  }

  [[nodiscard]] bool Intersects(const PhysicalRect&) const;
  [[nodiscard]] bool IntersectsInclusively(const PhysicalRect&) const;

  // Whether all edges of the rect are at full-pixel boundaries.
  // i.e.: ToEnclosingRect(this)) == this
  bool EdgesOnPixelBoundaries() const {
    return !offset.left.HasFraction() && !offset.top.HasFraction() &&
           !size.width.HasFraction() && !size.height.HasFraction();
  }

  void Unite(const PhysicalRect&);
  void UniteIfNonZero(const PhysicalRect&);
  void UniteEvenIfEmpty(const PhysicalRect&);

  void Intersect(const PhysicalRect&);
  bool InclusiveIntersect(const PhysicalRect&);

  void Expand(const PhysicalBoxStrut&);
  void ExpandEdges(LayoutUnit top,
                   LayoutUnit right,
                   LayoutUnit bottom,
                   LayoutUnit left) {
    offset.top -= top;
    offset.left -= left;
    size.width += left + right;
    size.height += top + bottom;
  }
  void ExpandEdgesToPixelBoundaries();
  void Inflate(LayoutUnit d) { ExpandEdges(d, d, d, d); }

  void Contract(const PhysicalBoxStrut&);
  void ContractEdges(LayoutUnit top,
                     LayoutUnit right,
                     LayoutUnit bottom,
                     LayoutUnit left) {
    ExpandEdges(-top, -right, -bottom, -left);
  }

  void Move(const PhysicalOffset& o) { offset += o; }

  void ShiftLeftEdgeTo(LayoutUnit edge) {
    LayoutUnit delta = edge - X();
    SetX(edge);
    SetWidth((Width() - delta).ClampNegativeToZero());
  }
  void ShiftRightEdgeTo(LayoutUnit edge) {
    LayoutUnit delta = edge - Right();
    SetWidth((Width() + delta).ClampNegativeToZero());
  }
  void ShiftTopEdgeTo(LayoutUnit edge) {
    LayoutUnit delta = edge - Y();
    SetY(edge);
    SetHeight((Height() - delta).ClampNegativeToZero());
  }
  void ShiftBottomEdgeTo(LayoutUnit edge) {
    LayoutUnit delta = edge - Bottom();
    SetHeight((Height() + delta).ClampNegativeToZero());
  }

  // TODO(crbug.com/962299): These functions should upgraded to force correct
  // pixel snapping in a type-safe way.
  gfx::Point PixelSnappedOffset() const { return ToRoundedPoint(offset); }
  int PixelSnappedWidth() const {
    return SnapSizeToPixel(size.width, offset.left);
  }
  int PixelSnappedHeight() const {
    return SnapSizeToPixel(size.height, offset.top);
  }
  gfx::Size PixelSnappedSize() const {
    return {PixelSnappedWidth(), PixelSnappedHeight()};
  }

  PhysicalOffset Center() const {
    return offset + PhysicalOffset(size.width / 2, size.height / 2);
  }

  constexpr explicit operator gfx::RectF() const {
    return gfx::RectF(offset.left, offset.top, size.width, size.height);
  }

  static PhysicalRect EnclosingRect(const gfx::RectF& rect) {
    PhysicalOffset offset(LayoutUnit::FromFloatFloor(rect.x()),
                          LayoutUnit::FromFloatFloor(rect.y()));
    PhysicalSize size(LayoutUnit::FromFloatCeil(rect.right()) - offset.left,
                      LayoutUnit::FromFloatCeil(rect.bottom()) - offset.top);
    return PhysicalRect(offset, size);
  }

  // This is faster than EnclosingRect(). Can be used in situation that we
  // prefer performance to accuracy and haven't observed problems caused by the
  // tiny error (< LayoutUnit::Epsilon()).
  static PhysicalRect FastAndLossyFromRectF(const gfx::RectF& rect) {
    return PhysicalRect(LayoutUnit(rect.x()), LayoutUnit(rect.y()),
                        LayoutUnit(rect.width()), LayoutUnit(rect.height()));
  }

  explicit PhysicalRect(const gfx::Rect& r)
      : offset(r.origin()), size(r.size()) {}

  void Scale(float s) {
    offset.Scale(s);
    size.Scale(s);
  }

  WTF::String ToString() const;
};

inline PhysicalRect UnionRect(const PhysicalRect& a, const PhysicalRect& b) {
  auto r = a;
  r.Unite(b);
  return r;
}

inline PhysicalRect Intersection(const PhysicalRect& a, const PhysicalRect& b) {
  auto r = a;
  r.Intersect(b);
  return r;
}

// TODO(crbug.com/962299): These functions should upgraded to force correct
// pixel snapping in a type-safe way.
inline gfx::Rect ToEnclosingRect(const PhysicalRect& r) {
  gfx::Point location = ToFlooredPoint(r.offset);
  gfx::Point max_point = ToCeiledPoint(r.MaxXMaxYCorner());
  // Because the range of LayoutUnit is much smaller than int, the following
  // '-' operations can never overflow, so no clamping is needed.
  // TODO(1261553): We can have a special version of gfx::Rect constructor that
  // skips internal clamping to improve performance.
  return gfx::Rect(location.x(), location.y(), max_point.x() - location.x(),
                   max_point.y() - location.y());
}
inline gfx::Rect ToPixelSnappedRect(const PhysicalRect& r) {
  return {r.PixelSnappedOffset(), r.PixelSnappedSize()};
}

CORE_EXPORT PhysicalRect UnionRect(const Vector<PhysicalRect>& rects);
CORE_EXPORT PhysicalRect
UnionRectEvenIfEmpty(const Vector<PhysicalRect>& rects);

CORE_EXPORT std::ostream& operator<<(std::ostream&, const PhysicalRect&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_PHYSICAL_RECT_H_
