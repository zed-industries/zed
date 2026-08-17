/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_INTERPOLATED_TRANSFORM_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_INTERPOLATED_TRANSFORM_OPERATION_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/platform/transforms/transform_operation.h"
#include "third_party/blink/renderer/platform/transforms/transform_operations.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// This class is an implementation detail for deferred interpolations.
class PLATFORM_EXPORT InterpolatedTransformOperation final
    : public TransformOperation {
 public:
  InterpolatedTransformOperation(const TransformOperations& from,
                                 const TransformOperations& to,
                                 int starting_index,
                                 double progress)
      : from_(from),
        to_(to),
        starting_index_(starting_index),
        progress_(progress) {
    // This should only be generated during interpolation when it is impossible
    // to create a Matrix3DTransformOperation due to layout-dependence.
    DCHECK(BoxSizeDependencies());
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(from_);
    visitor->Trace(to_);
    TransformOperation::Trace(visitor);
  }

 protected:
  bool IsEqualAssumingSameType(const TransformOperation&) const override;

 private:
  OperationType GetType() const override { return kInterpolated; }

  void Apply(gfx::Transform&, const gfx::SizeF& border_box_size) const override;

  TransformOperation* Accumulate(const TransformOperation&) override {
    NOTREACHED();
  }

  TransformOperation* Blend(const TransformOperation* from,
                            double progress,
                            bool blend_to_identity = false) override;
  TransformOperation* Zoom(double factor) final {
    return MakeGarbageCollected<InterpolatedTransformOperation>(
        from_.Zoom(factor), to_.Zoom(factor), starting_index_, progress_);
  }

  bool PreservesAxisAlignment() const final {
    return from_.PreservesAxisAlignment() && to_.PreservesAxisAlignment();
  }
  bool IsIdentityOrTranslation() const final {
    return from_.IsIdentityOrTranslation() && to_.IsIdentityOrTranslation();
  }

  BoxSizeDependency BoxSizeDependencies() const override {
    return CombineDependencies(from_.BoxSizeDependencies(starting_index_),
                               to_.BoxSizeDependencies(starting_index_));
  }

  const TransformOperations from_;
  const TransformOperations to_;
  // Number of operations to skip from the start of each list. By spec,
  // pairwise interpolations are performed for compatible operations at the
  // start of the list and matrix interpolation for the remainder.
  int starting_index_;
  double progress_;
};

template <>
struct DowncastTraits<InterpolatedTransformOperation> {
  static bool AllowFrom(const TransformOperation& transform) {
    return transform.GetType() == TransformOperation::kInterpolated;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_INTERPOLATED_TRANSFORM_OPERATION_H_
