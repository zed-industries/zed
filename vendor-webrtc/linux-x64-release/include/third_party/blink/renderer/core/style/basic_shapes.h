/*
 * Copyright (C) 2012 Adobe Systems Incorporated. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDER "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY,
 * OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
 * TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BASIC_SHAPES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BASIC_SHAPES_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/geometry/length_size.h"
#include "third_party/blink/renderer/platform/geometry/path_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace gfx {
class PointF;
class RectF;
class SizeF;
}  // namespace gfx

namespace blink {

class Path;

class CORE_EXPORT BasicShape : public GarbageCollected<BasicShape> {
 public:
  virtual ~BasicShape() = default;
  virtual void Trace(Visitor*) const {}

  enum ShapeType {
    kBasicShapeEllipseType,
    kBasicShapePolygonType,
    kBasicShapeCircleType,
    kBasicShapeInsetType,
    kStyleRayType,
    kStylePathType,
    kStyleShapeType,
  };

  bool IsSameType(const BasicShape& other) const {
    return GetType() == other.GetType();
  }

  virtual Path GetPath(const gfx::RectF&,
                       float zoom,
                       float path_scale) const = 0;
  bool operator==(const BasicShape& o) const {
    return IsSameType(o) && IsEqualAssumingSameType(o);
  }

  virtual ShapeType GetType() const = 0;

 protected:
  BasicShape() = default;

  virtual bool IsEqualAssumingSameType(const BasicShape&) const = 0;
};

class BasicShapeCenterCoordinate {
  DISALLOW_NEW();

 public:
  enum Direction { kTopLeft, kBottomRight };

  explicit BasicShapeCenterCoordinate(Direction direction = kTopLeft,
                                      const Length& length = Length::Fixed(0))
      : direction_(direction),
        length_(length),
        computed_length_(direction == kTopLeft
                             ? length
                             : length.SubtractFromOneHundredPercent()) {}

  BasicShapeCenterCoordinate(const BasicShapeCenterCoordinate&) = default;
  BasicShapeCenterCoordinate& operator=(const BasicShapeCenterCoordinate&) =
      default;

  bool operator==(const BasicShapeCenterCoordinate& other) const {
    return direction_ == other.direction_ && length_ == other.length_ &&
           computed_length_ == other.computed_length_;
  }

  Direction GetDirection() const { return direction_; }

  // <length> that has not been adjusted based on starting edge/direction. Used
  // for recreating a CSSValue.
  const Length& length() const { return length_; }

  // <length> that has been adjusted based on starting edge/direction. Used to
  // produce a used value.
  const Length& ComputedLength() const { return computed_length_; }

 private:
  Direction direction_;
  Length length_;
  Length computed_length_;
};

// Resolve a pair of <x, y> "center coordinates" into a point.
gfx::PointF PointForCenterCoordinate(const BasicShapeCenterCoordinate& center_x,
                                     const BasicShapeCenterCoordinate& center_y,
                                     gfx::SizeF box_size);

class BasicShapeRadius {
  DISALLOW_NEW();

 public:
  enum RadiusType { kValue, kClosestSide, kFarthestSide };
  BasicShapeRadius() : type_(kClosestSide) {}
  explicit BasicShapeRadius(const Length& v) : value_(v), type_(kValue) {}
  explicit BasicShapeRadius(RadiusType t) : type_(t) {}
  BasicShapeRadius(const BasicShapeRadius&) = default;
  BasicShapeRadius& operator=(const BasicShapeRadius&) = default;
  bool operator==(const BasicShapeRadius& other) const {
    return type_ == other.type_ && value_ == other.value_;
  }

  const Length& Value() const { return value_; }
  RadiusType GetType() const { return type_; }

 private:
  Length value_;
  RadiusType type_;
};

class BasicShapeWithCenterAndRadii : public BasicShape {
 public:
  void SetHasExplicitCenter(bool is_center_explicitly_set) {
    is_center_explicitly_set_ = is_center_explicitly_set;
  }
  bool HasExplicitCenter() const { return is_center_explicitly_set_; }

  void SetCenterX(BasicShapeCenterCoordinate center_x) { center_x_ = center_x; }
  void SetCenterY(BasicShapeCenterCoordinate center_y) { center_y_ = center_y; }
  const BasicShapeCenterCoordinate& CenterX() const { return center_x_; }
  const BasicShapeCenterCoordinate& CenterY() const { return center_y_; }

  virtual Path GetPathFromCenter(const gfx::PointF&,
                                 const gfx::RectF&,
                                 float path_scale) const = 0;

 protected:
  BasicShapeCenterCoordinate center_x_;
  BasicShapeCenterCoordinate center_y_;

 private:
  bool is_center_explicitly_set_ = true;
};

template <>
struct DowncastTraits<BasicShapeWithCenterAndRadii> {
  static bool AllowFrom(const BasicShape& value) {
    BasicShape::ShapeType type = value.GetType();
    return type == BasicShape::kBasicShapeCircleType ||
           type == BasicShape::kBasicShapeEllipseType;
  }
};

class CORE_EXPORT BasicShapeCircle final : public BasicShapeWithCenterAndRadii {
 public:
  BasicShapeCircle() = default;

  const BasicShapeRadius& Radius() const { return radius_; }

  float FloatValueForRadiusInBox(const gfx::PointF& center,
                                 const gfx::SizeF& box_size) const;
  void SetRadius(BasicShapeRadius radius) { radius_ = radius; }

  Path GetPath(const gfx::RectF&, float, float) const override;
  Path GetPathFromCenter(const gfx::PointF&,
                         const gfx::RectF&,
                         float) const override;

  ShapeType GetType() const override { return kBasicShapeCircleType; }

 protected:
  bool IsEqualAssumingSameType(const BasicShape&) const override;

 private:
  BasicShapeRadius radius_;
};

template <>
struct DowncastTraits<BasicShapeCircle> {
  static bool AllowFrom(const BasicShape& value) {
    return value.GetType() == BasicShape::kBasicShapeCircleType;
  }
};

class BasicShapeEllipse final : public BasicShapeWithCenterAndRadii {
 public:
  BasicShapeEllipse() = default;

  const BasicShapeRadius& RadiusX() const { return radius_x_; }
  const BasicShapeRadius& RadiusY() const { return radius_y_; }
  float FloatValueForRadiusInBox(const BasicShapeRadius&,
                                 float center,
                                 float box_width_or_height) const;

  void SetRadiusX(BasicShapeRadius radius_x) { radius_x_ = radius_x; }
  void SetRadiusY(BasicShapeRadius radius_y) { radius_y_ = radius_y; }

  Path GetPath(const gfx::RectF&, float, float) const override;
  Path GetPathFromCenter(const gfx::PointF&,
                         const gfx::RectF&,
                         float) const override;

  ShapeType GetType() const override { return kBasicShapeEllipseType; }

 protected:
  bool IsEqualAssumingSameType(const BasicShape&) const override;

 private:

  BasicShapeRadius radius_x_;
  BasicShapeRadius radius_y_;
};

template <>
struct DowncastTraits<BasicShapeEllipse> {
  static bool AllowFrom(const BasicShape& value) {
    return value.GetType() == BasicShape::kBasicShapeEllipseType;
  }
};

class BasicShapePolygon final : public BasicShape {
 public:
  BasicShapePolygon() : wind_rule_(RULE_NONZERO) {}

  const Vector<Length>& Values() const { return values_; }

  void SetWindRule(WindRule wind_rule) { wind_rule_ = wind_rule; }
  void AppendPoint(const Length& x, const Length& y) {
    values_.push_back(x);
    values_.push_back(y);
  }

  Path GetPath(const gfx::RectF&, float, float) const override;

  WindRule GetWindRule() const { return wind_rule_; }

  ShapeType GetType() const override { return kBasicShapePolygonType; }

 protected:
  bool IsEqualAssumingSameType(const BasicShape&) const override;

 private:
  WindRule wind_rule_;
  Vector<Length> values_;
};

template <>
struct DowncastTraits<BasicShapePolygon> {
  static bool AllowFrom(const BasicShape& value) {
    return value.GetType() == BasicShape::kBasicShapePolygonType;
  }
};

class BasicShapeInset final : public BasicShape {
 public:
  BasicShapeInset() = default;

  ShapeType GetType() const override { return kBasicShapeInsetType; }
  Path GetPath(const gfx::RectF&, float, float) const override;

  const Length& Top() const { return top_; }
  const Length& Right() const { return right_; }
  const Length& Bottom() const { return bottom_; }
  const Length& Left() const { return left_; }

  const LengthSize& TopLeftRadius() const { return top_left_radius_; }
  const LengthSize& TopRightRadius() const { return top_right_radius_; }
  const LengthSize& BottomRightRadius() const { return bottom_right_radius_; }
  const LengthSize& BottomLeftRadius() const { return bottom_left_radius_; }

  void SetTop(const Length& top) { top_ = top; }
  void SetRight(const Length& right) { right_ = right; }
  void SetBottom(const Length& bottom) { bottom_ = bottom; }
  void SetLeft(const Length& left) { left_ = left; }

  void SetTopLeftRadius(const LengthSize& radius) { top_left_radius_ = radius; }
  void SetTopRightRadius(const LengthSize& radius) {
    top_right_radius_ = radius;
  }
  void SetBottomRightRadius(const LengthSize& radius) {
    bottom_right_radius_ = radius;
  }
  void SetBottomLeftRadius(const LengthSize& radius) {
    bottom_left_radius_ = radius;
  }

 private:

  bool IsEqualAssumingSameType(const BasicShape&) const override;

  Length right_;
  Length top_;
  Length bottom_;
  Length left_;

  LengthSize top_left_radius_;
  LengthSize top_right_radius_;
  LengthSize bottom_right_radius_;
  LengthSize bottom_left_radius_;
};

template <>
struct DowncastTraits<BasicShapeInset> {
  static bool AllowFrom(const BasicShape& value) {
    return value.GetType() == BasicShape::kBasicShapeInsetType;
  }
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BASIC_SHAPES_H_
