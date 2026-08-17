// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_UNDERLYING_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_UNDERLYING_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class InterpolableValue;
class NonInterpolableValue;

// Represents the 'underlying value' modified by an 'effect stack'.
//
// This is a virtual class because the representation of the underlying
// value may vary depending on circumstance. See UnderlyingValueOwner,
// and UnderlyingItemValue.
//
// https://drafts.csswg.org/web-animations-1/#underlying-value
// https://drafts.csswg.org/web-animations-1/#effect-stack
class CORE_EXPORT UnderlyingValue {
 public:
  virtual InterpolableValue& MutableInterpolableValue() = 0;

  virtual void SetInterpolableValue(InterpolableValue*) = 0;

  virtual const NonInterpolableValue* GetNonInterpolableValue() const = 0;

  // The NonInterpolableValue part of the underlying value may not be mutated,
  // hence there is no MutableNonInterpolableValue function. However, the
  // NonInterpolableValue part may be replaced entirely with this function.
  virtual void SetNonInterpolableValue(const NonInterpolableValue*) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_UNDERLYING_VALUE_H_
