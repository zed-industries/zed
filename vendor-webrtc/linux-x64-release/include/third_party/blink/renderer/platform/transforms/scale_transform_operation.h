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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_SCALE_TRANSFORM_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_SCALE_TRANSFORM_OPERATION_H_

#include "third_party/blink/renderer/platform/transforms/transform_operation.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class PLATFORM_EXPORT ScaleTransformOperation final
    : public TransformOperation {
 public:
  ScaleTransformOperation(double sx, double sy, double sz, OperationType type)
      : x_(sx), y_(sy), z_(sz), type_(type) {
    DCHECK(IsMatchingOperationType(type));
  }
  ScaleTransformOperation(double sx, double sy, OperationType type)
      : ScaleTransformOperation(sx, sy, 1, type) {}

  double X() const { return x_; }
  double Y() const { return y_; }
  double Z() const { return z_; }

  void Apply(gfx::Transform& transform, const gfx::SizeF&) const override {
    transform.Scale3d(x_, y_, z_);
  }
  TransformOperation* Accumulate(const TransformOperation& other) override;
  TransformOperation* Blend(const TransformOperation* from,
                            double progress,
                            bool blend_to_identity = false) override;

  static bool IsMatchingOperationType(OperationType type) {
    return type == kScale || type == kScaleX || type == kScaleY ||
           type == kScaleZ || type == kScale3D;
  }

  OperationType GetType() const override { return type_; }
  OperationType PrimitiveType() const final { return kScale3D; }

 protected:
  bool IsEqualAssumingSameType(const TransformOperation& o) const override {
    const ScaleTransformOperation* s =
        static_cast<const ScaleTransformOperation*>(&o);
    return x_ == s->x_ && y_ == s->y_ && z_ == s->z_;
  }

 private:
  bool HasNonTrivial3DComponent() const override { return z_ != 1.0; }

  void CommonPrimitiveForInterpolation(
      const TransformOperation* from,
      TransformOperation::OperationType& common_type) const;

  TransformOperation* Zoom(double factor) final { return this; }

  bool PreservesAxisAlignment() const final { return true; }
  bool IsIdentityOrTranslation() const final {
    return x_ == 1.0 && y_ == 1.0 && z_ == 1.0;
  }

  double x_;
  double y_;
  double z_;
  OperationType type_;
};

template <>
struct DowncastTraits<ScaleTransformOperation> {
  static bool AllowFrom(const TransformOperation& transform) {
    return ScaleTransformOperation::IsMatchingOperationType(
        transform.GetType());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_SCALE_TRANSFORM_OPERATION_H_
