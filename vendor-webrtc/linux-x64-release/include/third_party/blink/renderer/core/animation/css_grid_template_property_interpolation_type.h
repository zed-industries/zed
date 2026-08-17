// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_GRID_TEMPLATE_PROPERTY_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_GRID_TEMPLATE_PROPERTY_INTERPOLATION_TYPE_H_

#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"

namespace blink {

class CSSProperty;

class CSSGridTemplatePropertyInterpolationType : public CSSInterpolationType {
 public:
  explicit CSSGridTemplatePropertyInterpolationType(PropertyHandle property)
      : CSSInterpolationType(property) {
    property_id_ = property.GetCSSProperty().PropertyID();
    DCHECK(property_id_ == CSSPropertyID::kGridTemplateColumns ||
           property_id_ == CSSPropertyID::kGridTemplateRows);
  }

  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const final;
  PairwiseInterpolationValue MaybeMergeSingles(
      InterpolationValue&& start,
      InterpolationValue&& end) const final;
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const final;
  void Composite(UnderlyingValueOwner& underlying_value_owner,
                 double underlying_fraction,
                 const InterpolationValue& value,
                 double interpolation_fraction) const final;

  static InterpolableValue* CreateInterpolableGridTrackList(
      const NGGridTrackList& track_list,
      const CSSProperty& property,
      float zoom);

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

  CSSPropertyID property_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_GRID_TEMPLATE_PROPERTY_INTERPOLATION_TYPE_H_
