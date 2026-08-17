// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_TRANSFORM_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_TRANSFORM_INTERPOLATION_TYPE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"

namespace blink {

inline bool IsTransformFunction(CSSValueID function_id) {
  switch (function_id) {
    case CSSValueID::kScale:
    case CSSValueID::kScaleX:
    case CSSValueID::kScaleY:
    case CSSValueID::kScaleZ:
    case CSSValueID::kScale3d:
    case CSSValueID::kTranslate:
    case CSSValueID::kTranslateX:
    case CSSValueID::kTranslateY:
    case CSSValueID::kTranslateZ:
    case CSSValueID::kTranslate3d:
    case CSSValueID::kRotate:
    case CSSValueID::kRotateX:
    case CSSValueID::kRotateY:
    case CSSValueID::kRotateZ:
    case CSSValueID::kRotate3d:
    case CSSValueID::kSkew:
    case CSSValueID::kSkewX:
    case CSSValueID::kSkewY:
    case CSSValueID::kMatrix:
    case CSSValueID::kMatrix3d:
    case CSSValueID::kPerspective:
      return true;
    default:
      return false;
  }
}

class CSSTransformInterpolationType : public CSSInterpolationType {
 public:
  explicit CSSTransformInterpolationType(PropertyHandle property)
      : CSSInterpolationType(property) {
    DCHECK_EQ(CssProperty().PropertyID(), CSSPropertyID::kTransform);
  }

  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const override;
  PairwiseInterpolationValue MaybeMergeSingles(
      InterpolationValue&& start,
      InterpolationValue&& end) const final;
  void Composite(UnderlyingValueOwner&,
                 double underlying_fraction,
                 const InterpolationValue&,
                 double interpolation_fraction) const final;
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const override;

 protected:
  CSSTransformInterpolationType(PropertyHandle property,
                                const PropertyRegistration* registration)
      : CSSInterpolationType(property, registration) {}

  InterpolationValue PreInterpolationCompositeIfNeeded(
      InterpolationValue value,
      const InterpolationValue& underlying,
      EffectModel::CompositeOperation,
      ConversionCheckers&) const override;

 private:
  InterpolationValue MaybeConvertNeutral(const InterpolationValue& underlying,
                                         ConversionCheckers&) const override;
  InterpolationValue MaybeConvertInitial(const StyleResolverState&,
                                         ConversionCheckers&) const override;
  InterpolationValue MaybeConvertInherit(const StyleResolverState&,
                                         ConversionCheckers&) const override;
  InterpolationValue MaybeConvertValue(const CSSValue&,
                                       const StyleResolverState&,
                                       ConversionCheckers&) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_TRANSFORM_INTERPOLATION_TYPE_H_
