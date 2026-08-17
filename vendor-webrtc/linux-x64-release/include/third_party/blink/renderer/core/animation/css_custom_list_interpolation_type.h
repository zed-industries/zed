// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CUSTOM_LIST_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CUSTOM_LIST_INTERPOLATION_TYPE_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"

#include "third_party/blink/renderer/core/animation/list_interpolation_functions.h"
#include "third_party/blink/renderer/core/css/css_syntax_definition.h"

namespace blink {

class CSSCustomListInterpolationType : public CSSInterpolationType {
 public:
  CSSCustomListInterpolationType(
      PropertyHandle property,
      const PropertyRegistration* registration,
      std::unique_ptr<CSSInterpolationType> inner_interpolation_type,
      CSSSyntaxType syntax_type,
      CSSSyntaxRepeat syntax_repeat)
      : CSSInterpolationType(property, registration),
        inner_interpolation_type_(std::move(inner_interpolation_type)),
        syntax_repeat_(syntax_repeat) {
    DCHECK(property.IsCSSCustomProperty());
  }

  InterpolationValue MaybeConvertNeutral(const InterpolationValue& underlying,
                                         ConversionCheckers&) const final;
  InterpolationValue MaybeConvertValue(const CSSValue&,
                                       const StyleResolverState&,
                                       ConversionCheckers&) const final;
  InterpolationValue PreInterpolationCompositeIfNeeded(
      InterpolationValue value,
      const InterpolationValue& underlying,
      EffectModel::CompositeOperation,
      ConversionCheckers&) const override;
  const CSSValue* CreateCSSValue(const InterpolableValue&,
                                 const NonInterpolableValue*,
                                 const StyleResolverState&) const final;
  void Composite(UnderlyingValueOwner&,
                 double underlying_fraction,
                 const InterpolationValue&,
                 double interpolation_fraction) const final;
  PairwiseInterpolationValue MaybeMergeSingles(
      InterpolationValue&& start,
      InterpolationValue&& end) const final;
  InterpolationValue MaybeConvertCustomPropertyUnderlyingValue(
      const CSSValue&) const final;

 private:
  // These methods only apply to CSSInterpolationTypes used by standard CSS
  // properties. CSSCustomListInterpolationType is only accessible via
  // registered custom CSS properties.
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

  static bool NonInterpolableValuesAreCompatible(const NonInterpolableValue*,
                                                 const NonInterpolableValue*);

  // This InterpolationType represents the interpolation of elements inside
  // the list.
  //
  // TODO(andruud): Disallow applying inner interpolation type:
  //                https://crbug.com/882379
  //
  // Currently, InterpolationTypes are not designed to "nest" in this way due to
  // their mandatory association with specific properties, so please do not call
  // InterpolationType::Apply on inner_interpolation_type_.
  std::unique_ptr<CSSInterpolationType> inner_interpolation_type_;

  // TODO(crbug.com/981537, 981538, 981542): Add support for <image>,
  // <transform-function> and <transform-list> and make use of |syntax_type_|.
  // CSSSyntaxType syntax_type_;

  const CSSSyntaxRepeat syntax_repeat_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CUSTOM_LIST_INTERPOLATION_TYPE_H_
