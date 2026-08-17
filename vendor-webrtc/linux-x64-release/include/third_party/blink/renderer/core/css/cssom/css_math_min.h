// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_MIN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_MIN_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/v8_css_math_operator.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_math_variadic.h"

namespace blink {

struct CSSNumericSumValue;

// Represents the minimum of one or more CSSNumericValues.
// See CSSMathMin.idl for more information about this class.
class CORE_EXPORT CSSMathMin final : public CSSMathVariadic {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // The constructor defined in the IDL.
  static CSSMathMin* Create(const HeapVector<Member<V8CSSNumberish>>& args,
                            ExceptionState& exception_state);
  // Blink-internal constructor.
  static CSSMathMin* Create(CSSNumericValueVector);

  CSSMathMin(CSSNumericArray* values, const CSSNumericValueType& type)
      : CSSMathVariadic(values, type) {}
  CSSMathMin(const CSSMathMin&) = delete;
  CSSMathMin& operator=(const CSSMathMin&) = delete;

  V8CSSMathOperator getOperator() const final {
    return V8CSSMathOperator(V8CSSMathOperator::Enum::kMin);
  }

  // From CSSStyleValue.
  StyleValueType GetType() const final { return CSSStyleValue::kMinType; }

  CSSMathExpressionNode* ToCalcExpressionNode() const final;

 private:
  void BuildCSSText(Nested, ParenLess, StringBuilder&) const final;

  std::optional<CSSNumericSumValue> SumValue() const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_MIN_H_
