// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_FILTER_LIST_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_FILTER_LIST_INTERPOLATION_TYPE_H_

#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"

namespace blink {

class CSSFilterListInterpolationType : public CSSInterpolationType {
 public:
  explicit CSSFilterListInterpolationType(PropertyHandle property)
      : CSSInterpolationType(property) {}

  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const final;
  PairwiseInterpolationValue MaybeMergeSingles(
      InterpolationValue&& start,
      InterpolationValue&& end) const final;
  void Composite(UnderlyingValueOwner&,
                 double underlying_fraction,
                 const InterpolationValue&,
                 double interpolation_fraction) const final;
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const final;
  InterpolationValue PreInterpolationCompositeIfNeeded(
      InterpolationValue value,
      const InterpolationValue& underlying,
      EffectModel::CompositeOperation,
      ConversionCheckers&) const final;

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

  // Helper methods to perform either additive or accumulative composition, as
  // defined in https://drafts.fxtf.org/filter-effects-1/#addition and
  // https://drafts.fxtf.org/filter-effects-1/#accumulation
  InterpolationValue PerformAdditiveComposition(
      InterpolableList* interpolable_list,
      const InterpolableList& underlying_list) const;
  InterpolationValue PerformAccumulativeComposition(
      InterpolableList* interpolable_list,
      const InterpolableList& underlying_list) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_FILTER_LIST_INTERPOLATION_TYPE_H_
