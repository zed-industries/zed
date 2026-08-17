// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_TYPE_H_

#include <memory>

#include "third_party/blink/renderer/core/animation/interpolation_value.h"
#include "third_party/blink/renderer/core/animation/keyframe.h"
#include "third_party/blink/renderer/core/animation/pairwise_interpolation_value.h"
#include "third_party/blink/renderer/core/animation/primitive_interpolation.h"
#include "third_party/blink/renderer/core/animation/property_handle.h"
#include "third_party/blink/renderer/core/animation/underlying_value_owner.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CSSInterpolationEnvironment;

// Subclasses of InterpolationType implement the logic for a specific value type
// of a specific PropertyHandle to:
// - Convert PropertySpecificKeyframe values to (Pairwise)?InterpolationValues:
// maybeConvertPairwise() and maybeConvertSingle()
// - Convert the target Element's property value to an InterpolationValue:
// maybeConvertUnderlyingValue()
// - Apply an InterpolationValue to a target Element's property: apply().
class CORE_EXPORT InterpolationType {
  USING_FAST_MALLOC(InterpolationType);

 public:
  InterpolationType(const InterpolationType&) = delete;
  InterpolationType& operator=(const InterpolationType&) = delete;
  virtual ~InterpolationType() = default;

  PropertyHandle GetProperty() const { return property_; }

  // ConversionCheckers are returned from calls to maybeConvertPairwise() and
  // maybeConvertSingle() to enable the caller to check whether the result is
  // still valid given changes in the CSSInterpolationEnvironment and underlying
  // InterpolationValue.
  class ConversionChecker : public GarbageCollected<ConversionChecker> {
   public:
    ConversionChecker(const ConversionChecker&) = delete;
    ConversionChecker& operator=(const ConversionChecker&) = delete;
    virtual ~ConversionChecker() = default;
    virtual void Trace(Visitor*) const {}

    void SetType(const InterpolationType& type) { type_ = &type; }
    const InterpolationType& GetType() const { return *type_; }
    virtual bool IsValid(const CSSInterpolationEnvironment&,
                         const InterpolationValue& underlying) const = 0;

   protected:
    ConversionChecker() : type_(nullptr) {}
    const InterpolationType* type_;
  };
  using ConversionCheckers = HeapVector<Member<ConversionChecker>>;

  virtual PairwiseInterpolationValue MaybeConvertPairwise(
      const PropertySpecificKeyframe& start_keyframe,
      const PropertySpecificKeyframe& end_keyframe,
      const CSSInterpolationEnvironment& environment,
      const InterpolationValue& underlying,
      ConversionCheckers& conversion_checkers) const {
    InterpolationValue start = MaybeConvertSingle(
        start_keyframe, environment, underlying, conversion_checkers);
    if (!start)
      return nullptr;
    InterpolationValue end = MaybeConvertSingle(
        end_keyframe, environment, underlying, conversion_checkers);
    if (!end)
      return nullptr;
    return MaybeMergeSingles(std::move(start), std::move(end));
  }

  virtual InterpolationValue MaybeConvertSingle(
      const PropertySpecificKeyframe&,
      const CSSInterpolationEnvironment&,
      const InterpolationValue& underlying,
      ConversionCheckers&) const = 0;

  virtual PairwiseInterpolationValue MaybeMergeSingles(
      InterpolationValue&& start,
      InterpolationValue&& end) const {
    DCHECK(!start.non_interpolable_value);
    DCHECK(!end.non_interpolable_value);
    return PairwiseInterpolationValue(std::move(start.interpolable_value),
                                      std::move(end.interpolable_value),
                                      nullptr);
  }

  virtual InterpolationValue MaybeConvertUnderlyingValue(
      const CSSInterpolationEnvironment&) const = 0;

  virtual void Composite(UnderlyingValueOwner& underlying_value_owner,
                         double underlying_fraction,
                         const InterpolationValue& value,
                         double interpolation_fraction) const {
    DCHECK(!underlying_value_owner.Value().non_interpolable_value);
    DCHECK(!value.non_interpolable_value);
    underlying_value_owner.MutableValue().interpolable_value->ScaleAndAdd(
        underlying_fraction, *value.interpolable_value);
  }

  virtual void Apply(const InterpolableValue&,
                     const NonInterpolableValue*,
                     CSSInterpolationEnvironment&) const = 0;

  // If this returns true, then transition-behavior:allow-discrete must be set
  // in order to use this InterpolationType. Discrete properties generally don't
  // have an InterpolationType set because there is nothing to interpolate, but
  // some of them do in order to flip at the beginning or end of the animation
  // instead of in the middle.
  virtual bool IsDiscrete() const { return false; }

  // Implement reference equality checking via pointer equality checking as
  // these are singletons.
  bool operator==(const InterpolationType& other) const {
    return this == &other;
  }
  bool operator!=(const InterpolationType& other) const {
    return this != &other;
  }

 protected:
  explicit InterpolationType(PropertyHandle property) : property_(property) {}

  const PropertyHandle property_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_TYPE_H_
