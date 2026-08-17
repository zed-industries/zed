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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_TRANSLATE_TRANSFORM_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_TRANSLATE_TRANSFORM_OPERATION_H_

#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/geometry/length_functions.h"
#include "third_party/blink/renderer/platform/transforms/transform_operation.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class PLATFORM_EXPORT TranslateTransformOperation final
    : public TransformOperation {
 public:
  TranslateTransformOperation(const Length& tx,
                              const Length& ty,
                              double tz,
                              OperationType type)
      : x_(tx), y_(ty), z_(tz), type_(type) {
    DCHECK(IsMatchingOperationType(type));
  }

  TranslateTransformOperation(const Length& tx,
                              const Length& ty,
                              OperationType type)
      : TranslateTransformOperation(tx, ty, 0, type) {}

  BoxSizeDependency BoxSizeDependencies() const override {
    return CombineDependencies(
        (x_.HasPercent() ? kDependsWidth : kDependsNone),
        (y_.HasPercent() ? kDependsHeight : kDependsNone));
  }

  double X(const gfx::SizeF& border_box_size) const {
    return FloatValueForLength(x_, border_box_size.width());
  }
  double Y(const gfx::SizeF& border_box_size) const {
    return FloatValueForLength(y_, border_box_size.height());
  }

  const Length& X() const { return x_; }
  const Length& Y() const { return y_; }
  double Z() const { return z_; }

  void Apply(gfx::Transform& transform,
             const gfx::SizeF& border_box_size) const override {
    transform.Translate3d(X(border_box_size), Y(border_box_size), Z());
  }

  static bool IsMatchingOperationType(OperationType type) {
    return type == kTranslate || type == kTranslateX || type == kTranslateY ||
           type == kTranslateZ || type == kTranslate3D;
  }

  TranslateTransformOperation* ZoomTranslate(double factor);

  OperationType GetType() const override { return type_; }
  OperationType PrimitiveType() const final { return kTranslate3D; }

 protected:
  bool IsEqualAssumingSameType(const TransformOperation& o) const override {
    const TranslateTransformOperation* t =
        static_cast<const TranslateTransformOperation*>(&o);
    return x_ == t->x_ && y_ == t->y_ && z_ == t->z_;
  }

 private:
  TransformOperation* Accumulate(const TransformOperation& other) override;
  TransformOperation* Blend(const TransformOperation* from,
                            double progress,
                            bool blend_to_identity = false) override;
  TransformOperation* Zoom(double factor) final {
    return ZoomTranslate(factor);
  }

  bool PreservesAxisAlignment() const final { return true; }
  bool IsIdentityOrTranslation() const final { return true; }

  void CommonPrimitiveForInterpolation(
      const TransformOperation* from,
      TransformOperation::OperationType& common_type) const;

  bool HasNonTrivial3DComponent() const override { return z_ != 0.0; }

  Length x_;
  Length y_;
  double z_;
  OperationType type_;
};

template <>
struct DowncastTraits<TranslateTransformOperation> {
  static bool AllowFrom(const TransformOperation& transform) {
    return TranslateTransformOperation::IsMatchingOperationType(
        transform.GetType());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_TRANSLATE_TRANSFORM_OPERATION_H_
