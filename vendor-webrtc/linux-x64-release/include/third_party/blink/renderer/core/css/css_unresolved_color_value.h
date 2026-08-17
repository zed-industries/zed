// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_UNRESOLVED_COLOR_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_UNRESOLVED_COLOR_VALUE_H_

#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/properties/css_color_function_parser.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {
namespace cssvalue {

// Stores a CSS non-relative color that could not be fully resolved at parse
// time (relative unresolved colors are handled by CSSRelativeColorValue).
class CORE_EXPORT CSSUnresolvedColorValue : public CSSValue {
 public:
  // nullptr values are presumed to be “none”.
  CSSUnresolvedColorValue(
      Color::ColorSpace color_space,
      const CSSPrimitiveValue* channel0,
      const CSSPrimitiveValue* channel1,
      const CSSPrimitiveValue* channel2,
      const std::array<ColorFunctionParser::ChannelType, 3>& channel_types,
      const CSSPrimitiveValue* alpha,
      ColorFunctionParser::ChannelType alpha_channel_type)
      : CSSValue(kUnresolvedColorClass),
        color_space_(color_space),
        channels_({channel0, channel1, channel2}),
        channel_types_(channel_types),
        alpha_(alpha),
        alpha_channel_type_(alpha_channel_type) {}

  WTF::String CustomCSSText() const;

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    visitor->Trace(channels_[0]);
    visitor->Trace(channels_[1]);
    visitor->Trace(channels_[2]);
    visitor->Trace(alpha_);
    CSSValue::TraceAfterDispatch(visitor);
  }

  bool Equals(const CSSUnresolvedColorValue& other) const;

  Color Resolve(const CSSLengthResolver& resolver) const;

 private:
  const Color::ColorSpace color_space_;
  std::array<Member<const CSSPrimitiveValue>, 3> channels_;
  std::array<ColorFunctionParser::ChannelType, 3> channel_types_;
  Member<const CSSPrimitiveValue> alpha_;
  ColorFunctionParser::ChannelType alpha_channel_type_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSUnresolvedColorValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsUnresolvedColorValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_UNRESOLVED_COLOR_VALUE_H_
