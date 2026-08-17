// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RELATIVE_COLOR_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RELATIVE_COLOR_VALUE_H_

#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {
namespace cssvalue {

// Stores a CSS relative color that could not be fully resolved at parse time.
// See: https://www.w3.org/TR/css-color-5/#relative-colors
class CORE_EXPORT CSSRelativeColorValue : public CSSValue {
 public:
  CSSRelativeColorValue(const CSSValue& origin_color,
                        Color::ColorSpace color_interpolation_space,
                        const CSSValue& channel0,
                        const CSSValue& channel1,
                        const CSSValue& channel2,
                        const CSSValue* alpha);

  WTF::String CustomCSSText() const;

  void TraceAfterDispatch(blink::Visitor* visitor) const;

  bool Equals(const CSSRelativeColorValue& other) const;

  const CSSValue& OriginColor() const;
  Color::ColorSpace ColorInterpolationSpace() const;
  const CSSValue& Channel0() const;
  const CSSValue& Channel1() const;
  const CSSValue& Channel2() const;

  // Alpha will be nullptr if it was not specified.
  const CSSValue* Alpha() const;

 private:
  Member<const CSSValue> origin_color_;
  const Color::ColorSpace color_interpolation_space_;
  Member<const CSSValue> channel0_;
  Member<const CSSValue> channel1_;
  Member<const CSSValue> channel2_;
  Member<const CSSValue> alpha_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSRelativeColorValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsRelativeColorValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RELATIVE_COLOR_VALUE_H_
