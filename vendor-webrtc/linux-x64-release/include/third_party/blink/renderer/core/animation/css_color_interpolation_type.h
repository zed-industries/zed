// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COLOR_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COLOR_INTERPOLATION_TYPE_H_

#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"
#include "third_party/blink/renderer/core/animation/interpolable_color.h"
#include "third_party/blink/renderer/core/animation/interpolable_style_color.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/graphics/color.h"

namespace blink {

class OptionalStyleColor;
class StyleColor;

class CORE_EXPORT CSSColorInterpolationType : public CSSInterpolationType {
 public:
  explicit CSSColorInterpolationType(
      PropertyHandle property,
      const PropertyRegistration* registration = nullptr)
      : CSSInterpolationType(property, registration) {}

  InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const final;
  InterpolationValue MaybeConvertCustomPropertyUnderlyingValue(
      const CSSValue&) const final;
  void ApplyStandardPropertyValue(const InterpolableValue&,
                                  const NonInterpolableValue*,
                                  StyleResolverState&) const final;
  void Composite(UnderlyingValueOwner& underlying_value_owner,
                 double underlying_fraction,
                 const InterpolationValue& value,
                 double interpolation_fraction) const final;

  static void EnsureInterpolableStyleColor(InterpolableList& list,
                                           wtf_size_t index);
  static void EnsureCompatibleInterpolableColorTypes(InterpolableList& list_a,
                                                     InterpolableList& list_b);

  static InterpolableColor* CreateInterpolableColor(const Color&);
  static InterpolableColor* CreateInterpolableColor(
      CSSValueID,
      mojom::blink::ColorScheme color_scheme,
      const ui::ColorProvider* color_provider);
  static InterpolableColor* CreateInterpolableColor(
      const StyleColor&,
      mojom::blink::ColorScheme color_scheme,
      const ui::ColorProvider* color_provider);
  static InterpolableColor* MaybeCreateInterpolableColor(
      const CSSValue&,
      const StyleResolverState*);

  static BaseInterpolableColor* CreateBaseInterpolableColor(
      const StyleColor&,
      mojom::blink::ColorScheme color_scheme,
      const ui::ColorProvider* color_provider);

  static Color ResolveInterpolableColor(
      const InterpolableValue& interpolable_color,
      const StyleResolverState&,
      bool is_visited = false,
      bool is_text_decoration = false);

  static Color GetColor(const InterpolableValue&);

  // This method confirms that the two colors are in the same colorspace for
  // interpolation and converts them if necessary.
  PairwiseInterpolationValue MaybeMergeSingles(
      InterpolationValue&& start,
      InterpolationValue&& end) const final;

  static bool IsNonKeywordColor(const InterpolableValue&);

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
  static InterpolationValue ConvertStyleColorPair(
      const OptionalStyleColor&,
      const OptionalStyleColor&,
      mojom::blink::ColorScheme color_scheme,
      const ui::ColorProvider* color_provider);
  static InterpolationValue ConvertStyleColorPair(
      const StyleColor& unvisited_color,
      const StyleColor& visited_color,
      mojom::blink::ColorScheme color_scheme,
      const ui::ColorProvider* color_provider);

  const CSSValue* CreateCSSValue(const InterpolableValue&,
                                 const NonInterpolableValue*,
                                 const StyleResolverState&) const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COLOR_INTERPOLATION_TYPE_H_
