// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CUSTOM_TRANSFORM_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CUSTOM_TRANSFORM_INTERPOLATION_TYPE_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/css_transform_interpolation_type.h"

namespace blink {

// Interpolation type for <transform-list> and <transform-function>+ for
// registered custom properties.
class CSSCustomTransformInterpolationType
    : public CSSTransformInterpolationType {
 public:
  CSSCustomTransformInterpolationType(PropertyHandle property,
                                      const PropertyRegistration* registration)
      : CSSTransformInterpolationType(property, registration) {
    CHECK(property.IsCSSCustomProperty());
  }

  InterpolationValue MaybeConvertNeutral(const InterpolationValue& underlying,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertValue(const CSSValue&,
                                       const StyleResolverState&,
                                       ConversionCheckers&) const final;

  const CSSValue* CreateCSSValue(const InterpolableValue&,
                                 const NonInterpolableValue*,
                                 const StyleResolverState&) const final;
  InterpolationValue MaybeConvertCustomPropertyUnderlyingValue(
      const CSSValue&) const final;

 private:
  InterpolationValue MaybeConvertTransformList(
      const CSSValue&,
      const CSSToLengthConversionData&) const;

  // These methods only apply to CSSInterpolationTypes used by standard CSS
  // properties.
  // CSSCustomTransformInterpolationType is only accessible via registered
  // custom CSS properties.
  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const final {
    NOTREACHED();
  }
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const final {
    NOTREACHED();
  }
  InterpolationValue MaybeConvertInitial(const StyleResolverState&,
                                         ConversionCheckers&) const final {
    NOTREACHED();
  }
  InterpolationValue MaybeConvertInherit(const StyleResolverState&,
                                         ConversionCheckers&) const final {
    NOTREACHED();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CUSTOM_TRANSFORM_INTERPOLATION_TYPE_H_
