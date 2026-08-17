// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_TRANSFORM_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_TRANSFORM_LIST_H_

#include <memory>

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/transforms/transform_operations.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSToLengthConversionData;
class CSSValue;

// Represents a blink::TransformOperations, converted into a form that can be
// interpolated from/to.
class CORE_EXPORT InterpolableTransformList final : public InterpolableValue {
 public:
  InterpolableTransformList(
      TransformOperations&& operations,
      TransformOperations::BoxSizeDependentMatrixBlending box_size_dependent)
      : operations_(std::move(operations)),
        box_size_dependent_(box_size_dependent) {}

  static InterpolableTransformList* ConvertCSSValue(
      const CSSValue&,
      const CSSToLengthConversionData&,
      TransformOperations::BoxSizeDependentMatrixBlending);

  // Return the underlying TransformOperations. Usually called after composition
  // and interpolation, to apply the results back to the style.
  TransformOperations operations() const { return operations_; }

  void PreConcat(const InterpolableTransformList& underlying);
  void AccumulateOnto(const InterpolableTransformList& underlying);

  // InterpolableValue implementation:
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsTransformList() const final { return true; }
  bool Equals(const InterpolableValue& other) const final { NOTREACHED(); }
  void Scale(double scale) final { NOTREACHED(); }
  void Add(const InterpolableValue& other) final { NOTREACHED(); }
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  void Trace(Visitor* v) const override {
    InterpolableValue::Trace(v);
    v->Trace(operations_);
  }

 private:
  InterpolableTransformList* RawClone() const final {
    return MakeGarbageCollected<InterpolableTransformList>(
        TransformOperations(operations_), box_size_dependent_);
  }
  InterpolableTransformList* RawCloneAndZero() const final { NOTREACHED(); }

  TransformOperations operations_;
  TransformOperations::BoxSizeDependentMatrixBlending box_size_dependent_;
};

template <>
struct DowncastTraits<InterpolableTransformList> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsTransformList();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_TRANSFORM_LIST_H_
