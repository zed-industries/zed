/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2007 Alexey Proskuryakov <ap@webkit.org>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PRIMITIVE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PRIMITIVE_VALUE_H_

#include <array>
#include <bitset>
#include <concepts>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"

namespace blink {

class CSSLengthResolver;
class CSSMathExpressionNode;

// Dimension calculations are imprecise, often resulting in values of e.g.
// 44.99998. We need to go ahead and round if we're really close to the next
// integer value.
template <typename T>
inline T RoundForImpreciseConversion(double value) {
  value += (value < 0) ? -0.01 : +0.01;
  return ((value > std::numeric_limits<T>::max()) ||
          (value < std::numeric_limits<T>::min()))
             ? 0
             : static_cast<T>(value);
}

template <>
inline float RoundForImpreciseConversion(double value) {
  double ceiled_value = ceil(value);
  double proximity_to_next_int = ceiled_value - value;
  if (proximity_to_next_int <= 0.01 && value > 0) {
    return static_cast<float>(ceiled_value);
  }
  if (proximity_to_next_int >= 0.99 && value < 0) {
    return static_cast<float>(floor(value));
  }
  return static_cast<float>(value);
}

// Common interface for numeric data types, including both literals (e.g. 1,
// 10px, 4%) and values involving math functions (e.g. calc(3px + 2em)).
class CORE_EXPORT CSSPrimitiveValue : public CSSValue {
 public:
  // These units are iterated through, so be careful when adding or changing the
  // order.
  enum class UnitType {
    kUnknown,
    kNumber,
    kPercentage,
    // Length units
    kEms,
    kExs,
    kPixels,
    kCentimeters,
    kMillimeters,
    kInches,
    kPoints,
    kPicas,
    kQuarterMillimeters,

    // https://drafts.csswg.org/css-values-4/#viewport-relative-lengths
    //
    // See also IsViewportPercentageLength.
    kViewportWidth,
    kViewportHeight,
    kViewportInlineSize,
    kViewportBlockSize,
    kViewportMin,
    kViewportMax,
    kSmallViewportWidth,
    kSmallViewportHeight,
    kSmallViewportInlineSize,
    kSmallViewportBlockSize,
    kSmallViewportMin,
    kSmallViewportMax,
    kLargeViewportWidth,
    kLargeViewportHeight,
    kLargeViewportInlineSize,
    kLargeViewportBlockSize,
    kLargeViewportMin,
    kLargeViewportMax,
    kDynamicViewportWidth,
    kDynamicViewportHeight,
    kDynamicViewportInlineSize,
    kDynamicViewportBlockSize,
    kDynamicViewportMin,
    kDynamicViewportMax,

    // https://drafts.csswg.org/css-contain-3/#container-lengths
    //
    // See also IsContainerPercentageLength.
    kContainerWidth,
    kContainerHeight,
    kContainerInlineSize,
    kContainerBlockSize,
    kContainerMin,
    kContainerMax,

    kRems,
    kRexs,
    kRchs,
    kRics,
    kChs,
    kIcs,
    kLhs,
    kRlhs,
    kCaps,
    kRcaps,
    kUserUnits,  // The SVG term for unitless lengths
    // Angle units
    kDegrees,
    kRadians,
    kGradians,
    kTurns,
    // Time units
    kMilliseconds,
    kSeconds,
    kHertz,
    kKilohertz,
    // Resolution
    kDotsPerPixel,
    kX,  // Short alias for kDotsPerPixel
    kDotsPerInch,
    kDotsPerCentimeter,
    // Other units
    kFlex,
    kInteger,
    kIdent,

    // This value is used to handle quirky margins in reflow roots (body, td,
    // and th) like WinIE. The basic idea is that a stylesheet can use the value
    // __qem (for quirky em) instead of em. When the quirky value is used, if
    // you're in quirks mode, the margin will collapse away inside a table cell.
    // This quirk is specified in the HTML spec but our impl is different.
    // TODO: Remove this. crbug.com/443952
    kQuirkyEms,
  };

  enum LengthUnitType {
    kUnitTypePixels = 0,
    kUnitTypePercentage,
    kUnitTypeFontSize,
    kUnitTypeFontXSize,
    kUnitTypeFontCapitalHeight,
    kUnitTypeRootFontCapitalHeight,
    kUnitTypeRootFontSize,
    kUnitTypeRootFontXSize,
    kUnitTypeRootFontZeroCharacterWidth,
    kUnitTypeZeroCharacterWidth,
    kUnitTypeViewportWidth,
    kUnitTypeViewportHeight,
    kUnitTypeViewportInlineSize,
    kUnitTypeViewportBlockSize,
    kUnitTypeViewportMin,
    kUnitTypeViewportMax,
    // Units above this line are supported by CSSLengthArray.
    // See CSSLengthArray::kSize.
    kUnitTypeSmallViewportWidth,
    kUnitTypeSmallViewportHeight,
    kUnitTypeSmallViewportInlineSize,
    kUnitTypeSmallViewportBlockSize,
    kUnitTypeSmallViewportMin,
    kUnitTypeSmallViewportMax,
    kUnitTypeLargeViewportWidth,
    kUnitTypeLargeViewportHeight,
    kUnitTypeLargeViewportInlineSize,
    kUnitTypeLargeViewportBlockSize,
    kUnitTypeLargeViewportMin,
    kUnitTypeLargeViewportMax,
    kUnitTypeDynamicViewportWidth,
    kUnitTypeDynamicViewportHeight,
    kUnitTypeDynamicViewportInlineSize,
    kUnitTypeDynamicViewportBlockSize,
    kUnitTypeDynamicViewportMin,
    kUnitTypeDynamicViewportMax,
    kUnitTypeContainerWidth,
    kUnitTypeContainerHeight,
    kUnitTypeContainerInlineSize,
    kUnitTypeContainerBlockSize,
    kUnitTypeContainerMin,
    kUnitTypeContainerMax,
    kUnitTypeIdeographicFullWidth,
    kUnitTypeRootFontIdeographicFullWidth,
    kUnitTypeLineHeight,
    kUnitTypeRootLineHeight,

    // This value must come after the last length unit type to enable iteration
    // over the length unit types.
    kLengthUnitTypeCount,
  };

  // For performance reasons, InterpolableLength represents "sufficiently
  // simple" <length> values as the terms in a sum, e.g.(10px + 1em + ...),
  // stored in this class.
  //
  // For cases which can't be covered by CSSLengthArray [1], we instead
  // interpolate using CSSMathExpressionNodes.
  //
  // To avoid an excessively large array of size kLengthUnitTypeCount, only a
  // small subset of the units are supported in this optimization.
  //
  // [1] See AccumulateLengthArray.
  struct CSSLengthArray {
    static const wtf_size_t kSize = kUnitTypeViewportMax + 1u;
    static_assert(kUnitTypePixels < kSize, "px unit supported");
    static_assert(kUnitTypePercentage < kSize, "percentage supported");
    static_assert(kUnitTypeFontSize < kSize, "em unit supported");
    static_assert(kUnitTypeFontXSize < kSize, "ex unit supported");
    static_assert(kUnitTypeRootFontSize < kSize, "rem unit supported");
    static_assert(kUnitTypeRootFontXSize < kSize, "rex unit supported");
    static_assert(kUnitTypeZeroCharacterWidth < kSize, "ch unit supported");
    static_assert(kUnitTypeRootFontZeroCharacterWidth < kSize,
                  "rch unit supported");
    static_assert(kUnitTypeFontCapitalHeight < kSize, "cap unit supported");
    static_assert(kUnitTypeRootFontCapitalHeight < kSize,
                  "rcap unit supported");
    static_assert(kUnitTypeViewportWidth < kSize, "vw unit supported");
    static_assert(kUnitTypeViewportHeight < kSize, "vh unit supported");
    static_assert(kUnitTypeViewportInlineSize < kSize, "vi unit supported");
    static_assert(kUnitTypeViewportBlockSize < kSize, "vb unit supported");
    static_assert(kUnitTypeViewportMin < kSize, "vmin unit supported");
    static_assert(kUnitTypeViewportMax < kSize, "vmax unit supported");

    std::array<double, kSize> values{{0}};
    // Indicates whether or not a given value is explicitly set in |values|.
    std::bitset<kSize> type_flags;
  };

  // Returns false if the value cannot be represented as a CSSLengthArray,
  // which happens when comparisons are involved (e.g., max(10px, 10%)),
  // or when we encounter a unit which is not supported by CSSLengthArray.
  bool AccumulateLengthArray(CSSLengthArray&, double multiplier = 1) const;

  // Returns all types of length units involved in this value.
  using LengthTypeFlags = std::bitset<kLengthUnitTypeCount>;
  void AccumulateLengthUnitTypes(LengthTypeFlags& types) const;

  // v*, sv*, lv*
  static bool HasStaticViewportUnits(const LengthTypeFlags&);
  // dv*
  static bool HasDynamicViewportUnits(const LengthTypeFlags&);

  enum UnitCategory {
    kUNumber,
    kUPercent,
    kULength,
    kUAngle,
    kUTime,
    kUFrequency,
    kUResolution,
    kUOther
  };
  static UnitCategory UnitTypeToUnitCategory(UnitType);
  static float ClampToCSSLengthRange(double);

  enum class ValueRange {
    kAll,
    kNonNegative,
    kInteger,
    kNonNegativeInteger,
    kPositiveInteger
  };

  static Length::ValueRange ConversionToLengthValueRange(ValueRange);
  static ValueRange ValueRangeForLengthValueRange(Length::ValueRange);

  static bool IsAngle(UnitType unit) {
    return unit == UnitType::kDegrees || unit == UnitType::kRadians ||
           unit == UnitType::kGradians || unit == UnitType::kTurns;
  }
  bool IsAngle() const;
  static bool IsViewportPercentageLength(UnitType type) {
    return type >= UnitType::kViewportWidth &&
           type <= UnitType::kDynamicViewportMax;
  }
  static bool IsContainerPercentageLength(UnitType type) {
    return type >= UnitType::kContainerWidth && type <= UnitType::kContainerMax;
  }
  static bool IsLength(UnitType type) {
    return (type >= UnitType::kEms && type <= UnitType::kUserUnits) ||
           type == UnitType::kQuirkyEms;
  }
  static inline bool IsRelativeUnit(UnitType type) {
    return type == UnitType::kPercentage || type == UnitType::kEms ||
           type == UnitType::kExs || type == UnitType::kRems ||
           type == UnitType::kChs || type == UnitType::kIcs ||
           type == UnitType::kLhs || type == UnitType::kRexs ||
           type == UnitType::kRchs || type == UnitType::kRics ||
           type == UnitType::kRlhs || type == UnitType::kCaps ||
           type == UnitType::kRcaps || IsViewportPercentageLength(type) ||
           IsContainerPercentageLength(type);
  }
  bool IsLength() const;
  bool IsNumber() const;
  bool IsInteger() const;
  static bool IsPercentage(UnitType unit) {
    return unit == UnitType::kPercentage;
  }
  bool IsPercentage() const;
  // Is this a percentage *or* a calc() with a percentage?
  bool HasPercentage() const;
  bool IsPx() const;
  static bool IsTime(UnitType unit) {
    return unit == UnitType::kSeconds || unit == UnitType::kMilliseconds;
  }
  bool IsTime() const;
  static bool IsFrequency(UnitType unit) {
    return unit == UnitType::kHertz || unit == UnitType::kKilohertz;
  }
  bool IsCalculated() const { return IsMathFunctionValue(); }

  // Whether we are able to resolve to a single value (possibly with a unit)
  // before layout time; i.e. when parsing or calculating style. Things that
  // must wait until layout include all sorts of size-dependent calculations,
  // e.g. calc(100% + 10px) (will be reduced to a length in pixels, but only
  // once we know how wide/tall 100% is), calc(sign(1em - 1px)) (-1, 0 or 1
  // depending on the font size) and so on. Note that pure percentages
  // (e.g. calc(80%)) are specifically allowed; a percentage value counts as
  // a single value with the unit “%”. All values that are _not_ calc()
  // will also by definition return true here.
  bool IsResolvableBeforeLayout() const;

  static bool IsResolution(UnitType type) {
    return type >= UnitType::kDotsPerPixel &&
           type <= UnitType::kDotsPerCentimeter;
  }
  bool IsResolution() const;
  static bool IsFlex(UnitType unit) { return unit == UnitType::kFlex; }
  bool IsFlex() const;

  // https://drafts.css-houdini.org/css-properties-values-api-1/#computationally-independent
  // A property value is computationally independent if it can be converted into
  // a computed value using only the value of the property on the element, and
  // "global" information that cannot be changed by CSS.
  bool IsComputationallyIndependent() const;

  // Returns true if the value has a calculation that depends on an element
  // context. For instance sibling-index().
  //
  // Note that font-relative units are not element-dependent since they resolve
  // against the initial font outside an element context.
  bool IsElementDependent() const;

  // True if this value contains any of cq[w,h,i,b,min,max], false otherwise.
  bool HasContainerRelativeUnits() const;

  // Creates either a |CSSNumericLiteralValue| or a |CSSMathFunctionValue|,
  // depending on whether |value| is calculated or not. We should never create a
  // |CSSPrimitiveValue| that's not of any of its subclasses.
  static CSSPrimitiveValue* CreateFromLength(const Length& value, float zoom);

  double ComputeDegrees() const;

  double ComputeDegrees(const CSSLengthResolver&) const;
  double ComputeSeconds(const CSSLengthResolver&) const;
  double ComputeDotsPerPixel(const CSSLengthResolver&) const;

  // Computes a length in pixels, resolving relative lengths
  template <typename T>
  T ComputeLength(const CSSLengthResolver&) const;

  // Converts to a Length (Fixed, Percent or Calculated)
  Length ConvertToLength(const CSSLengthResolver&) const;

  // this + value
  CSSPrimitiveValue* Add(double value, UnitType unit_type) const;
  // value + this
  CSSPrimitiveValue* AddTo(double value, UnitType unit_type) const;
  // this + value
  CSSPrimitiveValue* Add(const CSSPrimitiveValue& value) const;
  // value + this
  CSSPrimitiveValue* AddTo(const CSSPrimitiveValue& value) const;
  // this - value
  CSSPrimitiveValue* Subtract(double value, UnitType unit_type) const;
  // value - this
  CSSPrimitiveValue* SubtractFrom(double value, UnitType unit_type) const;
  // this - value
  CSSPrimitiveValue* Subtract(const CSSPrimitiveValue& value) const;
  // value - this
  CSSPrimitiveValue* SubtractFrom(const CSSPrimitiveValue& value) const;
  // this * value
  CSSPrimitiveValue* Multiply(double value, UnitType unit_type) const;
  // value * this
  CSSPrimitiveValue* MultiplyBy(double value, UnitType unit_type) const;
  // this * value
  CSSPrimitiveValue* Multiply(const CSSPrimitiveValue& value) const;
  // value * this
  CSSPrimitiveValue* MultiplyBy(const CSSPrimitiveValue& value) const;
  // this / value
  CSSPrimitiveValue* Divide(double value, UnitType unit_type) const;
  // Note: value / this is not allowed until typed arithmetic is implemented.
  CSSPrimitiveValue* DivideBy(double value, UnitType unit_type) const = delete;
  // Note: this / value is not allowed until typed arithmetic is implemented.
  CSSPrimitiveValue* Divide(const CSSPrimitiveValue& value) const = delete;
  // Note: value / this is not allowed until typed arithmetic is implemented.
  CSSPrimitiveValue* DivideBy(const CSSPrimitiveValue& value) const = delete;
  // Replaces every percentage numeric literal node with number typed numeric
  // literal node with value divided by 100 (e.g. 93% -> 0.93). This is needed
  // e.g. for interpolation between <number> and <percentage>, see
  // https://www.w3.org/TR/filter-effects-1/#interpolation-of-filter-functions.
  CSSPrimitiveValue* ConvertLiteralsFromPercentageToNumber() const;

  // TODO(crbug.com/979895): The semantics of these untyped getters are not very
  // clear if |this| is a math function. Do not add new callers before further
  // refactoring and cleanups.
  // These getters can be called only when |this| is a numeric literal or a math
  // expression can be resolved into a single numeric value *without any type
  // conversion* (e.g., between px and em). Otherwise, it hits a DCHECK.
  // In particular, you cannot call this if IsResolvableBeforeLayout()
  // returns false.
  double GetDoubleValue() const;

  // Returns Double Value including infinity, -infinity, and NaN.
  double GetDoubleValueWithoutClamping() const;

  float GetFloatValue() const { return GetValue<float>(); }
  int GetIntValue() const { return GetValue<int>(); }

  template <typename T>
    requires std::integral<T> || std::floating_point<T>
  inline T ConvertTo(const CSSLengthResolver& length_resolver) const {
    DCHECK(IsNumber() || IsPercentage());
    return ClampTo<T>(ComputeNumber(length_resolver));
  }

  int ComputeInteger(const CSSLengthResolver&) const;
  // NOTE: As a special exception, we allow treating percentage values
  // implicitly as numbers divided by 100. This allows us to parse using
  // ConsumeNumberOrPercent() and call ComputeNumber() on whatever we get
  // back.
  double ComputeNumber(const CSSLengthResolver&) const;

  template <typename T = double>
  T ComputePercentage(const CSSLengthResolver&) const;
  double ComputeValueInCanonicalUnit(const CSSLengthResolver&) const;

  std::optional<double> GetValueIfKnown() const;

  static const char* UnitTypeToString(UnitType);
  static UnitType StringToUnitType(StringView string) {
    return string.Is8Bit() ? StringToUnitType(string.Span8())
                           : StringToUnitType(string.Span16());
  }

  String CustomCSSText() const;

  void TraceAfterDispatch(blink::Visitor*) const;

  static UnitType CanonicalUnitTypeForCategory(UnitCategory);
  static UnitType CanonicalUnit(UnitType unit_type);
  static double ConversionToCanonicalUnitsScaleFactor(UnitType);

  // Returns true and populates lengthUnitType, if unitType is a length unit.
  // Otherwise, returns false.
  static bool UnitTypeToLengthUnitType(UnitType, LengthUnitType&);
  static UnitType LengthUnitTypeToUnitType(LengthUnitType);

 protected:
  explicit CSSPrimitiveValue(ClassType class_type) : CSSValue(class_type) {}

  // Code generated by css_primitive_value_unit_trie.cc.tmpl
  static UnitType StringToUnitType(base::span<const LChar>);
  static UnitType StringToUnitType(base::span<const UChar>);

  double ComputeLengthDouble(const CSSLengthResolver&) const;

 protected:
  bool IsResolvableLength() const;

 private:
  template <typename T>
  inline T GetValue() const {
    return ClampTo<T>(GetDoubleValue());
  }

  bool InvolvesLayout() const;
  const CSSMathExpressionNode* ToMathExpressionNode() const;
};

using CSSLengthArray = CSSPrimitiveValue::CSSLengthArray;

template <>
struct DowncastTraits<CSSPrimitiveValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsPrimitiveValue();
  }
};

template <>
int CSSPrimitiveValue::ComputeLength(const CSSLengthResolver&) const;

template <>
Length CSSPrimitiveValue::ComputeLength(const CSSLengthResolver&) const;

template <>
unsigned CSSPrimitiveValue::ComputeLength(const CSSLengthResolver&) const;

template <>
int16_t CSSPrimitiveValue::ComputeLength(const CSSLengthResolver&) const;

template <>
CORE_EXPORT float CSSPrimitiveValue::ComputeLength(
    const CSSLengthResolver&) const;

template <>
CORE_EXPORT double CSSPrimitiveValue::ComputeLength(
    const CSSLengthResolver&) const;
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PRIMITIVE_VALUE_H_
