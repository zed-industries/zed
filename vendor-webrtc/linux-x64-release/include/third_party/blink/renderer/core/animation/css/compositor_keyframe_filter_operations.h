// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_FILTER_OPERATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_FILTER_OPERATIONS_H_

#include "third_party/blink/renderer/core/animation/css/compositor_keyframe_value.h"
#include "third_party/blink/renderer/core/style/filter_operations.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class CompositorKeyframeFilterOperations final
    : public CompositorKeyframeValue {
 public:
  CompositorKeyframeFilterOperations(const FilterOperations& operations)
      : operation_wrapper_(
            MakeGarbageCollected<FilterOperationsWrapper>(operations)) {}
  ~CompositorKeyframeFilterOperations() override = default;

  const FilterOperations& Operations() const {
    return operation_wrapper_->Operations();
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(operation_wrapper_);
    CompositorKeyframeValue::Trace(visitor);
  }

 private:
  Type GetType() const override { return Type::kFilterOperations; }

  Member<FilterOperationsWrapper> operation_wrapper_;
};

template <>
struct DowncastTraits<CompositorKeyframeFilterOperations> {
  static bool AllowFrom(const CompositorKeyframeValue& value) {
    return value.IsFilterOperations();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_FILTER_OPERATIONS_H_
