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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_MATRIX_TRANSFORM_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_MATRIX_TRANSFORM_OPERATION_H_

#include "third_party/blink/renderer/platform/transforms/transform_operation.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class PLATFORM_EXPORT MatrixTransformOperation final
    : public TransformOperation {
 public:
  explicit MatrixTransformOperation(const gfx::Transform& t) : matrix_(t) {
    DCHECK(t.Is2dTransform());
  }

  MatrixTransformOperation(double a,
                           double b,
                           double c,
                           double d,
                           double e,
                           double f)
      : matrix_(gfx::Transform::Affine(a, b, c, d, e, f)) {}

  const gfx::Transform& Matrix() const { return matrix_; }

  static bool IsMatchingOperationType(OperationType type) {
    return type == kMatrix;
  }

 protected:
  bool IsEqualAssumingSameType(const TransformOperation& o) const override {
    return matrix_ == static_cast<const MatrixTransformOperation&>(o).matrix_;
  }

 private:
  OperationType GetType() const override { return kMatrix; }

  void Apply(gfx::Transform& transform, const gfx::SizeF&) const override {
    transform.PreConcat(Matrix());
  }

  TransformOperation* Accumulate(const TransformOperation&) override;

  TransformOperation* Blend(const TransformOperation* from,
                            double progress,
                            bool blend_to_identity = false) override;
  TransformOperation* Zoom(double factor) final;

  bool PreservesAxisAlignment() const final {
    return Matrix().Preserves2dAxisAlignment();
  }
  bool IsIdentityOrTranslation() const final {
    return Matrix().IsIdentityOr2dTranslation();
  }

  // TODO(wangxianzhu): Use AffineTransform when we have Decompose2dTransform()
  // in ui/gfx/geometry/transform_utils.h.
  gfx::Transform matrix_;
};

template <>
struct DowncastTraits<MatrixTransformOperation> {
  static bool AllowFrom(const TransformOperation& transform) {
    return MatrixTransformOperation::IsMatchingOperationType(
        transform.GetType());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_MATRIX_TRANSFORM_OPERATION_H_
