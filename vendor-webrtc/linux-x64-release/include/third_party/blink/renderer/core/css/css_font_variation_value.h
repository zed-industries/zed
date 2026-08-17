// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_VARIATION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_VARIATION_VALUE_H_

#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
namespace cssvalue {

class CSSFontVariationValue : public CSSValue {
 public:
  CSSFontVariationValue(const AtomicString& tag,
                        const CSSPrimitiveValue* value);

  const AtomicString& Tag() const { return tag_; }
  const CSSPrimitiveValue* Value() const { return value_.Get(); }
  String CustomCSSText() const;

  bool Equals(const CSSFontVariationValue&) const;

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    visitor->Trace(value_);
    CSSValue::TraceAfterDispatch(visitor);
  }

 private:
  AtomicString tag_;
  Member<const CSSPrimitiveValue> value_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSFontVariationValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsFontVariationValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_VARIATION_VALUE_H_
