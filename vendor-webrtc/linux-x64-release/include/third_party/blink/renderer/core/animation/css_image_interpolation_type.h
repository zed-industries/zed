// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_IMAGE_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_IMAGE_INTERPOLATION_TYPE_H_

#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"

namespace blink {

class StyleImage;

class CSSImageInterpolationType : public CSSInterpolationType {
 public:
  explicit CSSImageInterpolationType(
      PropertyHandle property,
      const PropertyRegistration* registration = nullptr)
      : CSSInterpolationType(property, registration) {}

  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const final;
  void Composite(UnderlyingValueOwner&,
                 double underlying_fraction,
                 const InterpolationValue&,
                 double interpolation_fraction) const final;
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const final;

  static InterpolationValue MaybeConvertCSSValue(const CSSValue&,
                                                 bool accept_gradients);
  static InterpolationValue MaybeConvertStyleImage(const StyleImage&,
                                                   bool accept_gradients);
  static InterpolationValue MaybeConvertStyleImage(const StyleImage* image,
                                                   bool accept_gradients) {
    return image ? MaybeConvertStyleImage(*image, accept_gradients) : nullptr;
  }
  static PairwiseInterpolationValue StaticMergeSingleConversions(
      InterpolationValue&& start,
      InterpolationValue&& end);
  static StyleImage* ResolveStyleImage(const CSSProperty&,
                                       const InterpolableValue&,
                                       const NonInterpolableValue*,
                                       StyleResolverState&);
  static bool EqualNonInterpolableValues(const NonInterpolableValue*,
                                         const NonInterpolableValue*);

 private:
  InterpolationValue MaybeConvertNeutral(const InterpolationValue& underlying,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertInitial(const StyleResolverState&,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertInherit(const StyleResolverState&,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertValue(const CSSValue&,
                                       const StyleResolverState&,
                                       ConversionCheckers&) const final;

  PairwiseInterpolationValue MaybeMergeSingles(
      InterpolationValue&& start,
      InterpolationValue&& end) const final {
    return StaticMergeSingleConversions(std::move(start), std::move(end));
  }

  static const CSSValue* StaticCreateCSSValue(const InterpolableValue&,
                                              const NonInterpolableValue*,
                                              const CSSLengthResolver&);
  const CSSValue* CreateCSSValue(const InterpolableValue&,
                                 const NonInterpolableValue*,
                                 const StyleResolverState&) const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_IMAGE_INTERPOLATION_TYPE_H_
