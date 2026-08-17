// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNSUPPORTED_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNSUPPORTED_COLOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_color.h"
#include "third_party/blink/renderer/core/css/cssom/css_unsupported_style_value.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// CSSUnsupportedColor represents all color values that are normally
// treated as CSSUnsupportedValue.  When compositing color values cross thread,
// this class can be used to differentiate between color values and other types
// that use CSSUnsupportedStyleValue.
class CORE_EXPORT CSSUnsupportedColor final : public CSSUnsupportedStyleValue {
 public:
  explicit CSSUnsupportedColor(Color color)
      : CSSUnsupportedStyleValue(
            cssvalue::CSSColor::SerializeAsCSSComponentValue(color)),
        color_value_(color) {}
  explicit CSSUnsupportedColor(const CSSPropertyName& name, Color color)
      : CSSUnsupportedStyleValue(
            name,
            cssvalue::CSSColor::SerializeAsCSSComponentValue(color)),
        color_value_(color) {}
  explicit CSSUnsupportedColor(const cssvalue::CSSColor& color_value)
      : CSSUnsupportedColor(color_value.Value()) {}
  CSSUnsupportedColor(const CSSUnsupportedColor&) = delete;
  CSSUnsupportedColor& operator=(const CSSUnsupportedColor&) = delete;

  StyleValueType GetType() const override { return kUnsupportedColorType; }

  Color Value() const;

  const CSSValue* ToCSSValue() const override;

 private:
  Color color_value_;
};

template <>
struct DowncastTraits<CSSUnsupportedColor> {
  static bool AllowFrom(const CSSStyleValue& value) {
    return value.GetType() ==
           CSSStyleValue::StyleValueType::kUnsupportedColorType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNSUPPORTED_COLOR_H_
