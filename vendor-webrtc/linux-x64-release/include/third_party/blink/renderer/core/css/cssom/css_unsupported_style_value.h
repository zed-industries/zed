// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNSUPPORTED_STYLE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNSUPPORTED_STYLE_VALUE_H_

#include <optional>

#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_name.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// CSSUnsupportedStyleValue is the internal representation of a base
// CSSStyleValue that is returned when we do not yet support a CSS Typed OM type
// for a given CSS Value.
//
// It is either:
//
// * Tied to a specific CSS property, and therefore only valid for that
//   property, or
// * Tied to no CSS property at all, in which case it's not valid for any
//   property.

class CORE_EXPORT CSSUnsupportedStyleValue : public CSSStyleValue {
 public:
  CSSUnsupportedStyleValue(const String& css_text) { SetCSSText(css_text); }
  CSSUnsupportedStyleValue(const CSSPropertyName& name, const String& css_text)
      : name_(name) {
    SetCSSText(css_text);
  }
  CSSUnsupportedStyleValue(const CSSPropertyName& name, const CSSValue& value)
      : name_(name) {
    SetCSSText(value.CssText());
  }
  CSSUnsupportedStyleValue(const CSSUnsupportedStyleValue&) = delete;
  CSSUnsupportedStyleValue& operator=(const CSSUnsupportedStyleValue&) = delete;

  StyleValueType GetType() const override {
    return StyleValueType::kUnknownType;
  }
  bool IsValidFor(const CSSPropertyName& name) const {
    return name_ && *name_ == name;
  }

  const CSSValue* ToCSSValue() const override { NOTREACHED(); }

  String toString() const final { return CSSText(); }

 private:
  std::optional<CSSPropertyName> name_;
};

template <>
struct DowncastTraits<CSSUnsupportedStyleValue> {
  static bool AllowFrom(const CSSStyleValue& value) {
    return value.GetType() == CSSStyleValue::StyleValueType::kUnknownType ||
           value.GetType() ==
               CSSStyleValue::StyleValueType::kUnsupportedColorType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNSUPPORTED_STYLE_VALUE_H_
