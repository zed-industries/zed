// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INVALIDATABLE_INTERPOLATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INVALIDATABLE_INTERPOLATION_H_

#include <memory>
#include "third_party/blink/renderer/core/animation/interpolation.h"
#include "third_party/blink/renderer/core/animation/interpolation_type.h"
#include "third_party/blink/renderer/core/animation/interpolation_types_map.h"
#include "third_party/blink/renderer/core/animation/primitive_interpolation.h"
#include "third_party/blink/renderer/core/animation/typed_interpolation_value.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

// See the documentation of Interpolation for general information about this
// class hierarchy.
//
// The InvalidatableInterpolation subclass stores the start and end keyframes as
// PropertySpecificKeyframe objects.
//
// InvalidatableInterpolation uses conversion checkers and the interpolation
// environment to respond to changes to the underlying property value during
// interpolation.
//
// InvalidatableInterpolation is used to implement additive animations. During
// the effect application phase of animation computation, the current animated
// value of the property is applied to the element by calling the static
// ApplyStack function with an ordered list of InvalidatableInterpolation
// objects.
class CORE_EXPORT InvalidatableInterpolation : public Interpolation {
 public:
  InvalidatableInterpolation(const PropertyHandle& property,
                             PropertySpecificKeyframe* start_keyframe,
                             PropertySpecificKeyframe* end_keyframe)
      : Interpolation(),
        property_(property),
        interpolation_types_(nullptr),
        interpolation_types_version_(0),
        start_keyframe_(start_keyframe),
        end_keyframe_(end_keyframe),
        current_fraction_(std::numeric_limits<double>::quiet_NaN()),
        is_conversion_cached_(false) {}

  const PropertyHandle& GetProperty() const final { return property_; }
  void Interpolate(int iteration, double fraction) override;
  bool DependsOnUnderlyingValue() const final;
  static void ApplyStack(const ActiveInterpolations&,
                         CSSInterpolationEnvironment&);

  bool IsInvalidatableInterpolation() const override { return true; }

  const TypedInterpolationValue* GetCachedValueForTesting() const {
    return cached_value_.Get();
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(start_keyframe_);
    visitor->Trace(end_keyframe_);
    visitor->Trace(cached_pair_conversion_);
    visitor->Trace(conversion_checkers_);
    visitor->Trace(cached_value_);
    Interpolation::Trace(visitor);
  }

 private:
  using ConversionCheckers = InterpolationType::ConversionCheckers;

  TypedInterpolationValue* MaybeConvertUnderlyingValue(
      const CSSInterpolationEnvironment&) const;
  const TypedInterpolationValue* EnsureValidConversion(
      CSSInterpolationEnvironment&,
      const UnderlyingValueOwner&) const;
  void EnsureValidInterpolationTypes(CSSInterpolationEnvironment&) const;
  void ClearConversionCache(CSSInterpolationEnvironment& environment) const;
  bool IsConversionCacheValid(const CSSInterpolationEnvironment&,
                              const UnderlyingValueOwner&) const;
  bool IsNeutralKeyframeActive() const;
  PairwisePrimitiveInterpolation* MaybeConvertPairwise(
      const CSSInterpolationEnvironment&,
      const UnderlyingValueOwner&) const;
  TypedInterpolationValue* ConvertSingleKeyframe(
      const PropertySpecificKeyframe&,
      const CSSInterpolationEnvironment&,
      const UnderlyingValueOwner&) const;
  void AddConversionCheckers(const InterpolationType&,
                             ConversionCheckers&) const;
  void SetFlagIfInheritUsed(CSSInterpolationEnvironment&) const;
  double UnderlyingFraction() const;

  const PropertyHandle property_;
  mutable const InterpolationTypes* interpolation_types_;
  mutable size_t interpolation_types_version_;
  Member<PropertySpecificKeyframe> start_keyframe_;
  Member<PropertySpecificKeyframe> end_keyframe_;
  double current_fraction_;
  mutable bool is_conversion_cached_;
  mutable Member<PrimitiveInterpolation> cached_pair_conversion_;
  mutable ConversionCheckers conversion_checkers_;
  mutable Member<TypedInterpolationValue> cached_value_;
};

template <>
struct DowncastTraits<InvalidatableInterpolation> {
  static bool AllowFrom(const Interpolation& value) {
    return value.IsInvalidatableInterpolation();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INVALIDATABLE_INTERPOLATION_H_
