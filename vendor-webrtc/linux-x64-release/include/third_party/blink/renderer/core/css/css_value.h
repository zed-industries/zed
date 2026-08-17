/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VALUE_H_

#include "base/memory/values_equivalent.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/custom_spaces.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class Document;
class Length;
class TreeScope;

class CORE_EXPORT CSSValue : public GarbageCollected<CSSValue> {
 public:
  // TODO(sashab): Remove this method and move logic to the caller.
  static CSSValue* Create(const Length& value, float zoom);

  WTF::String CssText() const;
  unsigned Hash() const;

  bool IsNumericLiteralValue() const {
    return class_type_ == kNumericLiteralClass;
  }
  bool IsMathFunctionValue() const { return class_type_ == kMathFunctionClass; }
  bool IsPrimitiveValue() const {
    return IsNumericLiteralValue() || IsMathFunctionValue();
  }
  bool IsIdentifierValue() const { return class_type_ == kIdentifierClass; }
  bool IsScopedKeywordValue() const {
    return class_type_ == kScopedKeywordClass;
  }
  bool IsValuePair() const { return class_type_ == kValuePairClass; }
  bool IsValueList() const { return class_type_ >= kValueListClass; }

  bool IsBaseValueList() const { return class_type_ == kValueListClass; }

  bool IsBasicShapeValue() const {
    return class_type_ >= kBasicShapeCircleClass &&
           class_type_ <= kBasicShapeXYWHClass;
  }
  bool IsBasicShapeCircleValue() const {
    return class_type_ == kBasicShapeCircleClass;
  }
  bool IsBasicShapeEllipseValue() const {
    return class_type_ == kBasicShapeEllipseClass;
  }
  bool IsBasicShapePolygonValue() const {
    return class_type_ == kBasicShapePolygonClass;
  }
  bool IsBasicShapeInsetValue() const {
    return class_type_ == kBasicShapeInsetClass;
  }
  bool IsBasicShapeRectValue() const {
    return class_type_ == kBasicShapeRectClass;
  }
  bool IsBasicShapeXYWHValue() const {
    return class_type_ == kBasicShapeXYWHClass;
  }

  bool IsBorderImageSliceValue() const {
    return class_type_ == kBorderImageSliceClass;
  }
  bool IsColorValue() const { return class_type_ == kColorClass; }
  bool IsColorMixValue() const { return class_type_ == kColorMixClass; }
  bool IsCounterValue() const { return class_type_ == kCounterClass; }
  bool IsCursorImageValue() const { return class_type_ == kCursorImageClass; }
  bool IsCrossfadeValue() const { return class_type_ == kCrossfadeClass; }
  bool IsDynamicRangeLimitMixValue() const {
    return class_type_ == kDynamicRangeLimitMixClass;
  }
  bool IsPaintValue() const { return class_type_ == kPaintClass; }
  bool IsFontFeatureValue() const { return class_type_ == kFontFeatureClass; }
  bool IsFontFamilyValue() const { return class_type_ == kFontFamilyClass; }
  bool IsFontFaceSrcValue() const { return class_type_ == kFontFaceSrcClass; }
  bool IsFontStyleRangeValue() const {
    return class_type_ == kFontStyleRangeClass;
  }
  bool IsFontVariationValue() const {
    return class_type_ == kFontVariationClass;
  }
  bool IsFunctionValue() const { return class_type_ == kFunctionClass; }
  bool IsCustomIdentValue() const { return class_type_ == kCustomIdentClass; }
  bool IsImageGeneratorValue() const {
    return class_type_ >= kCrossfadeClass &&
           class_type_ <= kConstantGradientClass;
  }
  bool IsGradientValue() const {
    return class_type_ >= kLinearGradientClass &&
           class_type_ <= kConstantGradientClass;
  }
  bool IsImageSetOptionValue() const {
    return class_type_ == kImageSetOptionClass;
  }
  bool IsImageSetTypeValue() const { return class_type_ == kImageSetTypeClass; }
  bool IsImageSetValue() const { return class_type_ == kImageSetClass; }
  bool IsImageValue() const { return class_type_ == kImageClass; }
  bool IsInheritedValue() const { return class_type_ == kInheritedClass; }
  bool IsInitialValue() const { return class_type_ == kInitialClass; }
  bool IsUnsetValue() const { return class_type_ == kUnsetClass; }
  bool IsRevertValue() const { return class_type_ == kRevertClass; }
  bool IsRevertLayerValue() const { return class_type_ == kRevertLayerClass; }
  bool IsCSSWideKeyword() const {
    return class_type_ >= kInheritedClass && class_type_ <= kRevertLayerClass;
  }
  bool IsLayoutFunctionValue() const {
    return class_type_ == kLayoutFunctionClass;
  }
  bool IsLinearGradientValue() const {
    return class_type_ == kLinearGradientClass;
  }
  bool IsPaletteMixValue() const { return class_type_ == kPaletteMixClass; }
  bool IsPathValue() const { return class_type_ == kPathClass; }
  bool IsShapeValue() const { return class_type_ == kShapeClass; }
  bool IsQuadValue() const { return class_type_ == kQuadClass; }
  bool IsRayValue() const { return class_type_ == kRayClass; }
  bool IsRadialGradientValue() const {
    return class_type_ == kRadialGradientClass;
  }
  bool IsConicGradientValue() const {
    return class_type_ == kConicGradientClass;
  }
  bool IsConstantGradientValue() const {
    return class_type_ == kConstantGradientClass;
  }
  bool IsProgressValue() const { return class_type_ == kProgressClass; }
  bool IsReflectValue() const { return class_type_ == kReflectClass; }
  bool IsShadowValue() const { return class_type_ == kShadowClass; }
  bool IsStringValue() const { return class_type_ == kStringClass; }
  bool IsSuperellipseValue() const { return class_type_ == kSuperellipseClass; }
  bool IsURIValue() const { return class_type_ == kURIClass; }
  bool IsLinearTimingFunctionValue() const {
    return class_type_ == kLinearTimingFunctionClass;
  }
  bool IsCubicBezierTimingFunctionValue() const {
    return class_type_ == kCubicBezierTimingFunctionClass;
  }
  bool IsStepsTimingFunctionValue() const {
    return class_type_ == kStepsTimingFunctionClass;
  }
  bool IsGridTemplateAreasValue() const {
    return class_type_ == kGridTemplateAreasClass;
  }
  bool IsContentDistributionValue() const {
    return class_type_ == kCSSContentDistributionClass;
  }
  bool IsUnicodeRangeValue() const { return class_type_ == kUnicodeRangeClass; }
  bool IsGridLineNamesValue() const {
    return class_type_ == kGridLineNamesClass;
  }
  bool IsUnparsedDeclaration() const {
    return class_type_ == kUnparsedDeclarationClass;
  }
  bool IsGridAutoRepeatValue() const {
    return class_type_ == kGridAutoRepeatClass;
  }
  bool IsGridIntegerRepeatValue() const {
    return class_type_ == kGridIntegerRepeatClass;
  }
  bool IsGridRepeatValue() const {
    return IsGridAutoRepeatValue() || IsGridIntegerRepeatValue();
  }
  bool IsPendingSubstitutionValue() const {
    return class_type_ == kPendingSubstitutionValueClass;
  }
  bool IsPendingSystemFontValue() const {
    return class_type_ == kPendingSystemFontValueClass;
  }
  bool IsInvalidVariableValue() const {
    return class_type_ == kInvalidVariableValueClass ||
           class_type_ == kCyclicVariableValueClass;
  }
  bool IsCyclicVariableValue() const {
    return class_type_ == kCyclicVariableValueClass;
  }
  bool IsFlipRevertValue() const { return class_type_ == kFlipRevertClass; }
  bool IsAlternateValue() const { return class_type_ == kAlternateClass; }
  bool IsAxisValue() const { return class_type_ == kAxisClass; }
  bool IsShorthandWrapperValue() const {
    return class_type_ == kKeyframeShorthandClass;
  }
  bool IsInitialColorValue() const {
    return class_type_ == kInitialColorValueClass;
  }
  bool IsLightDarkValuePair() const {
    return class_type_ == kLightDarkValuePairClass;
  }

  bool IsScrollValue() const { return class_type_ == kScrollClass; }
  bool IsViewValue() const { return class_type_ == kViewClass; }
  bool IsRatioValue() const { return class_type_ == kRatioClass; }

  bool IsRepeatStyleValue() const { return class_type_ == kRepeatStyleClass; }

  bool IsRelativeColorValue() const {
    return class_type_ == kRelativeColorClass;
  }

  // NOTE: Relative colors can also be unresolved; this is about
  // the specific case of unresolved absolute colors.
  bool IsUnresolvedColorValue() const {
    return class_type_ == kUnresolvedColorClass;
  }

  bool IsRepeatValue() const { return class_type_ == kRepeatClass; }

  bool HasFailedOrCanceledSubresources() const;
  bool MayContainUrl() const;
  void ReResolveUrl(const Document&) const;

  bool operator==(const CSSValue&) const;
  bool operator!=(const CSSValue& o) const { return !(*this == o); }

  // Returns the same CSS value, but populated with the given tree scope for
  // tree-scoped names and references.
  const CSSValue& EnsureScopedValue(const TreeScope* tree_scope) const {
    if (!needs_tree_scope_population_) {
      return *this;
    }
    return PopulateWithTreeScope(tree_scope);
  }
  bool IsScopedValue() const { return !needs_tree_scope_population_; }

#if DCHECK_IS_ON()
  WTF::String ClassTypeToString() const;
#endif

  void TraceAfterDispatch(blink::Visitor* visitor) const {}
  void Trace(Visitor*) const;

  static const size_t kValueListSeparatorBits = 2;
  enum ValueListSeparator { kSpaceSeparator, kCommaSeparator, kSlashSeparator };

 protected:
  enum ClassType {
    kNumericLiteralClass,
    kMathFunctionClass,
    kIdentifierClass,
    kScopedKeywordClass,
    kColorClass,
    kUnresolvedColorClass,
    kColorMixClass,
    kCounterClass,
    kQuadClass,
    kCustomIdentClass,
    kStringClass,
    kURIClass,
    kValuePairClass,
    kLightDarkValuePairClass,
    kScrollClass,
    kViewClass,
    kRatioClass,
    kRelativeColorClass,

    // Basic shape classes.
    // TODO(sashab): Represent these as a single subclass, BasicShapeClass.
    kBasicShapeCircleClass,
    kBasicShapeEllipseClass,
    kBasicShapePolygonClass,
    kBasicShapeInsetClass,
    kBasicShapeRectClass,
    kBasicShapeXYWHClass,

    // Image classes.
    kImageClass,
    kCursorImageClass,

    // Image generator classes.
    kCrossfadeClass,
    kPaintClass,
    kLinearGradientClass,
    kRadialGradientClass,
    kConicGradientClass,
    kConstantGradientClass,

    // Timing function classes.
    kLinearTimingFunctionClass,
    kCubicBezierTimingFunctionClass,
    kStepsTimingFunctionClass,

    kProgressClass,

    // Other class types.
    kBorderImageSliceClass,
    kDynamicRangeLimitMixClass,
    kFontFeatureClass,
    kFontFaceSrcClass,
    kFontFamilyClass,
    kFontStyleRangeClass,
    kFontVariationClass,
    kAlternateClass,

    kInheritedClass,
    kInitialClass,
    kUnsetClass,
    kRevertClass,
    kRevertLayerClass,

    kReflectClass,
    kShadowClass,
    kUnicodeRangeClass,
    kGridTemplateAreasClass,
    kPaletteMixClass,
    kPathClass,
    kShapeClass,
    kRayClass,
    kUnparsedDeclarationClass,
    kPendingSubstitutionValueClass,
    kPendingSystemFontValueClass,
    kInvalidVariableValueClass,
    kCyclicVariableValueClass,
    kFlipRevertClass,
    kLayoutFunctionClass,

    kCSSContentDistributionClass,

    kKeyframeShorthandClass,
    kInitialColorValueClass,

    kImageSetOptionClass,
    kImageSetTypeClass,

    kRepeatStyleClass,

    kSuperellipseClass,

    // List class types must appear after ValueListClass.
    kValueListClass,
    kFunctionClass,
    kImageSetClass,
    kGridLineNamesClass,
    kGridAutoRepeatClass,
    kGridIntegerRepeatClass,
    kAxisClass,
    kRepeatClass,
    // Do not append non-list class types here.
  };

  ClassType GetClassType() const { return static_cast<ClassType>(class_type_); }

  const CSSValue& PopulateWithTreeScope(const TreeScope*) const;

  explicit CSSValue(ClassType class_type)
      : allows_negative_percentage_reference_(false),
        needs_tree_scope_population_(false),
        class_type_(class_type) {}

  // NOTE: This class is non-virtual for memory and performance reasons.
  // Don't go making it virtual again unless you know exactly what you're doing!

 protected:
  // The value in this section are only used by specific subclasses but kept
  // here to maximize struct packing. If we need space for more, we could use
  // bitfields, but we don't currently (and Clang creates better code if we
  // avoid it). (This class used to be 3 and not 4 bytes, but this doesn't
  // actually save any memory, due to padding.)

  // CSSNumericLiteralValue bits:
  // This field holds CSSPrimitiveValue::UnitType.
  uint8_t numeric_literal_unit_type_ = 0;

  // CSSNumericLiteralValue bits:
  uint8_t value_list_separator_ = kSpaceSeparator;

  // CSSMathFunctionValue:
  uint8_t allows_negative_percentage_reference_ : 1;  // NOLINT

  // Any CSS value that defines/references a global name should be tree-scoped.
  // However, to allow sharing StyleSheetContents, we don't directly populate
  // CSS values with tree scope in parsed results, but wait until resolving an
  // element's style.
  // The flag is true if the value contains such references but hasn't been
  // populated with a tree scope.
  uint8_t needs_tree_scope_population_ : 1;  // NOLINT

  // Whether this value originally came from a quirksmode-specific declaration.
  // Used for use counting of such situations (to see if we can try to remove
  // the functionality).
  uint8_t was_quirky_ : 1 = false;

 private:
  const uint8_t class_type_;  // ClassType
};

template <typename CSSValueType, wtf_size_t inlineCapacity>
inline bool CompareCSSValueVector(
    const HeapVector<Member<CSSValueType>, inlineCapacity>& first_vector,
    const HeapVector<Member<CSSValueType>, inlineCapacity>& second_vector) {
  wtf_size_t size = first_vector.size();
  if (size != second_vector.size()) {
    return false;
  }

  for (wtf_size_t i = 0; i < size; i++) {
    if (!base::ValuesEquivalent(first_vector[i], second_vector[i])) {
      return false;
    }
  }
  return true;
}

}  // namespace blink

namespace cppgc {
// Assign CSSValue to be allocated on custom CSSValueSpace.
template <typename T>
struct SpaceTrait<
    T,
    std::enable_if_t<std::is_base_of<blink::CSSValue, T>::value>> {
  using Space = blink::CSSValueSpace;
};
}  // namespace cppgc

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VALUE_H_
