// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_LENGTH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_LENGTH_H_

#include <optional>

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/interpolation_value.h"
#include "third_party/blink/renderer/core/animation/pairwise_interpolation_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/core/style/computed_style_base_constants.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSToLengthConversionData;
class CSSMathExpressionNode;
class CSSProperty;

class CORE_EXPORT InterpolableLength final : public InterpolableValue {
 public:
  InterpolableLength(CSSLengthArray&& length_array);
  explicit InterpolableLength(const CSSMathExpressionNode& expression);
  explicit InterpolableLength(CSSValueID keyword,
                              std::optional<EInterpolateSize> interpolate_size);

  static InterpolableLength* CreatePixels(double pixels);
  static InterpolableLength* CreatePercent(double pixels);
  static InterpolableLength* CreateNeutral();

  static InterpolableLength* MaybeConvertCSSValue(const CSSValue& value);
  static InterpolableLength* MaybeConvertLength(
      const Length& length,
      const CSSProperty& property,
      float zoom,
      std::optional<EInterpolateSize> interpolate_size);

  static bool CanMergeValues(const InterpolableValue* start,
                             const InterpolableValue* end);

  static PairwiseInterpolationValue MaybeMergeSingles(InterpolableValue* start,
                                                      InterpolableValue* end);

  Length CreateLength(const CSSToLengthConversionData& conversion_data,
                      Length::ValueRange range) const;

  // Unlike CreateLength() this preserves all specified unit types via calc()
  // expressions.
  const CSSPrimitiveValue* CreateCSSValue(Length::ValueRange range) const;

  void SetHasPercentage();
  bool HasPercentage() const;
  void SubtractFromOneHundredPercent();

  bool IsNeutralValue() const;

  InterpolableLength* Clone() const { return RawClone(); }
  InterpolableLength* CloneAndZero() const { return RawCloneAndZero(); }

  // InterpolableValue:
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsLength() const final { return true; }
  bool Equals(const InterpolableValue& other) const final { NOTREACHED(); }
  void Scale(double scale) final;
  void Add(const InterpolableValue& other) final;
  // We override this to avoid two passes in the case of LengthArrays.
  void ScaleAndAdd(double scale, const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  static CSSValueID LengthTypeToCSSValueID(Length::Type lt);
  static Length::Type CSSValueIDToLengthType(CSSValueID id);

  void SetInterpolateSize(EInterpolateSize interpolate_size);

  void Trace(Visitor* v) const override;

 private:
  InterpolableLength* RawClone() const final;
  InterpolableLength* RawCloneAndZero() const final {
    return MakeGarbageCollected<InterpolableLength>(CSSLengthArray());
  }

  bool IsKeyword() const {
    return type_ == Type::kRestrictedKeyword ||
           type_ == Type::kFullyInterpolableKeyword ||
           type_ == Type::kUnknownKeyword;
  }
  bool IsLengthArray() const { return type_ == Type::kLengthArray; }
  bool IsExpression() const { return type_ == Type::kExpression; }

  bool IsKeywordFullyInterpolable() const {
    CHECK(IsKeyword());
    CHECK_NE(type_, Type::kUnknownKeyword);
    return type_ == Type::kFullyInterpolableKeyword;
  }

  bool IsCalcSize() const;

  void SetKeyword(CSSValueID keyword,
                  std::optional<EInterpolateSize> interpolate_size);
  void SetLengthArray(CSSLengthArray&& length_array);
  void SetExpression(const CSSMathExpressionNode& expression);
  const CSSMathExpressionNode& AsExpression() const;

  // We store keyword values (such as 'auto', 'min-content', etc.) in
  // two different ways.  kRestrictedKeyword indicates that the keyword
  // comes from an element that had 'interpolate-size: numeric-only',
  // whereas kFullyInterpolableKeyword indicates that the keyword comes
  // from an element that had `interpolate-size: allow-keywords'.
  // kUnknownKeyword means we don't know which type it is.
  enum class Type : unsigned char {
    kLengthArray,
    kExpression,
    kRestrictedKeyword,
    kFullyInterpolableKeyword,
    kUnknownKeyword
  };

  Type type_;
  CSSValueID keyword_;
  CSSLengthArray length_array_;
  Member<const CSSMathExpressionNode> expression_;
};

template <>
struct DowncastTraits<InterpolableLength> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsLength();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_LENGTH_H_
