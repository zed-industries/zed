// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_KEYWORD_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_KEYWORD_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class ExceptionState;

class CORE_EXPORT CSSKeywordValue final : public CSSStyleValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CSSKeywordValue* Create(const String& keyword);
  static CSSKeywordValue* Create(const String& keyword, ExceptionState&);
  static CSSKeywordValue* FromCSSValue(const CSSValue&);

  explicit CSSKeywordValue(const String& keyword) : keyword_value_(keyword) {}
  explicit CSSKeywordValue(CSSValueID keyword_value);
  CSSKeywordValue(const CSSKeywordValue&) = delete;
  CSSKeywordValue& operator=(const CSSKeywordValue&) = delete;

  StyleValueType GetType() const override { return kKeywordType; }

  const String& value() const;
  void setValue(const String& keyword, ExceptionState&);
  CSSValueID KeywordValueID() const;

  const CSSValue* ToCSSValue() const override;

 private:
  String keyword_value_;
};

template <>
struct DowncastTraits<CSSKeywordValue> {
  static bool AllowFrom(const CSSStyleValue& value) {
    return value.GetType() == CSSStyleValue::StyleValueType::kKeywordType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_KEYWORD_VALUE_H_
