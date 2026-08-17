// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BOX_REFLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BOX_REFLECTION_H_

#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "ui/gfx/geometry/rect_f.h"

class SkMatrix;

namespace blink {

// A reflection, as created by -webkit-box-reflect. Consists of:
// * a direction (either vertical or horizontal)
// * an offset to be applied to the reflection after flipping about the
//   x- or y-axis, according to the direction
// * a mask image, which will be applied to the reflection before the
//   reflection matrix is applied
class PLATFORM_EXPORT BoxReflection {
  DISALLOW_NEW();

 public:
  enum ReflectionDirection {
    // Vertically flipped (to appear above or below).
    kVerticalReflection,
    // Horizontally flipped (to appear to the left or right).
    kHorizontalReflection,
  };

  BoxReflection(ReflectionDirection direction, float offset);
  BoxReflection(ReflectionDirection direction,
                float offset,
                PaintRecord mask,
                const gfx::RectF& mask_bounds);
  BoxReflection(const BoxReflection& reflection);
  ~BoxReflection();

  ReflectionDirection Direction() const { return direction_; }
  float Offset() const { return offset_; }
  const PaintRecord& Mask() const { return mask_; }
  const gfx::RectF& MaskBounds() const { return mask_bounds_; }

  // Returns a matrix which maps points between the original content and its
  // reflection. Reflections are self-inverse, so this matrix can be used to
  // map in either direction.
  SkMatrix ReflectionMatrix() const;

  // Maps a source rectangle to the destination rectangle it can affect,
  // including this reflection. Due to the symmetry of reflections, this can
  // also be used to map from a destination rectangle to the source rectangle
  // which contributes to it.
  gfx::RectF MapRect(const gfx::RectF&) const;

 private:
  ReflectionDirection direction_;
  float offset_;
  PaintRecord mask_;
  gfx::RectF mask_bounds_;
};

inline bool operator==(const BoxReflection& a, const BoxReflection& b) {
  return a.Direction() == b.Direction() && a.Offset() == b.Offset() &&
         // False negatives are allowed, but not false positives.
         a.Mask().empty() && b.Mask().empty() &&
         a.MaskBounds() == b.MaskBounds();
}

inline bool operator!=(const BoxReflection& a, const BoxReflection& b) {
  return !(a == b);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BOX_REFLECTION_H_
