// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_PAIRWISE_INTERPOLATION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_PAIRWISE_INTERPOLATION_VALUE_H_

#include <memory>
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/animation/non_interpolable_value.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Represents the smooth interpolation between an adjacent pair of
// PropertySpecificKeyframes.
struct PairwiseInterpolationValue {
  DISALLOW_NEW();

  PairwiseInterpolationValue(
      InterpolableValue* start_interpolable_value,
      InterpolableValue* end_interpolable_value,
      const NonInterpolableValue* non_interpolable_value = nullptr)
      : start_interpolable_value(start_interpolable_value),
        end_interpolable_value(end_interpolable_value),
        non_interpolable_value(non_interpolable_value) {}

  PairwiseInterpolationValue(std::nullptr_t) {}

  PairwiseInterpolationValue(PairwiseInterpolationValue&& other)
      : start_interpolable_value(other.start_interpolable_value),
        end_interpolable_value(other.end_interpolable_value),
        non_interpolable_value(std::move(other.non_interpolable_value)) {}

  operator bool() const { return start_interpolable_value.Get(); }

  void Trace(Visitor* v) const {
    v->Trace(start_interpolable_value);
    v->Trace(end_interpolable_value);
    v->Trace(non_interpolable_value);
  }

  Member<InterpolableValue> start_interpolable_value;
  Member<InterpolableValue> end_interpolable_value;
  Member<const NonInterpolableValue> non_interpolable_value;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_PAIRWISE_INTERPOLATION_VALUE_H_
