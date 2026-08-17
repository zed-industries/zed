// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_VALUE_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_sum_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value_type.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace WTF {
class StringBuilder;
}  // namespace WTF

namespace blink {

class CSSMathExpressionNode;
class CSSMathSum;
class CSSNumericType;
class CSSNumericValue;
class CSSUnitValue;
class ExceptionState;

using CSSNumericValueVector = HeapVector<Member<CSSNumericValue>>;

class CORE_EXPORT CSSNumericValue : public CSSStyleValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSNumericValue(const CSSNumericValue&) = delete;
  CSSNumericValue& operator=(const CSSNumericValue&) = delete;

  static CSSNumericValue* parse(const ExecutionContext*,
                                const String& css_text,
                                ExceptionState&);
  // Blink-internal ways of creating CSSNumericValues.
  static CSSNumericValue* FromCSSValue(const CSSPrimitiveValue&);
  // https://drafts.css-houdini.org/css-typed-om/#rectify-a-numberish-value
  static CSSNumericValue* FromNumberish(const V8CSSNumberish* value);
  // https://drafts.css-houdini.org/css-typed-om/#rectify-a-percentish-value
  static CSSNumericValue* FromPercentish(const V8CSSNumberish* value);

  // Methods defined in the IDL.
  CSSNumericValue* add(const HeapVector<Member<V8CSSNumberish>>& numberishes,
                       ExceptionState& exception_state);
  CSSNumericValue* sub(const HeapVector<Member<V8CSSNumberish>>& numberishes,
                       ExceptionState& exception_state);
  CSSNumericValue* mul(const HeapVector<Member<V8CSSNumberish>>& numberishes,
                       ExceptionState& exception_state);
  CSSNumericValue* div(const HeapVector<Member<V8CSSNumberish>>& numberishes,
                       ExceptionState& exception_state);
  CSSNumericValue* min(const HeapVector<Member<V8CSSNumberish>>& numberishes,
                       ExceptionState& exception_state);
  CSSNumericValue* max(const HeapVector<Member<V8CSSNumberish>>& numberishes,
                       ExceptionState& exception_state);
  bool equals(const HeapVector<Member<V8CSSNumberish>>& numberishes);

  // Converts between compatible types, as defined in the IDL.
  CSSUnitValue* to(const String&, ExceptionState&);
  CSSMathSum* toSum(const Vector<String>&, ExceptionState&);

  CSSNumericType* type() const;

  String toString() const final;

  // Internal methods.
  // Arithmetic
  virtual CSSNumericValue* Negate();
  virtual CSSNumericValue* Invert();

  // Converts between compatible types.
  CSSUnitValue* to(CSSPrimitiveValue::UnitType) const;
  virtual bool IsUnitValue() const = 0;
  virtual std::optional<CSSNumericSumValue> SumValue() const = 0;

  virtual bool Equals(const CSSNumericValue&) const = 0;
  const CSSNumericValueType& Type() const { return type_; }

  virtual CSSMathExpressionNode* ToCalcExpressionNode() const = 0;

  enum class Nested : bool { kYes, kNo };
  enum class ParenLess : bool { kYes, kNo };
  virtual void BuildCSSText(Nested, ParenLess, WTF::StringBuilder&) const = 0;

 protected:
  static bool IsValidUnit(CSSPrimitiveValue::UnitType);
  static CSSPrimitiveValue::UnitType UnitFromName(const String& name);

  CSSNumericValue(const CSSNumericValueType& type) : type_(type) {}

 private:
  CSSNumericValueType type_;
};

CSSNumericValueVector CSSNumberishesToNumericValues(
    const HeapVector<Member<V8CSSNumberish>>& values);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_VALUE_H_
