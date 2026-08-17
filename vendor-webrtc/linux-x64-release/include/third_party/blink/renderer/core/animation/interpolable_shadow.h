// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_SHADOW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_SHADOW_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/interpolable_color.h"
#include "third_party/blink/renderer/core/animation/interpolable_length.h"
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/animation/interpolation_value.h"
#include "third_party/blink/renderer/core/animation/pairwise_interpolation_value.h"
#include "third_party/blink/renderer/core/style/shadow_data.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSValue;
class StyleResolverState;
class UnderlyingValue;

// Represents a CSS shadow value (such as [0] or [1]), converted into a form
// that can be interpolated from/to.
//
// [0]: https://drafts.csswg.org/css-backgrounds-3/#typedef-shadow
// [1]: https://drafts.fxtf.org/filter-effects/#funcdef-filter-drop-shadow
class InterpolableShadow : public InterpolableValue {
 public:
  InterpolableShadow(InterpolableLength* x,
                     InterpolableLength* y,
                     InterpolableLength* blur,
                     InterpolableLength* spread,
                     InterpolableColor* color,
                     ShadowStyle);

  static InterpolableShadow* Create(const ShadowData&,
                                    double zoom,
                                    mojom::blink::ColorScheme color_scheme,
                                    const ui::ColorProvider* color_provider);
  static InterpolableShadow* CreateNeutral();

  static InterpolableShadow* MaybeConvertCSSValue(const CSSValue&,
                                                  const StyleResolverState&);

  // Helpers for CSSListInterpolationFunctions.
  static PairwiseInterpolationValue MaybeMergeSingles(InterpolableValue* start,
                                                      InterpolableValue* end);
  static bool CompatibleForCompositing(const InterpolableValue*,
                                       const InterpolableValue*);
  static void Composite(UnderlyingValue&,
                        double underlying_fraction,
                        const InterpolableValue&,
                        const NonInterpolableValue*);
  ShadowStyle GetShadowStyle() const { return shadow_style_; }
  // Convert this InterpolableShadow back into a ShadowData class, usually to be
  // applied to the style after interpolating it.
  ShadowData CreateShadowData(const StyleResolverState&) const;

  // InterpolableValue implementation:
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsShadow() const final { return true; }
  bool Equals(const InterpolableValue& other) const final { NOTREACHED(); }
  void Scale(double scale) final;
  void Add(const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  void Trace(Visitor* v) const override {
    InterpolableValue::Trace(v);
    v->Trace(x_);
    v->Trace(y_);
    v->Trace(blur_);
    v->Trace(spread_);
    v->Trace(color_);
  }

 private:
  InterpolableShadow* RawClone() const final;
  InterpolableShadow* RawCloneAndZero() const final;

  // The interpolable components of a shadow. These should all be non-null.
  Member<InterpolableLength> x_;
  Member<InterpolableLength> y_;
  Member<InterpolableLength> blur_;
  Member<InterpolableLength> spread_;
  // TODO(crbug.com/1500708): Handle unresolved-color-mix.
  Member<InterpolableColor> color_;

  ShadowStyle shadow_style_;
};

template <>
struct DowncastTraits<InterpolableShadow> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsShadow();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_SHADOW_H_
