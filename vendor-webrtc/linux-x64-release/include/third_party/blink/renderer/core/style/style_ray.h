// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_RAY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_RAY_H_

#include "third_party/blink/renderer/core/style/basic_shapes.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace gfx {
class PointF;
}

namespace blink {

struct PointAndTangent;

class StyleRay : public BasicShape {
 public:
  enum class RaySize {
    kClosestSide,
    kClosestCorner,
    kFarthestSide,
    kFarthestCorner,
    kSides
  };

  StyleRay(float angle,
           RaySize,
           bool contain,
           const BasicShapeCenterCoordinate& center_x,
           const BasicShapeCenterCoordinate& center_y,
           bool has_explicit_center);
  ~StyleRay() override = default;

  float CalculateRayPathLength(const gfx::PointF& starting_point,
                               const gfx::SizeF& reference_box_size) const;
  PointAndTangent PointAndNormalAtLength(const gfx::PointF& starting_point,
                                         float length) const;

  float Angle() const { return ClampTo<float, float>(angle_); }
  RaySize Size() const { return size_; }
  bool Contain() const { return contain_; }

  bool HasExplicitCenter() const { return has_explicit_center_; }
  const BasicShapeCenterCoordinate& CenterX() const { return center_x_; }
  const BasicShapeCenterCoordinate& CenterY() const { return center_y_; }

  Path GetPath(const gfx::RectF&, float, float) const override;

  ShapeType GetType() const override { return kStyleRayType; }

 protected:
  bool IsEqualAssumingSameType(const BasicShape&) const override;

 private:
  float angle_;
  RaySize size_;
  bool contain_;
  BasicShapeCenterCoordinate center_x_;
  BasicShapeCenterCoordinate center_y_;
  bool has_explicit_center_ = true;
};

template <>
struct DowncastTraits<StyleRay> {
  static bool AllowFrom(const BasicShape& value) {
    return value.GetType() == BasicShape::kStyleRayType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_RAY_H_
