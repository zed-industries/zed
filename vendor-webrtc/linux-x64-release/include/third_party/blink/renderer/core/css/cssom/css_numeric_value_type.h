// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_VALUE_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_VALUE_TYPE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"

namespace blink {

// Represents the type of a CSSNumericValue, which is a map of base types to
// integers, and an associated percent hint.
// https://drafts.css-houdini.org/css-typed-om/#numeric-typing
class CORE_EXPORT CSSNumericValueType {
 public:
  enum class BaseType : unsigned {
    kLength,
    kAngle,
    kTime,
    kFrequency,
    kResolution,
    kFlex,
    kPercent,
    kNumBaseTypes
  };

  static constexpr unsigned kNumBaseTypes =
      static_cast<unsigned>(BaseType::kNumBaseTypes);

  static String BaseTypeToString(BaseType);

  explicit CSSNumericValueType(
      CSSPrimitiveValue::UnitType = CSSPrimitiveValue::UnitType::kNumber);
  CSSNumericValueType(int exponent, CSSPrimitiveValue::UnitType);

  static CSSNumericValueType NegateExponents(CSSNumericValueType);
  static CSSNumericValueType Add(CSSNumericValueType,
                                 CSSNumericValueType,
                                 bool& error);
  static CSSNumericValueType Multiply(CSSNumericValueType,
                                      CSSNumericValueType,
                                      bool& error);

  int Exponent(BaseType type) const {
    DCHECK_LT(type, BaseType::kNumBaseTypes);
    return exponents_[static_cast<unsigned>(type)];
  }

  void SetExponent(BaseType type, int new_value) {
    DCHECK_LT(type, BaseType::kNumBaseTypes);
    int& old_value = exponents_[static_cast<unsigned>(type)];
    if (old_value == 0 && new_value != 0) {
      num_non_zero_entries_++;
    } else if (old_value != 0 && new_value == 0) {
      num_non_zero_entries_--;
    }
    old_value = new_value;
  }

  bool HasPercentHint() const { return has_percent_hint_; }
  BaseType PercentHint() const { return percent_hint_; }
  void ApplyPercentHint(BaseType hint);

  bool MatchesBaseType(BaseType base_type) const {
    DCHECK_NE(base_type, BaseType::kPercent);
    return IsOnlyNonZeroEntry(base_type, 1) && !HasPercentHint();
  }

  bool MatchesPercentage() const {
    return IsOnlyNonZeroEntry(BaseType::kPercent, 1);
  }

  bool MatchesBaseTypePercentage(BaseType base_type) const {
    DCHECK_NE(base_type, BaseType::kPercent);
    return IsOnlyNonZeroEntry(base_type, 1) ||
           IsOnlyNonZeroEntry(BaseType::kPercent, 1);
  }

  bool MatchesNumber() const {
    return !HasNonZeroEntries() && !HasPercentHint();
  }

  bool MatchesNumberPercentage() const {
    return !HasNonZeroEntries() || IsOnlyNonZeroEntry(BaseType::kPercent, 1);
  }

 private:
  bool HasNonZeroEntries() const { return num_non_zero_entries_ > 0; }
  bool IsOnlyNonZeroEntry(BaseType base_type, int value) const {
    DCHECK_NE(value, 0);
    return num_non_zero_entries_ == 1 && Exponent(base_type) == value;
  }

  Vector<int, kNumBaseTypes> exponents_;
  BaseType percent_hint_ = BaseType::kPercent;
  bool has_percent_hint_ = false;
  unsigned num_non_zero_entries_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_VALUE_TYPE_H_
