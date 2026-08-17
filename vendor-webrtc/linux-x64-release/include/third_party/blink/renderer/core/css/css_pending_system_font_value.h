// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PENDING_SYSTEM_FONT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PENDING_SYSTEM_FONT_VALUE_H_

#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

namespace cssvalue {

// The 'font' shorthand accepts some special system font values, like 'caption'
// (https://drafts.csswg.org/css-fonts/#valdef-font-caption).
//
// The resolution of these values into longhands is platform-dependent, and can
// also depend on user's settings, like the default font size.
//
// The CSS parser wouldn't be able to resolve these, since we need a |Document|
// in order to retrieve the settings, and |CSSParserContext::GetDocument()|
// would be null when system fonts are set in UA styles.
//
// So the parser sets all the font longhands to a |CSSPendingSystemFontValue|,
// and the resolution is deferred until computed-value time, when we can use
// |StyleResolverState::GetDocument()|.
class CSSPendingSystemFontValue : public CSSValue {
 public:
  static CSSPendingSystemFontValue* Create(CSSValueID);

  explicit CSSPendingSystemFontValue(CSSValueID);

  CSSValueID SystemFontId() const { return system_font_id_; }

  const AtomicString& ResolveFontFamily() const;
  float ResolveFontSize(const Document*) const;

  bool Equals(const CSSPendingSystemFontValue& other) const {
    return system_font_id_ == other.system_font_id_;
  }

  WTF::String CustomCSSText() const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  const CSSValueID system_font_id_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSPendingSystemFontValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsPendingSystemFontValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PENDING_SYSTEM_FONT_VALUE_H_
