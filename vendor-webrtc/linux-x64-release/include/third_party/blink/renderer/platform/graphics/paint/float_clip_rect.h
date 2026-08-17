// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_FLOAT_CLIP_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_FLOAT_CLIP_RECT_H_

#include "third_party/blink/renderer/platform/geometry/float_rounded_rect.h"
#include "third_party/blink/renderer/platform/geometry/infinite_int_rect.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/rect_f.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class PLATFORM_EXPORT FloatClipRect {
  USING_FAST_MALLOC(FloatClipRect);

 public:
  FloatClipRect()
      : rect_(InfiniteIntRect()),
        has_radius_(false),
        is_tight_(true),
        is_infinite_(true) {}

  explicit FloatClipRect(const gfx::RectF& rect) { SetRect(rect); }

  explicit FloatClipRect(const FloatRoundedRect& rect)
      : rect_(rect.Rect()),
        has_radius_(rect.IsRounded()),
        is_tight_(!rect.IsRounded()),
        is_infinite_(false) {}

  const gfx::RectF& Rect() const { return rect_; }
  gfx::RectF& Rect() { return rect_; }

  void SetRect(const gfx::RectF& rect) {
    rect_ = rect;
    has_radius_ = false;
    is_tight_ = true;
    is_infinite_ = false;
  }

  void Intersect(const FloatClipRect& other) {
    if (other.is_infinite_)
      return;

    if (is_infinite_) {
      is_infinite_ = other.is_infinite_;
      rect_ = other.rect_;
    } else {
      rect_.Intersect(other.Rect());
    }
    if (other.HasRadius())
      SetHasRadius();
    else if (!other.IsTight())
      ClearIsTight();
  }

  // See gfx::RectF::InclusiveIntersect for description of the return value.
  // TL;DR, this returns true if rects actually intersect.
  bool InclusiveIntersect(const FloatClipRect& other) {
    if (other.is_infinite_)
      return true;

    bool retval = true;
    if (is_infinite_) {
      is_infinite_ = other.is_infinite_;
      rect_ = other.rect_;
    } else {
      retval = rect_.InclusiveIntersect(other.Rect());
    }
    if (other.HasRadius())
      SetHasRadius();
    else if (!other.IsTight())
      ClearIsTight();
    return retval;
  }

  bool HasRadius() const { return has_radius_; }
  void SetHasRadius() {
    has_radius_ = true;
    is_tight_ = false;
    is_infinite_ = false;
  }

  // The rect is tight means that the rect covers only clipped result and
  // nothing else.
  bool IsTight() const {
    DCHECK(!is_tight_ || !has_radius_);
    return is_tight_;
  }
  void ClearIsTight() { is_tight_ = false; }

  void Move(const gfx::Vector2dF& offset) {
    if (is_infinite_)
      return;
    rect_.Offset(offset);
  }

  void Map(const gfx::Transform& matrix) {
    if (matrix.IsIdentityOr2dTranslation()) {
      Move(matrix.To2dTranslation());
      return;
    }
    // Otherwise assumes that transform makes the clip rect not tight.
    is_tight_ = false;
    if (is_infinite_)
      return;
    rect_ = matrix.MapRect(rect_);
  }

  bool IsInfinite() const { return is_infinite_; }

 private:
  gfx::RectF rect_;
  bool has_radius_ : 1;
  bool is_tight_ : 1;
  bool is_infinite_ : 1;
};

inline bool operator==(const FloatClipRect& a, const FloatClipRect& b) {
  if (a.IsTight() != b.IsTight())
    return false;
  if (a.IsInfinite() && b.IsInfinite())
    return true;
  return !a.IsInfinite() && !b.IsInfinite() && a.HasRadius() == b.HasRadius() &&
         a.Rect() == b.Rect();
}

inline bool operator!=(const FloatClipRect& a, const FloatClipRect& b) {
  return !(a == b);
}

// The difference between FloatClipRect() (infinite by default) and
// InfiniteLooseFloatClipRect() is that the former means no clip at all, and the
// latter means a clip with unknown bounds. The intersection of the former with
// a tight clip rect is tight, and that of the latter is loose.
inline FloatClipRect InfiniteLooseFloatClipRect() {
  FloatClipRect rect;
  rect.ClearIsTight();
  return rect;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_FLOAT_CLIP_RECT_H_
