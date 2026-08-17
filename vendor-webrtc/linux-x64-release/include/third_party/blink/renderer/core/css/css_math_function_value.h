// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MATH_FUNCTION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MATH_FUNCTION_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_math_expression_node.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"

namespace blink {

class TryTacticTransform;
class WritingDirectionMode;

// Numeric values that involve math functions (calc(), min(), max(), etc). This
// is the equivalence of CSS Typed OM's |CSSMathValue| in the |CSSValue| class
// hierarchy.
class CORE_EXPORT CSSMathFunctionValue : public CSSPrimitiveValue {
 public:
  static CSSMathFunctionValue* Create(const Length&, float zoom);
  static CSSMathFunctionValue* Create(const CSSMathExpressionNode*,
                                      ValueRange = ValueRange::kAll);

  CSSMathFunctionValue(const CSSMathExpressionNode* expression,
                       ValueRange range);

  const CSSMathExpressionNode* ExpressionNode() const {
    return expression_.Get();
  }

  scoped_refptr<const CalculationValue> ToCalcValue(
      const CSSLengthResolver&) const;

  bool MayHaveRelativeUnit() const;

  CalculationResultCategory Category() const { return expression_->Category(); }

  bool IsAngle() const { return Category() == kCalcAngle; }
  bool IsLength() const { return Category() == kCalcLength; }
  bool IsNumber() const { return Category() == kCalcNumber; }
  bool IsPercentage() const { return Category() == kCalcPercent; }
  bool IsTime() const { return Category() == kCalcTime; }
  bool IsResolution() const { return Category() == kCalcResolution; }

  bool IsPx() const;

  ValueRange PermittedValueRange() const {
    return value_range_in_target_context_;
  }

  // When |false|, comparisons between percentage values can be resolved without
  // providing a reference value (e.g., min(10%, 20%) == 10%). When |true|, the
  // result depends on the sign of the reference value (e.g., when referring to
  // a negative value, min(10%, 20%) == 20%).
  // Note: 'background-position' property allows negative reference values.
  bool AllowsNegativePercentageReference() const {
    return allows_negative_percentage_reference_;
  }
  void SetAllowsNegativePercentageReference() {
    // TODO(crbug.com/825895): So far, 'background-position' is the only
    // property that allows resolving a percentage against a negative value. If
    // we have more of such properties, we should instead pass an additional
    // argument to ask the parser to set this flag when constructing |this|.
    allows_negative_percentage_reference_ = true;
  }

  bool IsComputationallyIndependent() const;
  bool IsElementDependent() const;

  // TODO(crbug.com/979895): The semantics of this function is still not very
  // clear. Do not add new callers before further refactoring and cleanups.
  // |DoubleValue()| can be called only when the math expression can be
  // resolved into a single numeric value *without any type conversion* (e.g.,
  // between px and em). Otherwise, it hits a DCHECK.
  double DoubleValue() const;

  double ComputeSeconds(const CSSLengthResolver&) const;
  double ComputeDegrees() const;
  double ComputeDegrees(const CSSLengthResolver&) const;
  double ComputeLengthPx(const CSSLengthResolver&) const;
  double ComputeDotsPerPixel(const CSSLengthResolver&) const;
  int ComputeInteger(const CSSLengthResolver&) const;
  double ComputeNumber(const CSSLengthResolver&) const;
  double ComputePercentage(const CSSLengthResolver&) const;
  double ComputeValueInCanonicalUnit(const CSSLengthResolver&) const;
  std::optional<double> GetValueIfKnown() const {
    std::optional<double> val = expression_->GetValueIfKnown();
    if (val.has_value()) {
      return ClampToPermittedRange(*val);
    } else {
      return val;
    }
  }

  bool AccumulateLengthArray(CSSLengthArray& length_array,
                             double multiplier) const;
  Length ConvertToLength(const CSSLengthResolver&) const;

  void AccumulateLengthUnitTypes(LengthTypeFlags& types) const {
    expression_->AccumulateLengthUnitTypes(types);
  }

  String CustomCSSText() const;
  bool Equals(const CSSMathFunctionValue& other) const;

  bool HasComparisons() const { return expression_->HasComparisons(); }

  // True if this value has anchor() or anchor-size() somewhere within
  // the math expression (regardless of the validity of those functions).
  //
  // https://drafts.csswg.org/css-anchor-position-1/#anchor-pos
  // https://drafts.csswg.org/css-anchor-position-1/#anchor-size-fn
  bool HasAnchorFunctions() const { return expression_->HasAnchorFunctions(); }

  // Checks if any anchor() or anchor-size() functions, when evaluated, would
  // cause the declaration holding this value to become invalid at
  // computed-value time.
  //
  // https://drafts.csswg.org/css-anchor-position-1/#anchor-valid
  // https://drafts.csswg.org/css-anchor-position-1/#anchor-size-valid
  bool HasInvalidAnchorFunctions(
      const CSSLengthResolver& length_resolver) const {
    return expression_->HasInvalidAnchorFunctions(length_resolver);
  }

  const CSSValue& PopulateWithTreeScope(const TreeScope*) const;

  // Rewrite this function according to the specified TryTacticTransform,
  // e.g. anchor(left) -> anchor(right). If this function is not affected
  // by the transform, returns `this`.
  //
  // LogicalAxis determines how to interpret the values that don't
  // intrinsically indicate the axis: start, end, self-start, self-end.
  // For LogicalAxis::kInline, any start (etc) within this value is
  // interpreted to mean 'inline-start', and similarly for kBlock.
  //
  // See also TryTacticTransform.
  const CSSMathFunctionValue* TransformAnchors(
      LogicalAxis,
      const TryTacticTransform&,
      const WritingDirectionMode&) const;

  void TraceAfterDispatch(blink::Visitor* visitor) const;

 private:
  double ClampToPermittedRange(double) const;

  Member<const CSSMathExpressionNode> expression_;
  ValueRange value_range_in_target_context_;
};

template <>
struct DowncastTraits<CSSMathFunctionValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsMathFunctionValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MATH_FUNCTION_VALUE_H_
