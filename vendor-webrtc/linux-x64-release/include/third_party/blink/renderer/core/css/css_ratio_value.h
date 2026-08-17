// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RATIO_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RATIO_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

namespace cssvalue {

// https://drafts.csswg.org/css-values-4/#ratios
class CORE_EXPORT CSSRatioValue : public CSSValue {
 public:
  CSSRatioValue(const CSSPrimitiveValue& first,
                const CSSPrimitiveValue& second);

  // Numerator, but called 'first' by the spec.
  const CSSPrimitiveValue& First() const { return *first_; }

  // Denominator, but called 'second' by the spec.
  const CSSPrimitiveValue& Second() const { return *second_; }

  String CustomCSSText() const;
  bool Equals(const CSSRatioValue&) const;

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    visitor->Trace(first_);
    visitor->Trace(second_);
    CSSValue::TraceAfterDispatch(visitor);
  }

 private:
  Member<const CSSPrimitiveValue> first_;
  Member<const CSSPrimitiveValue> second_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSRatioValue> {
  static bool AllowFrom(const CSSValue& value) { return value.IsRatioValue(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RATIO_VALUE_H_
