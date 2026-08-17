// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_CONTOURED_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_CONTOURED_RECT_H_

#include <optional>

#include "third_party/blink/renderer/platform/geometry/float_rounded_rect.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/insets_f.h"
#include "ui/gfx/geometry/outsets_f.h"
#include "ui/gfx/geometry/rect_f.h"
#include "ui/gfx/geometry/rrect_f.h"
#include "ui/gfx/geometry/size_f.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace gfx {
class QuadF;
}

namespace blink {

class Path;

// A ContouredRect is a rect with corners that can have arbitrary
// radius and curvature, as in not necessarily rounded.
// It is a superset of FloatRoundedRect, with the added information needed
// to render corner shapes correctly.
class PLATFORM_EXPORT ContouredRect {
  DISALLOW_NEW();

 public:
  // A corner curvature is the exponent of a superellipse quadrant.
  // e.g. 2 is round, 1 is bevel/angled, infinity is defunct/straight.
  class CornerCurvature {
   public:
    static constexpr float kRound = 2;
    static constexpr float kBevel = 1;
    static constexpr float kStraight = 1000;
    static constexpr float kNotch = 1 / kStraight;

    constexpr CornerCurvature() = default;
    constexpr CornerCurvature(float top_left,
                              float top_right,
                              float bottom_right,
                              float bottom_left)
        : top_left_(top_left),
          top_right_(top_right),
          bottom_right_(bottom_right),
          bottom_left_(bottom_left) {
      DCHECK_GE(top_left, 0);
      DCHECK_GE(top_right, 0);
      DCHECK_GE(bottom_right, 0);
      DCHECK_GE(bottom_left, 0);
    }

    constexpr bool IsRound() const {
      return (top_left_ == kRound) && IsUniform();
    }

    constexpr bool IsUniform() const {
      return top_left_ == top_right_ && top_left_ == bottom_right_ &&
             top_left_ == bottom_left_;
    }

    constexpr float TopLeft() const { return top_left_; }
    constexpr float TopRight() const { return top_right_; }
    constexpr float BottomRight() const { return bottom_right_; }
    constexpr float BottomLeft() const { return bottom_left_; }

    constexpr bool operator==(const CornerCurvature&) const = default;

    String ToString() const;

   private:
    float top_left_ = kRound;
    float top_right_ = kRound;
    float bottom_right_ = kRound;
    float bottom_left_ = kRound;
  };

  // A Corner is a axis-aligned quad, with the points ordered (start, outer,
  // end, center), and a curvature. It is used as a convenient way to perform
  // corner-related computations without having to worry about the
  // transpositions of the different corners.
  class PLATFORM_EXPORT Corner {
   public:
    constexpr Corner(const gfx::RectF& rect, size_t rotation, float curvature)
        : curvature_(curvature) {
      vertices_ = {rect.origin(), rect.top_right(), rect.bottom_right(),
                   rect.bottom_left()};
      std::rotate(vertices_.begin(), vertices_.begin() + rotation,
                  vertices_.end());
    }

    constexpr Corner(const std::array<gfx::PointF, 4>& vertices,
                     float curvature)
        : vertices_(vertices), curvature_(ClampCurvature(curvature)) {}

    constexpr const gfx::PointF& Start() const { return vertices_.at(0); }
    constexpr const gfx::PointF& Outer() const { return vertices_.at(1); }
    constexpr const gfx::PointF& End() const { return vertices_.at(2); }
    constexpr const gfx::PointF& Center() const { return vertices_.at(3); }
    static constexpr float ClampCurvature(float curvature) {
      return std::clamp(curvature, CornerCurvature::kNotch,
                        CornerCurvature::kStraight);
    }
    constexpr float Curvature() const { return curvature_; }
    constexpr bool IsStraight() const {
      return Curvature() == CornerCurvature::kStraight;
    }
    constexpr bool IsBevel() const {
      return curvature_ == CornerCurvature::kBevel;
    }
    constexpr bool IsRound() const {
      return curvature_ == CornerCurvature::kRound;
    }
    constexpr bool IsNotch() const {
      return curvature_ == CornerCurvature::kNotch;
    }
    constexpr bool IsConcave() const { return curvature_ < 1; }
    constexpr bool IsZero() const { return Start() == End(); }
    constexpr bool operator==(const Corner&) const = default;
    constexpr Corner Inverse() const {
      return Corner({Start(), Center(), End(), Outer()}, 1 / Curvature());
    }

    constexpr gfx::Vector2dF v1() const { return Outer() - Start(); }
    constexpr gfx::Vector2dF v2() const { return End() - Outer(); }
    constexpr gfx::Vector2dF v3() const { return Center() - End(); }
    constexpr gfx::Vector2dF v4() const { return Start() - Center(); }
    constexpr float DiagonalLength() const {
      return (End() - Start()).Length();
    }
    static constexpr float HalfCornerForCurvature(float curvature) {
      return std::pow(0.5, 1 / ClampCurvature(curvature));
    }

    static float CurvatureForHalfCorner(float half_corner);

    constexpr gfx::PointF MapPoint(
        const gfx::Vector2dF& normalized_point) const {
      return Center() + gfx::ScaleVector2d(v1(), normalized_point.x()) +
             gfx::ScaleVector2d(v4(), normalized_point.y());
    }

    Corner AlignedToOrigin(const Corner& origin) const;
    String ToString() const;

   private:
    std::array<gfx::PointF, 4> vertices_;
    float curvature_;
  };

  constexpr ContouredRect() = default;
  constexpr explicit ContouredRect(const FloatRoundedRect& rect)
      : rect_(rect) {}
  constexpr ContouredRect(const FloatRoundedRect& rect,
                          const CornerCurvature& curvature)
      : rect_(rect), corner_curvature_(curvature) {}
  bool operator==(const ContouredRect&) const = default;
  constexpr const gfx::RectF& BoundingRect() const { return rect_.Rect(); }
  constexpr const CornerCurvature& GetCornerCurvature() const {
    return corner_curvature_;
  }

  constexpr bool HasRoundCurvature() const {
    return corner_curvature_.IsRound() || !IsRounded();
  }

  const FloatRoundedRect::Radii& GetRadii() const { return rect_.GetRadii(); }

  void SetRadii(const FloatRoundedRect::Radii& radii) { rect_.SetRadii(radii); }

  bool IsRounded() const { return rect_.IsRounded(); }

  const FloatRoundedRect& AsRoundedRect() const { return rect_; }

  const gfx::RectF& Rect() const { return rect_.Rect(); }

  constexpr bool IsEmpty() const { return rect_.IsEmpty(); }

  void SetCornerCurvature(const CornerCurvature& curvature) {
    corner_curvature_ = curvature;
  }

  void Move(const gfx::Vector2dF& offset) { rect_.Move(offset); }
  void Inset(const gfx::InsetsF& insets) { rect_.Inset(insets); }
  void Inset(float inset) { rect_.Inset(inset); }
  void OutsetForMarginOrShadow(float outset) {
    OutsetForMarginOrShadow(gfx::OutsetsF(outset));
  }

  void Outset(const gfx::OutsetsF& outsets) { rect_.Outset(outsets); }
  void OutsetForMarginOrShadow(const gfx::OutsetsF&);

  void ConstrainRadii() { rect_.ConstrainRadii(); }

  bool IntersectsQuad(const gfx::QuadF&) const;

  // Whether the radii are constrained in the size of rect().
  bool IsRenderable() const { return rect_.IsRenderable(); }
  String ToString() const;
  Path GetPath() const;
  const FloatRoundedRect& GetOriginRect() const {
    return origin_rect_ ? *origin_rect_ : rect_;
  }
  void SetOriginRect(const FloatRoundedRect& rect) { origin_rect_ = rect; }

  constexpr Corner TopRightCorner() const {
    return origin_rect_ ? TopRightCornerInternal().AlignedToOrigin(
                              ContouredRect(*origin_rect_, corner_curvature_)
                                  .TopRightCornerInternal())
                        : TopRightCornerInternal();
  }

  constexpr Corner BottomRightCorner() const {
    return origin_rect_ ? BottomRightCornerInternal().AlignedToOrigin(
                              ContouredRect(*origin_rect_, corner_curvature_)
                                  .BottomRightCornerInternal())
                        : BottomRightCornerInternal();
  }

  constexpr Corner BottomLeftCorner() const {
    return origin_rect_ ? BottomLeftCornerInternal().AlignedToOrigin(
                              ContouredRect(*origin_rect_, corner_curvature_)
                                  .BottomLeftCornerInternal())
                        : BottomLeftCornerInternal();
  }

  constexpr Corner TopLeftCorner() const {
    return origin_rect_ ? TopLeftCornerInternal().AlignedToOrigin(
                              ContouredRect(*origin_rect_, corner_curvature_)
                                  .TopLeftCornerInternal())
                        : TopLeftCornerInternal();
  }

 private:
  constexpr Corner TopRightCornerInternal() const {
    return Corner(rect_.TopRightCorner(), 0, corner_curvature_.TopRight());
  }
  constexpr Corner BottomRightCornerInternal() const {
    return Corner(rect_.BottomRightCorner(), 1,
                  corner_curvature_.BottomRight());
  }
  constexpr Corner BottomLeftCornerInternal() const {
    return Corner(rect_.BottomLeftCorner(), 2, corner_curvature_.BottomLeft());
  }
  constexpr Corner TopLeftCornerInternal() const {
    return Corner(rect_.TopLeftCorner(), 3, corner_curvature_.TopLeft());
  }

  FloatRoundedRect rect_;
  CornerCurvature corner_curvature_;
  std::optional<FloatRoundedRect> origin_rect_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_CONTOURED_RECT_H_
