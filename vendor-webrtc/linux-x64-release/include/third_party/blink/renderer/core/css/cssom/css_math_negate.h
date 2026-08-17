// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_NEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_NEGATE_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_css_math_operator.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_math_value.h"

namespace WTF {
class StringBuilder;
}  // namespace WTF

namespace blink {

// Represents the negation of a CSSNumericValue.
// See CSSMathNegate.idl for more information about this class.
class CORE_EXPORT CSSMathNegate : public CSSMathValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // The constructor defined in the IDL.
  static CSSMathNegate* Create(V8CSSNumberish* arg) {
    return Create(CSSNumericValue::FromNumberish(arg));
  }
  // Blink-internal constructor
  static CSSMathNegate* Create(CSSNumericValue* value) {
    return MakeGarbageCollected<CSSMathNegate>(value, value->Type());
  }

  CSSMathNegate(CSSNumericValue* value, const CSSNumericValueType& type)
      : CSSMathValue(type), value_(value) {}
  CSSMathNegate(const CSSMathNegate&) = delete;
  CSSMathNegate& operator=(const CSSMathNegate&) = delete;

  V8CSSMathOperator getOperator() const final {
    return V8CSSMathOperator(V8CSSMathOperator::Enum::kNegate);
  }

  V8CSSNumberish* value();

  // Blink-internal methods
  const CSSNumericValue& Value() const { return *value_; }

  // From CSSStyleValue.
  StyleValueType GetType() const final { return CSSStyleValue::kNegateType; }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(value_);
    CSSMathValue::Trace(visitor);
  }

  bool Equals(const CSSNumericValue& other) const final {
    if (other.GetType() != kNegateType) {
      return false;
    }

    // We can safely cast here as we know 'other' has the same type as us.
    const auto& other_negate = static_cast<const CSSMathNegate&>(other);
    return value_->Equals(*other_negate.value_);
  }

  CSSMathExpressionNode* ToCalcExpressionNode() const final;

 private:
  // From CSSNumericValue
  CSSNumericValue* Negate() final { return value_.Get(); }
  std::optional<CSSNumericSumValue> SumValue() const final;

  void BuildCSSText(Nested, ParenLess, WTF::StringBuilder&) const final;

  Member<CSSNumericValue> value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_NEGATE_H_
