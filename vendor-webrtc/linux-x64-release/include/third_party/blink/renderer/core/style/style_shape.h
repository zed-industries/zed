// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SHAPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SHAPE_H_

#include <cstddef>
#include <optional>
#include <variant>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/style/basic_shapes.h"
#include "third_party/blink/renderer/core/svg/svg_path_data.h"
#include "third_party/blink/renderer/platform/geometry/length_point.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

// The computed version of the shape() function
// https://drafts.csswg.org/css-shapes-2/#shape-function
// It stores each shape segment as a struct with the appropriate Length or enum
// values.
class StyleShape final : public BasicShape {
 public:
  using CloseSegment = std::monostate;
  template <SVGPathSegType Type>
  struct SegmentWithTargetPoint {
    static constexpr SVGPathSegType kSegType = Type;
    LengthPoint target_point;
    bool operator==(const SegmentWithTargetPoint& other) const = default;
  };
  struct MoveToSegment
      : public SegmentWithTargetPoint<SVGPathSegType::kPathSegMoveToAbs> {};
  struct MoveBySegment
      : public SegmentWithTargetPoint<SVGPathSegType::kPathSegMoveToRel> {};
  struct LineToSegment
      : public SegmentWithTargetPoint<SVGPathSegType::kPathSegLineToAbs> {};
  struct LineBySegment
      : public SegmentWithTargetPoint<SVGPathSegType::kPathSegLineToRel> {};
  struct HLineSegment {
    Length x;
    bool operator==(const HLineSegment& other) const = default;
  };
  struct HLineToSegment : public HLineSegment {
    static constexpr SVGPathSegType kSegType =
        SVGPathSegType::kPathSegLineToHorizontalAbs;
  };
  struct HLineBySegment : public HLineSegment {
    static constexpr SVGPathSegType kSegType =
        SVGPathSegType::kPathSegLineToHorizontalRel;
  };

  struct VLineSegment {
    Length y;
    bool operator==(const VLineSegment& other) const = default;
  };
  struct VLineToSegment : public VLineSegment {
    static constexpr SVGPathSegType kSegType =
        SVGPathSegType::kPathSegLineToVerticalAbs;
  };
  struct VLineBySegment : public VLineSegment {
    static constexpr SVGPathSegType kSegType =
        SVGPathSegType::kPathSegLineToVerticalRel;
  };
  struct ControlPoint {
    enum class Origin { kSegmentStart, kSegmentEnd, kReferenceBox };
    Origin origin;
    LengthPoint point;
    bool operator==(const ControlPoint& other) const = default;
  };

  template <size_t NumControlPoints, SVGPathSegType Type>
  struct CurveSegment : SegmentWithTargetPoint<Type> {
    static constexpr size_t GetNumControlPoints() { return NumControlPoints; }
    std::array<ControlPoint, NumControlPoints> control_points;
    bool operator==(const CurveSegment<NumControlPoints, Type>& other) const =
        default;
  };
  struct CubicCurveToSegment
      : public CurveSegment<2, SVGPathSegType::kPathSegCurveToCubicAbs> {};
  struct CubicCurveBySegment
      : public CurveSegment<2, SVGPathSegType::kPathSegCurveToCubicRel> {};
  struct QuadraticCurveToSegment
      : public CurveSegment<1, SVGPathSegType::kPathSegCurveToQuadraticAbs> {};
  struct QuadraticCurveBySegment
      : public CurveSegment<1, SVGPathSegType::kPathSegCurveToQuadraticRel> {};
  struct SmoothCubicCurveToSegment
      : public CurveSegment<1, SVGPathSegType::kPathSegCurveToCubicSmoothAbs> {
  };
  struct SmoothCubicCurveBySegment
      : public CurveSegment<1, SVGPathSegType::kPathSegCurveToCubicSmoothRel> {
  };
  struct SmoothQuadraticCurveToSegment
      : public SegmentWithTargetPoint<
            SVGPathSegType::kPathSegCurveToQuadraticSmoothAbs> {};
  struct SmoothQuadraticCurveBySegment
      : public SegmentWithTargetPoint<
            SVGPathSegType::kPathSegCurveToQuadraticSmoothRel> {};

  template <SVGPathSegType Type>
  struct ArcSegment : public SegmentWithTargetPoint<Type> {
    double angle;
    LengthSize radius;
    bool large;
    bool sweep;
    bool operator==(const ArcSegment& other) const = default;
  };
  struct ArcToSegment : public ArcSegment<SVGPathSegType::kPathSegArcAbs> {};
  struct ArcBySegment : public ArcSegment<SVGPathSegType::kPathSegArcRel> {};

  using Segment = std::variant<MoveToSegment,
                               MoveBySegment,
                               LineToSegment,
                               LineBySegment,
                               HLineToSegment,
                               HLineBySegment,
                               VLineToSegment,
                               VLineBySegment,
                               CubicCurveToSegment,
                               CubicCurveBySegment,
                               QuadraticCurveToSegment,
                               QuadraticCurveBySegment,
                               SmoothCubicCurveToSegment,
                               SmoothCubicCurveBySegment,
                               SmoothQuadraticCurveToSegment,
                               SmoothQuadraticCurveBySegment,
                               ArcToSegment,
                               ArcBySegment,
                               CloseSegment>;

  StyleShape(WindRule wind_rule,
             const LengthPoint& origin,
             Vector<Segment> segments);

  ShapeType GetType() const override { return kStyleShapeType; }
  Path GetPath(const gfx::RectF& bounding_box,
               float zoom,
               float path_scale) const override;

  WindRule GetWindRule() const { return wind_rule_; }
  const LengthPoint& GetOrigin() const { return origin_; }
  const Vector<Segment>& Segments() const { return segments_; }

 protected:
  bool IsEqualAssumingSameType(const BasicShape&) const override;

 private:
  WindRule wind_rule_;
  LengthPoint origin_;
  Vector<Segment> segments_;
};

template <>
struct DowncastTraits<StyleShape> {
  static bool AllowFrom(const BasicShape& value) {
    return value.GetType() == BasicShape::kStyleShapeType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SHAPE_H_
