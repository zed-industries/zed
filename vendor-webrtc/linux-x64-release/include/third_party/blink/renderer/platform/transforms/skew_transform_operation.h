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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_SKEW_TRANSFORM_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_SKEW_TRANSFORM_OPERATION_H_

#include "third_party/blink/renderer/platform/transforms/transform_operation.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class PLATFORM_EXPORT SkewTransformOperation final : public TransformOperation {
 public:
  SkewTransformOperation(double angle_x, double angle_y, OperationType type)
      : angle_x_(angle_x), angle_y_(angle_y), type_(type) {}

  double AngleX() const { return angle_x_; }
  double AngleY() const { return angle_y_; }

  static bool IsMatchingOperationType(OperationType type) {
    return type == kSkewX || type == kSkewY || type == kSkew;
  }

 protected:
  bool IsEqualAssumingSameType(const TransformOperation& o) const override {
    const SkewTransformOperation* s =
        static_cast<const SkewTransformOperation*>(&o);
    return angle_x_ == s->angle_x_ && angle_y_ == s->angle_y_;
  }

 private:
  OperationType GetType() const override { return type_; }

  void Apply(gfx::Transform& transform, const gfx::SizeF&) const override {
    transform.Skew(angle_x_, angle_y_);
  }

  TransformOperation* Accumulate(const TransformOperation& other) override;
  TransformOperation* Blend(const TransformOperation* from,
                            double progress,
                            bool blend_to_identity = false) override;
  TransformOperation* Zoom(double factor) final { return this; }

  double angle_x_;
  double angle_y_;
  OperationType type_;
};

template <>
struct DowncastTraits<SkewTransformOperation> {
  static bool AllowFrom(const TransformOperation& transform) {
    return SkewTransformOperation::IsMatchingOperationType(transform.GetType());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_SKEW_TRANSFORM_OPERATION_H_
