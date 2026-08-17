// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_UNDERLYING_VALUE_OWNER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_UNDERLYING_VALUE_OWNER_H_

#include <memory>

#include "base/check_op.h"
#include "third_party/blink/renderer/core/animation/typed_interpolation_value.h"
#include "third_party/blink/renderer/core/animation/underlying_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Handles memory management of underlying InterpolationValues in applyStack()
// Ensures we perform copy on write if we are not the owner of an underlying
// InterpolationValue. This functions similar to a DataRef except on
// std::unique_ptr'd objects.
class CORE_EXPORT UnderlyingValueOwner : public UnderlyingValue {
  STACK_ALLOCATED();

 public:
  UnderlyingValueOwner()
      : type_(nullptr), value_owner_(nullptr), value_(nullptr) {}
  UnderlyingValueOwner(const UnderlyingValueOwner&) = delete;
  UnderlyingValueOwner& operator=(const UnderlyingValueOwner&) = delete;

  operator bool() const {
    DCHECK_EQ(static_cast<bool>(type_), static_cast<bool>(value_));
    return type_;
  }

  // UnderlyingValue
  InterpolableValue& MutableInterpolableValue() final;
  void SetInterpolableValue(InterpolableValue*) final;
  const NonInterpolableValue* GetNonInterpolableValue() const final;
  void SetNonInterpolableValue(const NonInterpolableValue*) final;

  const InterpolationType& GetType() const {
    DCHECK(type_);
    return *type_;
  }

  const InterpolationValue& Value() const;

  void Set(std::nullptr_t);
  void Set(const InterpolationType&, const InterpolationValue&);
  void Set(const InterpolationType&, InterpolationValue&&);
  void Set(TypedInterpolationValue*);
  void Set(const TypedInterpolationValue*);

  InterpolationValue& MutableValue();

 private:
  const InterpolationType* type_;
  InterpolationValue value_owner_;
  const InterpolationValue* value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_UNDERLYING_VALUE_OWNER_H_
