// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_FONT_STYLE_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_FONT_STYLE_INTERPOLATION_TYPE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"

namespace blink {

// This class performs validation and constructs InterpolationValues for
// font-style animation. Font-style property should be animated by computed
// value, i.e. if there is an 'oblique x deg' value, it should be interpolated
// by the angle value x. 'normal' animates as 'oblique 0deg', animating from/to
// 'italic' keyword should be discrete, see:
// https://www.w3.org/TR/css-fonts-4/#font-weight-absolute-values
// TODO(https://crbug.com/1404731): 'CSS transition' for 'italic' keyword
// currently works as 'oblique 14deg' since there is no way to distinguish
// 'italic' and 'oblique' keywords in ComputedStyle.
class CSSFontStyleInterpolationType : public CSSInterpolationType {
 public:
  explicit CSSFontStyleInterpolationType(PropertyHandle property)
      : CSSInterpolationType(property) {
    DCHECK_EQ(CssProperty().PropertyID(), CSSPropertyID::kFontStyle);
  }

  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const final;
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const final;

 private:
  InterpolationValue CreateFontStyleValue(FontSelectionValue) const;
  InterpolationValue MaybeConvertNeutral(const InterpolationValue& underlying,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertInitial(const StyleResolverState&,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertInherit(const StyleResolverState&,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertValue(const CSSValue&,
                                       const StyleResolverState&,
                                       ConversionCheckers&) const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_FONT_STYLE_INTERPOLATION_TYPE_H_
