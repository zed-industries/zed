// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_ALTERNATE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_ALTERNATE_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_custom_ident_value.h"
#include "third_party/blink/renderer/core/css/css_function_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

namespace cssvalue {

// A function-like entry in the font-variant-alternates property.
// https://drafts.csswg.org/css-fonts-4/#font-variant-alternates-prop
class CORE_EXPORT CSSAlternateValue : public CSSValue {
 public:
  CSSAlternateValue(const CSSFunctionValue& function,
                    const CSSValueList& alias_list);

  const CSSFunctionValue& Function() const { return *function_; }
  const CSSValueList& Aliases() const { return *aliases_; }

  String CustomCSSText() const;
  bool Equals(const CSSAlternateValue&) const;

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    visitor->Trace(function_);
    visitor->Trace(aliases_);
    CSSValue::TraceAfterDispatch(visitor);
  }

 private:
  Member<const CSSFunctionValue> function_;
  Member<const CSSValueList> aliases_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSAlternateValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsAlternateValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_ALTERNATE_VALUE_H_
