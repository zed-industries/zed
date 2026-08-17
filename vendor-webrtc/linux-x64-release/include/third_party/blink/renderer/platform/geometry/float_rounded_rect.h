/*
 * Copyright (C) 2013 Adobe Systems Incorporated. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer.
 * 2. Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials
 *    provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_FLOAT_ROUNDED_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_FLOAT_ROUNDED_RECT_H_

#include <array>
#include <iosfwd>
#include <optional>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/skia/include/core/SkRRect.h"
#include "ui/gfx/geometry/insets_f.h"
#include "ui/gfx/geometry/outsets_f.h"
#include "ui/gfx/geometry/rect_f.h"
#include "ui/gfx/geometry/rrect_f.h"
#include "ui/gfx/geometry/size_f.h"
#include "ui/gfx/geometry/skia_conversions.h"

namespace gfx {
class QuadF;
}

namespace blink {

// Represents a rect with rounded corners, where the rounding of each corner is
// controlled by x radius and y radius.
//
// A simple example of a rounded corner is:
// <div style="width: 50px; height: 50px; border-radius: 10px;"/>
// Here, rect_ will be 0,0 50x50, and radii_ for each corner will be 10x10.
//
// The radii of each corner can be controlled independently:
// <div style="... border-top-right-radius: 25px 10px;"/>
// Here, the radii_.top_right_ will be 25x10.
//
// We don't use gfx::RRectF in blink because gfx::RRectF is based on SkRRect
// which always keeps the radii constrained within the size of the rect, but
// in blink sometimes we need to keep the unconstrained status of a rounded
// rect. See ConstrainRadii().

class PLATFORM_EXPORT FloatRoundedRect {
  DISALLOW_NEW();

 public:
  class PLATFORM_EXPORT Radii {
    DISALLOW_NEW();

   public:
    constexpr Radii() = default;
    constexpr Radii(const gfx::SizeF& top_left,
                    const gfx::SizeF& top_right,
                    const gfx::SizeF& bottom_left,
                    const gfx::SizeF& bottom_right)
        : top_left_(top_left),
          top_right_(top_right),
          bottom_left_(bottom_left),
          bottom_right_(bottom_right) {}
    explicit constexpr Radii(float radius) : Radii(radius, radius) {}
    constexpr Radii(float radius_x, float radius_y)
        : Radii(gfx::SizeF(radius_x, radius_y),
                gfx::SizeF(radius_x, radius_y),
                gfx::SizeF(radius_x, radius_y),
                gfx::SizeF(radius_x, radius_y)) {}

    constexpr Radii(const Radii&) = default;
    constexpr Radii& operator=(const Radii&) = default;
    constexpr bool operator==(const Radii&) const = default;

    void SetTopLeft(const gfx::SizeF& size) { top_left_ = size; }
    void SetTopRight(const gfx::SizeF& size) { top_right_ = size; }
    void SetBottomLeft(const gfx::SizeF& size) { bottom_left_ = size; }
    void SetBottomRight(const gfx::SizeF& size) { bottom_right_ = size; }
    constexpr const gfx::SizeF& TopLeft() const { return top_left_; }
    constexpr const gfx::SizeF& TopRight() const { return top_right_; }
    constexpr const gfx::SizeF& BottomLeft() const { return bottom_left_; }
    constexpr const gfx::SizeF& BottomRight() const { return bottom_right_; }

    void SetMinimumRadius(float);
    std::optional<float> UniformRadius() const;

    constexpr bool IsZero() const {
      return top_left_.IsZero() && top_right_.IsZero() &&
             bottom_left_.IsZero() && bottom_right_.IsZero();
    }

    String ToString() const;

   private:
    friend class FloatRoundedRect;
    void Scale(float factor);
    void Outset(const gfx::OutsetsF& outsets);
    void OutsetForMarginOrShadow(const gfx::OutsetsF&);
    void OutsetForShapeMargin(float outset);

    gfx::SizeF top_left_;
    gfx::SizeF top_right_;
    gfx::SizeF bottom_left_;
    gfx::SizeF bottom_right_;
  };

  constexpr FloatRoundedRect() = default;
  explicit FloatRoundedRect(const gfx::RectF&, const Radii& radii = Radii());
  explicit FloatRoundedRect(const gfx::Rect&, const Radii& radii = Radii());
  explicit FloatRoundedRect(const SkRRect& r)
      : FloatRoundedRect(gfx::RRectF(r)) {}
  explicit FloatRoundedRect(const gfx::RRectF&);
  constexpr bool operator==(const FloatRoundedRect&) const = default;
  FloatRoundedRect(float x, float y, float width, float height);
  FloatRoundedRect(const gfx::RectF& rect,
                   const gfx::SizeF& top_left,
                   const gfx::SizeF& top_right,
                   const gfx::SizeF& bottom_left,
                   const gfx::SizeF& bottom_right);
  FloatRoundedRect(const gfx::RectF& rect, float radius)
      : rect_(rect), radii_(radius) {}
  FloatRoundedRect(const gfx::RectF& r, float radius_x, float radius_y)
      : FloatRoundedRect(r, Radii(radius_x, radius_y)) {}

  constexpr const gfx::RectF& Rect() const { return rect_; }
  constexpr const Radii& GetRadii() const { return radii_; }
  constexpr bool IsRounded() const { return !radii_.IsZero(); }
  constexpr bool IsEmpty() const { return rect_.IsEmpty(); }

  void SetRect(const gfx::RectF& rect) { rect_ = rect; }
  void SetRadii(const Radii& radii) { radii_ = radii; }

  void Move(const gfx::Vector2dF& offset) { rect_.Offset(offset); }

  // Inflates/shrinks the rounded rect by the specified amount on each side and
  // corner. Zero widths and heights of radii are kept zero so that sharp
  // corners are still sharp. Each side of |outsets|/|insets| can be positive,
  // zero or negative independently.
  void Outset(const gfx::OutsetsF& outsets);
  void Outset(float outset) { Outset(gfx::OutsetsF(outset)); }
  void Inset(const gfx::InsetsF& insets) { Outset(insets.ToOutsets()); }
  void Inset(float inset) { Inset(gfx::InsetsF(inset)); }

  // Inflates (or shrinks if |outset| is negative) the rect and the corners
  // based on the margin edge algorithm in
  // https://drafts.csswg.org/css-backgrounds-3/#corner-shaping which is the
  // same as the shadow spread algorithm in
  // https://drafts.csswg.org/css-backgrounds-3/#shadow-shape.
  // TODO(wangxianzhu): Consider merging this into Outset()/Inset() to apply
  // the margin/shadow algorithm to all outsets except shape-margin. For now
  // this is blocked by a problem of the algorithm
  // (https://github.com/w3c/csswg-drafts/issues/7103).
  void OutsetForMarginOrShadow(const gfx::OutsetsF& outsets);
  void OutsetForMarginOrShadow(float outset) {
    OutsetForMarginOrShadow(gfx::OutsetsF(outset));
  }

  // Inflates the rounded rect by the specified amount on each side and corner
  // for shape-margin. |outset| must be non-negative. This is different from
  // other outset methods in that it always expands by radial distance (always
  // produces rounding) rather than following rules for sharp corner
  // preservation and cubic reduction of the radius. See
  // https://drafts.csswg.org/css-shapes/#shape-margin-property.
  void OutsetForShapeMargin(float outset);

  constexpr gfx::RectF TopLeftCorner() const {
    return gfx::RectF(rect_.x(), rect_.y(), radii_.TopLeft().width(),
                      radii_.TopLeft().height());
  }
  constexpr gfx::RectF TopRightCorner() const {
    return gfx::RectF(rect_.right() - radii_.TopRight().width(), rect_.y(),
                      radii_.TopRight().width(), radii_.TopRight().height());
  }
  constexpr gfx::RectF BottomLeftCorner() const {
    return gfx::RectF(rect_.x(), rect_.bottom() - radii_.BottomLeft().height(),
                      radii_.BottomLeft().width(),
                      radii_.BottomLeft().height());
  }
  constexpr gfx::RectF BottomRightCorner() const {
    return gfx::RectF(rect_.right() - radii_.BottomRight().width(),
                      rect_.bottom() - radii_.BottomRight().height(),
                      radii_.BottomRight().width(),
                      radii_.BottomRight().height());
  }

  bool XInterceptsAtY(float y,
                      float& min_x_intercept,
                      float& max_x_intercept) const;

  // Tests whether the quad intersects any part of this rounded rectangle.
  // This only works for convex quads.
  // This intersection is edge-inclusive and will return true even if the
  // intersecting area is empty (i.e., the intersection is a line or a point).
  bool IntersectsQuad(const gfx::QuadF&) const;

  // Whether the radii are constrained in the size of rect().
  bool IsRenderable() const;

  // Constrains the radii to be no bigger than the size of rect().
  // This is not called automatically in this class because sometimes we want
  // to keep the !IsRenderable() status, e.g. for a rounded inner border edge
  // that is shrunk from a rounded outer border edge to keep uniform width of
  // the rounded border.
  void ConstrainRadii();

  explicit operator SkRRect() const;
  explicit operator gfx::RRectF() const { return gfx::RRectF(SkRRect(*this)); }

  String ToString() const;

 private:
  gfx::RectF rect_;
  Radii radii_;
};

inline FloatRoundedRect::operator SkRRect() const {
  SkRRect rrect;

  if (IsRounded()) {
    std::array<SkVector, 4> radii;
    radii[SkRRect::kUpperLeft_Corner].set(TopLeftCorner().width(),
                                          TopLeftCorner().height());
    radii[SkRRect::kUpperRight_Corner].set(TopRightCorner().width(),
                                           TopRightCorner().height());
    radii[SkRRect::kLowerRight_Corner].set(BottomRightCorner().width(),
                                           BottomRightCorner().height());
    radii[SkRRect::kLowerLeft_Corner].set(BottomLeftCorner().width(),
                                          BottomLeftCorner().height());

    rrect.setRectRadii(gfx::RectFToSkRect(Rect()), radii.data());
  } else {
    rrect.setRect(gfx::RectFToSkRect(Rect()));
  }

  return rrect;
}

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&,
                                         const FloatRoundedRect&);
PLATFORM_EXPORT std::ostream& operator<<(std::ostream&,
                                         const FloatRoundedRect::Radii&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_FLOAT_ROUNDED_RECT_H_
