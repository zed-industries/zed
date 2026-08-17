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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_PERSPECTIVE_TRANSFORM_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_PERSPECTIVE_TRANSFORM_OPERATION_H_

#include <algorithm>
#include <optional>

#include "third_party/blink/renderer/platform/transforms/transform_operation.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class PLATFORM_EXPORT PerspectiveTransformOperation final
    : public TransformOperation {
 public:
  explicit PerspectiveTransformOperation(std::optional<double> p) : p_(p) {}

  std::optional<double> Perspective() const { return p_; }

  double UsedPerspective() const {
    DCHECK(p_.has_value());
    return std::max(1.0, *p_);
  }

  double InverseUsedPerspective() const {
    if (!p_) {
      return 0.0;
    }
    return 1.0 / std::max(1.0, *p_);
  }

  static bool IsMatchingOperationType(OperationType type) {
    return type == kPerspective;
  }

 protected:
  bool IsEqualAssumingSameType(const TransformOperation& o) const override {
    const PerspectiveTransformOperation* p =
        static_cast<const PerspectiveTransformOperation*>(&o);
    return p_ == p->p_;
  }

 private:
  OperationType GetType() const override { return kPerspective; }

  void Apply(gfx::Transform& transform, const gfx::SizeF&) const override {
    if (Perspective()) {
      transform.ApplyPerspectiveDepth(UsedPerspective());
    }
  }

  TransformOperation* Accumulate(const TransformOperation& other) override;
  TransformOperation* Blend(const TransformOperation* from,
                            double progress,
                            bool blend_to_identity = false) override;
  TransformOperation* Zoom(double factor) final;

  // Perspective does not, by itself, specify a 3D transform.
  bool HasNonTrivial3DComponent() const override { return false; }

  // !p_.has_value() means the value is `none`, which is equivalent to
  // infinity.
  std::optional<double> p_;
};

template <>
struct DowncastTraits<PerspectiveTransformOperation> {
  static bool AllowFrom(const TransformOperation& transform) {
    return PerspectiveTransformOperation::IsMatchingOperationType(
        transform.GetType());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_PERSPECTIVE_TRANSFORM_OPERATION_H_
