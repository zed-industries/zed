// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_MATH_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_MATH_SUPPORT_H_

#include <optional>

#include "third_party/blink/renderer/platform/fonts/opentype/open_type_math_stretch_data.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class HarfBuzzFace;

class PLATFORM_EXPORT OpenTypeMathSupport {
 public:
  static bool HasMathData(const HarfBuzzFace*);

  // These constants are defined in the OpenType MATH table:
  // https://docs.microsoft.com/en-us/typography/opentype/spec/math#mathconstants-table
  // Their values match the indices in the MathConstants subtable.
  enum MathConstants {
    kScriptPercentScaleDown = 0,
    kScriptScriptPercentScaleDown = 1,
    kDelimitedSubFormulaMinHeight = 2,
    kDisplayOperatorMinHeight = 3,
    kMathLeading = 4,
    kAxisHeight = 5,
    kAccentBaseHeight = 6,
    kFlattenedAccentBaseHeight = 7,
    kSubscriptShiftDown = 8,
    kSubscriptTopMax = 9,
    kSubscriptBaselineDropMin = 10,
    kSuperscriptShiftUp = 11,
    kSuperscriptShiftUpCramped = 12,
    kSuperscriptBottomMin = 13,
    kSuperscriptBaselineDropMax = 14,
    kSubSuperscriptGapMin = 15,
    kSuperscriptBottomMaxWithSubscript = 16,
    kSpaceAfterScript = 17,
    kUpperLimitGapMin = 18,
    kUpperLimitBaselineRiseMin = 19,
    kLowerLimitGapMin = 20,
    kLowerLimitBaselineDropMin = 21,
    kStackTopShiftUp = 22,
    kStackTopDisplayStyleShiftUp = 23,
    kStackBottomShiftDown = 24,
    kStackBottomDisplayStyleShiftDown = 25,
    kStackGapMin = 26,
    kStackDisplayStyleGapMin = 27,
    kStretchStackTopShiftUp = 28,
    kStretchStackBottomShiftDown = 29,
    kStretchStackGapAboveMin = 30,
    kStretchStackGapBelowMin = 31,
    kFractionNumeratorShiftUp = 32,
    kFractionNumeratorDisplayStyleShiftUp = 33,
    kFractionDenominatorShiftDown = 34,
    kFractionDenominatorDisplayStyleShiftDown = 35,
    kFractionNumeratorGapMin = 36,
    kFractionNumDisplayStyleGapMin = 37,
    kFractionRuleThickness = 38,
    kFractionDenominatorGapMin = 39,
    kFractionDenomDisplayStyleGapMin = 40,
    kSkewedFractionHorizontalGap = 41,
    kSkewedFractionVerticalGap = 42,
    kOverbarVerticalGap = 43,
    kOverbarRuleThickness = 44,
    kOverbarExtraAscender = 45,
    kUnderbarVerticalGap = 46,
    kUnderbarRuleThickness = 47,
    kUnderbarExtraDescender = 48,
    kRadicalVerticalGap = 49,
    kRadicalDisplayStyleVerticalGap = 50,
    kRadicalRuleThickness = 51,
    kRadicalExtraAscender = 52,
    kRadicalKernBeforeDegree = 53,
    kRadicalKernAfterDegree = 54,
    kRadicalDegreeBottomRaisePercent = 55
  };

  // Returns the value of the requested math constant or null if the font does
  // not have any OpenType MATH table. All values are 16.16 fixed-point values
  // converted to float except percentages (kScriptPercentScaleDown,
  // kScriptScriptPercentScaleDown and kRadicalDegreeBottomRaisePercent) which
  // are represented by a number between 0 and 1.
  // https://docs.microsoft.com/en-us/typography/opentype/spec/math#mathconstants-table
  static std::optional<float> MathConstant(const HarfBuzzFace*, MathConstants);

  // Returns the italic correction corresponding to the specified glyph or null
  // if the font does not have any OpenType MATH table. This value provides an
  // estimation of how much the glyph is slanted, which can be used e.g. when
  // attaching scripts to the glyph.
  // https://docs.microsoft.com/en-us/typography/opentype/spec/math#mathitalicscorrectioninfo-table
  static std::optional<float> MathItalicCorrection(const HarfBuzzFace*, Glyph);

  // Returns a vector of GlyphVariantRecords corresponding to the specified
  // glyph and stretch axis. The base glyph is always added as the first item.
  // https://docs.microsoft.com/en-us/typography/opentype/spec/math#mathvariants-table
  static Vector<OpenTypeMathStretchData::GlyphVariantRecord>
  GetGlyphVariantRecords(const HarfBuzzFace*,
                         Glyph base_glyph,
                         OpenTypeMathStretchData::StretchAxis);

  // Returns a vector of GlyphPartRecords corresponding to the specified
  // glyph and stretch axis or an empty vector if there is no such construction.
  // If the italic_correction parameter is specified and a construction is
  // available, then it is set to the italic correction of the glyph assembly.
  // https://docs.microsoft.com/en-us/typography/opentype/spec/math#mathvariants-table
  static Vector<OpenTypeMathStretchData::GlyphPartRecord> GetGlyphPartRecords(
      const HarfBuzzFace*,
      Glyph base_glyph,
      OpenTypeMathStretchData::StretchAxis,
      float* italic_correction = nullptr);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_MATH_SUPPORT_H_
