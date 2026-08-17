// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TYPED_INTERPOLATION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TYPED_INTERPOLATION_VALUE_H_

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/animation/interpolation_value.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class InterpolationType;

// Represents an interpolated value between an adjacent pair of
// PropertySpecificKeyframes.
class TypedInterpolationValue
    : public GarbageCollected<TypedInterpolationValue> {
 public:
  TypedInterpolationValue(
      const InterpolationType& type,
      InterpolableValue* interpolable_value,
      const NonInterpolableValue* non_interpolable_value = nullptr)
      : type_(type), value_(interpolable_value, non_interpolable_value) {
    DCHECK(value_.interpolable_value);
  }

  TypedInterpolationValue* Clone() const {
    InterpolationValue copy = value_.Clone();
    return MakeGarbageCollected<TypedInterpolationValue>(
        type_, copy.interpolable_value, copy.non_interpolable_value);
  }

  const InterpolationType& GetType() const { return type_; }
  const InterpolableValue& GetInterpolableValue() const {
    return *value_.interpolable_value;
  }
  const NonInterpolableValue* GetNonInterpolableValue() const {
    return value_.non_interpolable_value.Get();
  }
  const InterpolationValue& Value() const { return value_; }

  InterpolationValue& MutableValue() { return value_; }

  void Trace(Visitor* v) const { v->Trace(value_); }

 private:
  const InterpolationType& type_;
  InterpolationValue value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TYPED_INTERPOLATION_VALUE_H_
