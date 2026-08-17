// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_LENGTH_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_LENGTH_INTERPOLATION_TYPE_H_

#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"
#include "third_party/blink/renderer/core/animation/length_property_functions.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class ComputedStyle;

class CORE_EXPORT CSSLengthInterpolationType : public CSSInterpolationType {
 public:
  explicit CSSLengthInterpolationType(PropertyHandle,
                                      const PropertyRegistration* = nullptr);

  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const final;
  InterpolationValue MaybeConvertCustomPropertyUnderlyingValue(
      const CSSValue&) const final;
  void Composite(UnderlyingValueOwner&,
                 double underlying_fraction,
                 const InterpolationValue&,
                 double interpolation_fraction) const final;
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const final;

 private:
  float EffectiveZoom(float effective_zoom) const {
    return is_zoomed_length_ ? effective_zoom : 1;
  }

  InterpolationValue MaybeConvertNeutral(const InterpolationValue& underlying,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertInitial(const StyleResolverState&,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertInherit(const StyleResolverState&,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertValue(const CSSValue&,
                                       const StyleResolverState&,
                                       ConversionCheckers&) const final;

  InterpolationValue PreInterpolationCompositeIfNeeded(
      InterpolationValue value,
      const InterpolationValue& underlying,
      EffectModel::CompositeOperation,
      ConversionCheckers&) const override;

  InterpolationValue MaybeConvertUnderlyingValue(
      const CSSInterpolationEnvironment&) const final;

  PairwiseInterpolationValue MaybeMergeSingles(
      InterpolationValue&& start,
      InterpolationValue&& end) const final;

  const CSSValue* CreateCSSValue(const InterpolableValue&,
                                 const NonInterpolableValue*,
                                 const StyleResolverState&) const final;

  const Length::ValueRange value_range_;
  bool is_zoomed_length_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_LENGTH_INTERPOLATION_TYPE_H_
