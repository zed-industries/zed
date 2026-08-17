// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_COLOR_H_

#include <memory>

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/base_interpolable_color.h"
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/graphics/color.h"

namespace ui {
class ColorProvider;
}  // namespace ui

namespace blink {

// InterpolableColors are created and manipulated by CSSColorInterpolationType.
// They store the three params and alpha from a blink::Color for interpolation,
// along with its color space. It is important that two colors are in the same
// color space when interpolating or the results will be incorrect. This is
// verified and adjusted in CSSColorInterpolationType::MaybeMergeSingles.
class CORE_EXPORT InterpolableColor final : public BaseInterpolableColor {
 public:
  InterpolableColor() {
    // All colors are zero-initialized (transparent black).
    static_assert(std::is_trivially_destructible_v<InterpolableColor>,
                  "Require trivial destruction for faster sweeping");
  }

  // Certain color keywords cannot be eagerly evaluated at specified value time.
  // For these keywords we store a separate entry in a list here, interpolate
  // that and evaluate the actual value of the color in
  // CSSColorInterpolationType::ResolveInterpolableColor. These entries
  // correspond to the color keywords that require this behavior.
  enum class ColorKeyword : unsigned {
    kCurrentcolor,
    kWebkitActivelink,
    kWebkitLink,
    kQuirkInherit,
  };

  static InterpolableColor* Create(Color color);
  static InterpolableColor* Create(ColorKeyword color_keyword);
  static InterpolableColor* Create(CSSValueID keyword,
                                   mojom::blink::ColorScheme color_scheme,
                                   const ui::ColorProvider* color_provider);

  Color GetColor() const;
  bool IsColor() const final { return true; }

  // Mutates the underlying colorspaces and values of "to" and "from" so that
  // they match for interpolation.
  static void SetupColorInterpolationSpaces(InterpolableColor& to,
                                            InterpolableColor& from);

  void Scale(double scale) final;
  void Add(const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  bool HasCurrentColorDependency() const final { return current_color_ != 0; }

  Color Resolve(const Color& current_color,
                const Color& active_link_color,
                const Color& link_color,
                const Color& text_color,
                mojom::blink::ColorScheme color_scheme) const final;

  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;

  bool IsKeywordColor() const;

  double Param0() const { return param0_; }
  double Param1() const { return param1_; }
  double Param2() const { return param2_; }
  double Alpha() const { return alpha_; }
  Color::ColorSpace ColorSpace() const { return color_space_; }

  double GetColorFraction(ColorKeyword keyword) const {
    switch (keyword) {
      case ColorKeyword::kCurrentcolor:
        return current_color_;
      case ColorKeyword::kWebkitActivelink:
        return webkit_active_link_;
      case ColorKeyword::kWebkitLink:
        return webkit_link_;
      case ColorKeyword::kQuirkInherit:
        return quirk_inherit_;
    }
  }

  InterpolableColor* Clone() const { return RawClone(); }

  InterpolableColor* CloneAndZero() const { return RawCloneAndZero(); }

  void Composite(const BaseInterpolableColor& other, double fraction) final;

  void Trace(Visitor* v) const final { BaseInterpolableColor::Trace(v); }

  InterpolableColor(double param0,
                    double param1,
                    double param2,
                    double alpha,
                    double current_color,
                    double webkit_active_link,
                    double webkit_link,
                    double quirk_inherit,
                    Color::ColorSpace color_space)
      : param0_(param0),
        param1_(param1),
        param2_(param2),
        alpha_(alpha),
        current_color_(current_color),
        webkit_active_link_(webkit_active_link),
        webkit_link_(webkit_link),
        quirk_inherit_(quirk_inherit),
        color_space_(color_space) {}

 private:
  void ConvertToColorSpace(Color::ColorSpace color_space);
  InterpolableColor* RawClone() const final;
  InterpolableColor* RawCloneAndZero() const final;

  // All color params are stored premultiplied by alpha.
  // https://csswg.sesse.net/css-color-4/#interpolation-space
  double param0_;
  double param1_;
  double param2_;
  double alpha_;

  double current_color_;
  double webkit_active_link_;
  double webkit_link_;
  double quirk_inherit_;

  Color::ColorSpace color_space_ = Color::ColorSpace::kNone;
};

template <>
struct DowncastTraits<InterpolableColor> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsColor();
  }
  static bool AllowFrom(const BaseInterpolableColor& base) {
    return base.IsColor();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_COLOR_H_
