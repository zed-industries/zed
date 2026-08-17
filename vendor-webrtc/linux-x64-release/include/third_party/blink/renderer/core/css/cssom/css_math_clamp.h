// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_CLAMP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_CLAMP_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/v8_css_math_operator.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_math_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_array.h"

namespace WTF {
class StringBuilder;
}  // namespace WTF

namespace blink {

// Represents the central calculation of three CSSNumericValues.
// See CSSMathClamp.idl for more information about this class.
class CORE_EXPORT CSSMathClamp final : public CSSMathValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // The constructor defined in the IDL.
  static CSSMathClamp* Create(V8CSSNumberish* lower,
                              V8CSSNumberish* value,
                              V8CSSNumberish* upper,
                              ExceptionState& exception_state);
  // Blink-internal constructor.
  static CSSMathClamp* Create(CSSNumericValue* lower,
                              CSSNumericValue* value,
                              CSSNumericValue* upper);

  CSSMathClamp(CSSNumericValue* lower,
               CSSNumericValue* value,
               CSSNumericValue* upper,
               const CSSNumericValueType& type)
      : CSSMathValue(type), lower_(lower), value_(value), upper_(upper) {}
  CSSMathClamp(const CSSMathClamp&) = delete;
  CSSMathClamp& operator=(const CSSMathClamp&) = delete;

  CSSNumericValue* lowerValue() { return lower_.Get(); }
  CSSNumericValue* valueValue() { return value_.Get(); }
  CSSNumericValue* upperValue() { return upper_.Get(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(lower_);
    visitor->Trace(value_);
    visitor->Trace(upper_);
    CSSMathValue::Trace(visitor);
  }

  V8CSSMathOperator getOperator() const final {
    return V8CSSMathOperator(V8CSSMathOperator::Enum::kClamp);
  }

  V8CSSNumberish* lower();
  V8CSSNumberish* value();
  V8CSSNumberish* upper();

  // From CSSStyleValue.
  StyleValueType GetType() const final { return CSSStyleValue::kClampType; }

  bool Equals(const CSSNumericValue& other) const final {
    if (GetType() != other.GetType()) {
      return false;
    }

    // We can safely cast here as we know 'other' has the same type as us.
    const auto& other_variadic = static_cast<const CSSMathClamp&>(other);
    return (lower_->Equals(*other_variadic.lower_) &&
            value_->Equals(*other_variadic.value_) &&
            upper_->Equals(*other_variadic.upper_));
  }

  template <class BinaryTypeOperation>
  static CSSNumericValueType TypeCheck(CSSNumericValue* lower,
                                       CSSNumericValue* value,
                                       CSSNumericValue* upper,
                                       BinaryTypeOperation op,
                                       bool& error) {
    error = false;
    auto final_type = op(lower->Type(), value->Type(), error);
    if (error) {
      return final_type;
    }

    return op(final_type, upper->Type(), error);
  }

  CSSMathExpressionNode* ToCalcExpressionNode() const final;

 private:
  void BuildCSSText(Nested, ParenLess, WTF::StringBuilder&) const final;

  std::optional<CSSNumericSumValue> SumValue() const final;

  Member<CSSNumericValue> lower_;
  Member<CSSNumericValue> value_;
  Member<CSSNumericValue> upper_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_CLAMP_H_
