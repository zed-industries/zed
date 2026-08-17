// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNIT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNIT_VALUE_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_numeric_literal_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class StringBuilder;
}  // namespace WTF

namespace blink {

// Represents numeric values that can be expressed as a single number plus a
// unit (or a naked number or percentage).
// See CSSUnitValue.idl for more information about this class.
class CORE_EXPORT CSSUnitValue final : public CSSNumericValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // The constructor defined in the IDL.
  static CSSUnitValue* Create(double value,
                              const String& unit,
                              ExceptionState&);
  // Blink-internal ways of creating CSSUnitValues.
  static CSSUnitValue* Create(
      double value,
      CSSPrimitiveValue::UnitType = CSSPrimitiveValue::UnitType::kNumber);
  static CSSUnitValue* FromCSSValue(const CSSNumericLiteralValue&);

  CSSUnitValue(double value, CSSPrimitiveValue::UnitType unit)
      : CSSNumericValue(CSSNumericValueType(unit)),
        value_(value),
        unit_(unit) {}
  CSSUnitValue(const CSSUnitValue&) = delete;
  CSSUnitValue& operator=(const CSSUnitValue&) = delete;

  // Setters and getters for attributes defined in the IDL.
  void setValue(double new_value) { value_ = new_value; }
  double value() const { return value_; }
  String unit() const;

  // Internal methods.
  CSSPrimitiveValue::UnitType GetInternalUnit() const { return unit_; }
  CSSUnitValue* ConvertTo(CSSPrimitiveValue::UnitType) const;

  // From CSSNumericValue.
  bool IsUnitValue() const final { return true; }
  std::optional<CSSNumericSumValue> SumValue() const final;

  bool Equals(const CSSNumericValue&) const final;

  // From CSSStyleValue.
  StyleValueType GetType() const final;
  const CSSNumericLiteralValue* ToCSSValue() const final;
  const CSSPrimitiveValue* ToCSSValueWithProperty(CSSPropertyID) const final;
  CSSMathExpressionNode* ToCalcExpressionNode() const final;

 private:
  double ConvertFixedLength(CSSPrimitiveValue::UnitType) const;
  double ConvertAngle(CSSPrimitiveValue::UnitType) const;

  void BuildCSSText(Nested, ParenLess, WTF::StringBuilder&) const final;

  // From CSSNumericValue
  CSSNumericValue* Negate() final;
  CSSNumericValue* Invert() final;

  double value_;
  CSSPrimitiveValue::UnitType unit_;
};

template <>
struct DowncastTraits<CSSUnitValue> {
  static bool AllowFrom(const CSSStyleValue& value) {
    return value.GetType() == CSSStyleValue::StyleValueType::kUnitType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNIT_VALUE_H_
