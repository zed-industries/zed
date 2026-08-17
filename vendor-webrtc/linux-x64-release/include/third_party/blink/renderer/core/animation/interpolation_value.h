// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_VALUE_H_

#include <memory>
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/animation/non_interpolable_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Represents a (non-strict) subset of a PropertySpecificKeyframe's value broken
// down into interpolable and non-interpolable parts. InterpolationValues can be
// composed together to represent a whole PropertySpecificKeyframe value.
struct CORE_EXPORT InterpolationValue {
  DISALLOW_NEW();

  explicit InterpolationValue(
      InterpolableValue* interpolable_value,
      const NonInterpolableValue* non_interpolable_value = nullptr)
      : interpolable_value(interpolable_value),
        non_interpolable_value(non_interpolable_value) {}

  InterpolationValue(std::nullptr_t) {}

  InterpolationValue(InterpolationValue&& other)
      : interpolable_value(std::move(other.interpolable_value)),
        non_interpolable_value(std::move(other.non_interpolable_value)) {}

  void operator=(InterpolationValue&& other) {
    interpolable_value = std::move(other.interpolable_value);
    non_interpolable_value = std::move(other.non_interpolable_value);
  }

  operator bool() const { return interpolable_value.Get(); }

  InterpolationValue Clone() const {
    return InterpolationValue(
        interpolable_value ? interpolable_value->Clone() : nullptr,
        non_interpolable_value);
  }

  void Clear() {
    interpolable_value = nullptr;
    non_interpolable_value = nullptr;
  }

  void Trace(Visitor* v) const {
    v->Trace(interpolable_value);
    v->Trace(non_interpolable_value);
  }

  Member<InterpolableValue> interpolable_value;
  Member<const NonInterpolableValue> non_interpolable_value;
};

// Wrapper to be used with MakeGarbageCollected<>.
class InterpolationValueGCed : public GarbageCollected<InterpolationValueGCed> {
 public:
  explicit InterpolationValueGCed(const InterpolationValue& underlying)
      : underlying_(underlying.Clone()) {}

  void Trace(Visitor* v) const { v->Trace(underlying_); }

  InterpolationValue& underlying() { return underlying_; }
  const InterpolationValue& underlying() const { return underlying_; }

 private:
  InterpolationValue underlying_;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::InterpolationValue)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_VALUE_H_
