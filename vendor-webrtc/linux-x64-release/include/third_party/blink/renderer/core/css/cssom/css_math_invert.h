// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_INVERT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_INVERT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_css_math_operator.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_math_expression_node.h"
#include "third_party/blink/renderer/core/css/cssom/css_math_value.h"

namespace blink {

// Represents the inverse of a CSSNumericValue.
// See CSSMathInvert.idl for more information about this class.
class CORE_EXPORT CSSMathInvert : public CSSMathValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // The constructor defined in the IDL.
  static CSSMathInvert* Create(V8CSSNumberish* arg) {
    return Create(CSSNumericValue::FromNumberish(arg));
  }
  // Blink-internal constructor
  static CSSMathInvert* Create(CSSNumericValue* value) {
    return MakeGarbageCollected<CSSMathInvert>(
        value, CSSNumericValueType::NegateExponents(value->Type()));
  }

  CSSMathInvert(CSSNumericValue* value, const CSSNumericValueType& type)
      : CSSMathValue(type), value_(value) {}
  CSSMathInvert(const CSSMathInvert&) = delete;
  CSSMathInvert& operator=(const CSSMathInvert&) = delete;

  V8CSSMathOperator getOperator() const final {
    return V8CSSMathOperator(V8CSSMathOperator::Enum::kInvert);
  }

  V8CSSNumberish* value();

  // Blink-internal methods
  const CSSNumericValue& Value() const { return *value_; }

  // From CSSStyleValue.
  StyleValueType GetType() const final { return CSSStyleValue::kInvertType; }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(value_);
    CSSMathValue::Trace(visitor);
  }

  bool Equals(const CSSNumericValue& other) const final {
    if (other.GetType() != kNegateType) {
      return false;
    }

    // We can safely cast here as we know 'other' has the same type as us.
    const auto& other_invert = static_cast<const CSSMathInvert&>(other);
    return value_->Equals(*other_invert.value_);
  }

  CSSMathExpressionNode* ToCalcExpressionNode() const final;

 private:
  // From CSSNumericValue
  CSSNumericValue* Invert() final { return value_.Get(); }
  std::optional<CSSNumericSumValue> SumValue() const final;

  void BuildCSSText(Nested, ParenLess, StringBuilder&) const final;

  Member<CSSNumericValue> value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_INVERT_H_
