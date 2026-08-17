/*
 * Copyright (C) 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_MATRIX_3D_TRANSFORM_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_MATRIX_3D_TRANSFORM_OPERATION_H_

#include "third_party/blink/renderer/platform/transforms/transform_operation.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class PLATFORM_EXPORT Matrix3DTransformOperation final
    : public TransformOperation {
 public:
  explicit Matrix3DTransformOperation(const gfx::Transform& matrix)
      : matrix_(matrix) {}

  gfx::Transform Matrix() const { return matrix_; }

  static bool IsMatchingOperationType(OperationType type) {
    return type == kMatrix3D;
  }

 protected:
  bool IsEqualAssumingSameType(const TransformOperation& o) const override {
    const Matrix3DTransformOperation* m =
        static_cast<const Matrix3DTransformOperation*>(&o);
    return matrix_ == m->matrix_;
  }

 private:
  OperationType GetType() const override { return kMatrix3D; }

  void Apply(gfx::Transform& transform, const gfx::SizeF&) const override {
    transform.PreConcat(matrix_);
  }

  TransformOperation* Accumulate(const TransformOperation& other) override;

  TransformOperation* Blend(const TransformOperation* from,
                            double progress,
                            bool blend_to_identity = false) override;
  TransformOperation* Zoom(double factor) final;

  bool PreservesAxisAlignment() const final {
    return matrix_.Preserves2dAxisAlignment();
  }
  bool IsIdentityOrTranslation() const final {
    return matrix_.IsIdentityOrTranslation();
  }

  gfx::Transform matrix_;
};

template <>
struct DowncastTraits<Matrix3DTransformOperation> {
  static bool AllowFrom(const TransformOperation& transform) {
    return Matrix3DTransformOperation::IsMatchingOperationType(
        transform.GetType());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_MATRIX_3D_TRANSFORM_OPERATION_H_
