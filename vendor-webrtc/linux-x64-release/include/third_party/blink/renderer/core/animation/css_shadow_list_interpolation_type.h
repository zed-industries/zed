// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_SHADOW_LIST_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_SHADOW_LIST_INTERPOLATION_TYPE_H_

#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"

namespace blink {

class ShadowList;

class CSSShadowListInterpolationType : public CSSInterpolationType {
 public:
  explicit CSSShadowListInterpolationType(PropertyHandle property)
      : CSSInterpolationType(property) {}

  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const final;
  void Composite(UnderlyingValueOwner&,
                 double underlying_fraction,
                 const InterpolationValue&,
                 double interpolation_fraction) const final;
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const final;

 private:
  InterpolationValue ConvertShadowList(
      const ShadowList*,
      double zoom,
      mojom::blink::ColorScheme color_scheme,
      const ui::ColorProvider* color_provider) const;
  InterpolationValue CreateNeutralValue() const;

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
      InterpolationValue&& end) const final;
  InterpolationValue PreInterpolationCompositeIfNeeded(
      InterpolationValue value,
      const InterpolationValue& underlying,
      EffectModel::CompositeOperation,
      ConversionCheckers&) const final;
  InterpolationValue PerformAdditiveComposition(
      InterpolableList* interpolable_list,
      const InterpolationValue& underlying) const;
  InterpolationValue PerformAccumulativeComposition(
      InterpolableList* interpolable_list,
      const InterpolationValue& underlying) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_SHADOW_LIST_INTERPOLATION_TYPE_H_
