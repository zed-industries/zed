// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_TIME_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_TIME_INTERPOLATION_TYPE_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"

namespace blink {

class CSSTimeInterpolationType : public CSSInterpolationType {
 public:
  explicit CSSTimeInterpolationType(
      PropertyHandle property,
      const PropertyRegistration* registration = nullptr)
      : CSSInterpolationType(property, registration) {}

  InterpolationValue MaybeConvertNeutral(const InterpolationValue& underlying,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertValue(const CSSValue&,
                                       const StyleResolverState&,
                                       ConversionCheckers&) const final;

  const CSSValue* CreateCSSValue(const InterpolableValue&,
                                 const NonInterpolableValue*,
                                 const StyleResolverState&) const final;

  static std::optional<double> GetSeconds(const CSSPropertyID& property,
                                          const ComputedStyle& style);

 private:
  InterpolationValue CreateTimeValue(double) const;
  std::optional<double> GetSeconds(const ComputedStyle& style) const;
  double ClampTime(const CSSPropertyID& property, double value) const;
  InterpolationValue MaybeConvertTime(const CSSValue&,
                                      const CSSToLengthConversionData&) const;
  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const final;
  InterpolationValue MaybeConvertCustomPropertyUnderlyingValue(
      const CSSValue&) const final;
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const final;
  InterpolationValue MaybeConvertInitial(const StyleResolverState&,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertInherit(const StyleResolverState&,
                                         ConversionCheckers&) const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_TIME_INTERPOLATION_TYPE_H_
