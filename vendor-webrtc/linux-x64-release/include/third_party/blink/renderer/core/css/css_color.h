// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COLOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CSSValuePool;

namespace cssvalue {

// Represents the non-keyword subset of <color>.
class CORE_EXPORT CSSColor : public CSSValue {
 public:
  static CSSColor* Create(const Color& color);

  CSSColor(Color color) : CSSValue(kColorClass), color_(color) {}

  String CustomCSSText() const { return SerializeAsCSSComponentValue(color_); }

  Color Value() const { return color_; }

  bool Equals(const CSSColor& other) const { return color_ == other.color_; }
  unsigned CustomHash() const { return color_.GetHash(); }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValue::TraceAfterDispatch(visitor);
  }

  // Returns the color serialized according to CSSOM:
  // https://drafts.csswg.org/cssom/#serialize-a-css-component-value
  static String SerializeAsCSSComponentValue(Color color);

 private:
  friend class ::blink::CSSValuePool;

  Color color_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSColor> {
  static bool AllowFrom(const CSSValue& value) { return value.IsColorValue(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COLOR_H_
