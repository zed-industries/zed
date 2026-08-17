// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_NUMBER_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_NUMBER_INTERPOLATION_TYPE_H_

#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class CORE_EXPORT CSSNumberInterpolationType : public CSSInterpolationType {
 public:
  explicit CSSNumberInterpolationType(
      PropertyHandle property,
      const PropertyRegistration* registration = nullptr,
      bool round_to_integer = false)
      : CSSInterpolationType(property, registration),
        round_to_integer_(round_to_integer) {
    // This integer flag only applies to registered custom properties.
    DCHECK(!round_to_integer_ || property.IsCSSCustomProperty());
  }

  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const final;
  InterpolationValue MaybeConvertCustomPropertyUnderlyingValue(
      const CSSValue&) const final;
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const final;

  const CSSValue* CreateCSSValue(const InterpolableValue&,
                                 const NonInterpolableValue*,
                                 const StyleResolverState&) const final;

 private:
  InterpolationValue CreateNumberValue(double number) const;
  InterpolationValue MaybeConvertNeutral(const InterpolationValue& underlying,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertInitial(const StyleResolverState&,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertInherit(const StyleResolverState&,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertValue(const CSSValue&,
                                       const StyleResolverState&,
                                       ConversionCheckers&) const final;

  CSSPrimitiveValue::UnitType UnitType() const {
    return round_to_integer_ ? CSSPrimitiveValue::UnitType::kInteger
                             : CSSPrimitiveValue::UnitType::kNumber;
  }

  const bool round_to_integer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_NUMBER_INTERPOLATION_TYPE_H_
