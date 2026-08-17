// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_VAR_CYCLE_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_VAR_CYCLE_INTERPOLATION_TYPE_H_

#include "third_party/blink/renderer/core/animation/interpolation_type.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class PropertyRegistration;

// This is a special handler for custom property animation keyframes designed to
// detect and handle cyclic dependencies between animated custom properties
// through the use of var() in keyframes.
// Example:
// element.animate({
//   '--x': 'var(--y)',
//   '--y': 'var(--x)',
// }, 1000);
// This is an unresolvable dependency between custom property animations.
// This class detects when this scenario occurs and treats the custom property
// as invalid at computed-value time:
// https://drafts.csswg.org/css-variables/#invalid-at-computed-value-time
class CSSVarCycleInterpolationType : public InterpolationType {
 public:
  CSSVarCycleInterpolationType(const PropertyHandle&,
                               const PropertyRegistration&);

 private:
  InterpolationValue MaybeConvertSingle(const PropertySpecificKeyframe&,
                                        const CSSInterpolationEnvironment&,
                                        const InterpolationValue& underlying,
                                        ConversionCheckers&) const final;

  PairwiseInterpolationValue MaybeConvertPairwise(
      const PropertySpecificKeyframe& start_keyframe,
      const PropertySpecificKeyframe& end_keyframe,
      const CSSInterpolationEnvironment&,
      const InterpolationValue& underlying,
      ConversionCheckers&) const final;

  InterpolationValue MaybeConvertUnderlyingValue(
      const CSSInterpolationEnvironment&) const final;

  void Composite(UnderlyingValueOwner& underlying_value_owner,
                 double underlying_fraction,
                 const InterpolationValue& value,
                 double interpolation_fraction) const final {
    underlying_value_owner.Set(*this, value);
  }

  void Apply(const InterpolableValue&,
             const NonInterpolableValue*,
             CSSInterpolationEnvironment&) const final;

  WeakPersistent<const PropertyRegistration> registration_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_VAR_CYCLE_INTERPOLATION_TYPE_H_
