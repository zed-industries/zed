// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PALETTE_MIX_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PALETTE_MIX_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSPrimitiveValue;

namespace cssvalue {

class CORE_EXPORT CSSPaletteMixValue : public CSSValue {
 public:
  CSSPaletteMixValue(
      const CSSValue* palette1,
      const CSSValue* palette2,
      const CSSPrimitiveValue* p1,
      const CSSPrimitiveValue* p2,
      const Color::ColorSpace color_interpolation_space,
      const Color::HueInterpolationMethod hue_interpolation_method)
      : CSSValue(kPaletteMixClass),
        palette1_(palette1),
        palette2_(palette2),
        percentage1_(p1),
        percentage2_(p2),
        color_interpolation_space_(color_interpolation_space),
        hue_interpolation_method_(hue_interpolation_method) {}

  WTF::String CustomCSSText() const;

  void TraceAfterDispatch(blink::Visitor* visitor) const;

  bool Equals(const CSSPaletteMixValue& other) const;

  const CSSValue& Palette1() const { return *palette1_; }
  const CSSValue& Palette2() const { return *palette2_; }
  const CSSPrimitiveValue* Percentage1() const { return percentage1_.Get(); }
  const CSSPrimitiveValue* Percentage2() const { return percentage2_.Get(); }
  Color::ColorSpace ColorInterpolationSpace() const {
    return color_interpolation_space_;
  }
  Color::HueInterpolationMethod HueInterpolationMethod() const {
    return hue_interpolation_method_;
  }

 private:
  Member<const CSSValue> palette1_;
  Member<const CSSValue> palette2_;
  Member<const CSSPrimitiveValue> percentage1_;
  Member<const CSSPrimitiveValue> percentage2_;
  const Color::ColorSpace color_interpolation_space_;
  const Color::HueInterpolationMethod hue_interpolation_method_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSPaletteMixValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsPaletteMixValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PALETTE_MIX_VALUE_H_
