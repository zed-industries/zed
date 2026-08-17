// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REPEAT_STYLE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REPEAT_STYLE_VALUE_H_

#include "third_party/blink/renderer/core/css/css_identifier_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// This class represents a repeat-style value as specified in:
// https://drafts.csswg.org/css-backgrounds-3/#typedef-repeat-style
// <repeat-style> = repeat-x | repeat-y | [repeat | space | round |
// no-repeat]{1,2}
class CORE_EXPORT CSSRepeatStyleValue : public CSSValue {
 public:
  explicit CSSRepeatStyleValue(const CSSIdentifierValue* id);
  CSSRepeatStyleValue(const CSSIdentifierValue* x, const CSSIdentifierValue* y);

  // It is expected that CSSRepeatStyleValue objects should always be created
  // with at least one non-null id value.
  CSSRepeatStyleValue() = delete;

  ~CSSRepeatStyleValue();

  String CustomCSSText() const;

  bool Equals(const CSSRepeatStyleValue& other) const;

  bool IsRepeat() const;

  const CSSIdentifierValue* x() const { return x_; }
  const CSSIdentifierValue* y() const { return y_; }

  void TraceAfterDispatch(blink::Visitor* visitor) const;

 private:
  Member<const CSSIdentifierValue> x_;
  Member<const CSSIdentifierValue> y_;
};

template <>
struct DowncastTraits<CSSRepeatStyleValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsRepeatStyleValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REPEAT_STYLE_VALUE_H_
