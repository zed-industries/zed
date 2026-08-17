// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_CULL_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_CULL_RECT_H_

#include <limits>
#include <optional>

#include "third_party/blink/renderer/platform/geometry/infinite_int_rect.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/rect.h"

namespace gfx {
class RectF;
}

namespace blink {

class AffineTransform;
class PropertyTreeState;
class ScrollPaintPropertyNode;
class TransformPaintPropertyNode;

class PLATFORM_EXPORT CullRect {
  DISALLOW_NEW();

 public:
  CullRect() = default;
  explicit CullRect(const gfx::Rect& rect) : rect_(rect) {}

  static CullRect Infinite() { return CullRect(InfiniteIntRect()); }

  bool IsInfinite() const { return rect_ == InfiniteIntRect(); }

  bool Intersects(const gfx::Rect&) const;
  bool IntersectsTransformed(const AffineTransform&, const gfx::RectF&) const;
  bool IntersectsHorizontalRange(LayoutUnit lo, LayoutUnit hi) const;
  bool IntersectsVerticalRange(LayoutUnit lo, LayoutUnit hi) const;

  void Move(const gfx::Vector2d& offset);

  // Applies one transform to the cull rect. Before this function is called,
  // the cull rect is in the space of the parent the transform node.
  void ApplyTransform(const TransformPaintPropertyNode&);

  // Similar to the above but also applies clips and expands for all directly
  // composited transforms (including scrolling and non-scrolling ones).
  // |root| is used to calculate the expansion distance in the local space,
  // to make the expansion distance approximately the same in the root space.
  // Returns whether the cull rect has been expanded.
  bool ApplyPaintProperties(const PropertyTreeState& root,
                            const PropertyTreeState& source,
                            const PropertyTreeState& destination,
                            const std::optional<CullRect>& old_cull_rect,
                            float expansion_ratio);

  const gfx::Rect& Rect() const { return rect_; }

  static bool CanExpandForScroll(const ScrollPaintPropertyNode&);

  bool HasScrolledEnough(const gfx::Vector2dF& delta,
                         const TransformPaintPropertyNode&,
                         float expansion_ratio);

  String ToString() const { return String(rect_.ToString()); }

 private:
  friend class CullRectTest;

  // Returns whether the cull rect is expanded along x and y axes.
  std::pair<bool, bool> ApplyScrollTranslation(
      const TransformPaintPropertyNode& root_transform,
      const TransformPaintPropertyNode& scroll_translation,
      float expansion_ratio);

  // Returns false if the rect is clipped to be invisible. Otherwise returns
  // true, even if the cull rect is empty due to a special 3d transform in case
  // later 3d transforms make the cull rect visible again.
  bool ApplyPaintPropertiesWithoutExpansion(
      const PropertyTreeState& source,
      const PropertyTreeState& destination);

  bool ChangedEnough(const std::pair<bool, bool>& expanded,
                     const CullRect& old_cull_rect,
                     const std::optional<gfx::Rect>& expansion_bounds,
                     const TransformPaintPropertyNode& local_transform,
                     float expansion_ratio) const;

  gfx::Rect rect_;
};

inline bool operator==(const CullRect& a, const CullRect& b) {
  return a.Rect() == b.Rect();
}
inline bool operator!=(const CullRect& a, const CullRect& b) {
  return !(a == b);
}

inline std::ostream& operator<<(std::ostream& os, const CullRect& cull_rect) {
  return os << cull_rect.ToString();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_CULL_RECT_H_
