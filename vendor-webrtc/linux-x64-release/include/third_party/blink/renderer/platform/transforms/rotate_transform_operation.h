/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2006 Graham Dennis (graham.dennis@gmail.com)
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_ROTATE_TRANSFORM_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_ROTATE_TRANSFORM_OPERATION_H_

#include "third_party/blink/renderer/platform/transforms/rotation.h"
#include "third_party/blink/renderer/platform/transforms/transform_operation.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "ui/gfx/geometry/vector3d_f.h"

namespace blink {

class PLATFORM_EXPORT RotateTransformOperation : public TransformOperation {
 public:
  RotateTransformOperation(const Rotation& rotation, OperationType type)
      : rotation_(rotation), type_(type) {}

  RotateTransformOperation(double angle, OperationType type)
      : RotateTransformOperation((Rotation(gfx::Vector3dF(0, 0, 1), angle)),
                                 type) {}

  RotateTransformOperation(double x,
                           double y,
                           double z,
                           double angle,
                           OperationType type)
      : RotateTransformOperation((Rotation(gfx::Vector3dF(x, y, z), angle)),
                                 type) {}

  double X() const { return rotation_.axis.x(); }
  double Y() const { return rotation_.axis.y(); }
  double Z() const { return rotation_.axis.z(); }
  double Angle() const { return rotation_.angle; }
  const gfx::Vector3dF& Axis() const { return rotation_.axis; }

  static bool GetCommonAxis(const RotateTransformOperation*,
                            const RotateTransformOperation*,
                            gfx::Vector3dF& result_axis,
                            double& result_angle_a,
                            double& result_angle_b);

  OperationType GetType() const override { return type_; }
  OperationType PrimitiveType() const override { return kRotate3D; }

  void Apply(gfx::Transform& transform,
             const gfx::SizeF& /*borderBoxSize*/) const override {
    if (type_ == kRotate)
      transform.Rotate(Angle());
    else
      transform.RotateAbout(rotation_.axis, rotation_.angle);
  }

  static bool IsMatchingOperationType(OperationType type) {
    return type == kRotate || type == kRotateX || type == kRotateY ||
           type == kRotateZ || type == kRotate3D;
  }

 protected:
  bool IsEqualAssumingSameType(const TransformOperation&) const override;

  bool HasNonTrivial3DComponent() const override {
    return Angle() && (X() || Y());
  }

  TransformOperation* Accumulate(const TransformOperation& other) override;
  TransformOperation* Blend(const TransformOperation* from,
                            double progress,
                            bool blend_to_identity = false) override;
  TransformOperation* Zoom(double factor) override { return this; }

  const Rotation rotation_;
  const OperationType type_;
};

template <>
struct DowncastTraits<RotateTransformOperation> {
  static bool AllowFrom(const TransformOperation& transform) {
    return RotateTransformOperation::IsMatchingOperationType(
        transform.GetType());
  }
};

class PLATFORM_EXPORT RotateAroundOriginTransformOperation final
    : public RotateTransformOperation {
 public:
  RotateAroundOriginTransformOperation(double angle,
                                       double origin_x,
                                       double origin_y);

  void Apply(gfx::Transform&, const gfx::SizeF&) const override;

  static bool IsMatchingOperationType(OperationType type) {
    return type == kRotateAroundOrigin;
  }
  OperationType PrimitiveType() const override { return kRotateAroundOrigin; }

 protected:
  bool IsEqualAssumingSameType(const TransformOperation&) const override;

 private:
  TransformOperation* Blend(const TransformOperation* from,
                            double progress,
                            bool blend_to_identity = false) override;
  TransformOperation* Zoom(double factor) override;

  double origin_x_;
  double origin_y_;
};

template <>
struct DowncastTraits<RotateAroundOriginTransformOperation> {
  static bool AllowFrom(const TransformOperation& transform) {
    return RotateAroundOriginTransformOperation::IsMatchingOperationType(
        transform.GetType());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_ROTATE_TRANSFORM_OPERATION_H_
