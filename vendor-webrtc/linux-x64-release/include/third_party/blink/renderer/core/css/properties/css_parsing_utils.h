// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_PARSING_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_PARSING_UTILS_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_anchor_query_enums.h"
#include "third_party/blink/renderer/core/css/css_custom_ident_value.h"
#include "third_party/blink/renderer/core/css/css_gap_decoration_property_enums.h"
#include "third_party/blink/renderer/core/css/css_identifier_value.h"
#include "third_party/blink/renderer/core/css/css_numeric_literal_value.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_repeat_style_value.h"
#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token_stream.h"
#include "third_party/blink/renderer/core/css/parser/css_property_parser.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/core/style/grid_area.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

namespace cssvalue {
class CSSFontFeatureValue;
class CSSScopedKeywordValue;
class CSSURIValue;
}  // namespace cssvalue
class CSSIdentifierValue;
class CSSParserContext;
class CSSParserLocalContext;
class CSSParserTokenStream;
class CSSPropertyValue;
class CSSShadowValue;
class CSSStringValue;
class CSSValue;
class CSSValueList;
class CSSValuePair;
class StylePropertyShorthand;

// "Consume" functions, when successful, should consume all the relevant tokens
// as well as any trailing whitespace. When the start of the stream doesn't
// match the type we're looking for, the position in the stream should
// not be modified.
namespace css_parsing_utils {

enum class AllowInsetAndSpread { kAllow, kForbid };
enum class AllowTextValue { kAllow, kForbid };
enum class AllowPathValue { kAllow, kForbid };
enum class AllowBasicShapeRectValue { kAllow, kForbid };
enum class AllowBasicShapeXYWHValue { kAllow, kForbid };
enum class AllowShapeValue { kAllow, kForbid };
enum class DefaultFill { kFill, kNoFill };
enum class ParsingStyle { kLegacy, kNotLegacy };
enum class TrackListType {
  kGridAuto,
  kGridTemplate,
  kGridTemplateNoRepeat,
  kGridTemplateSubgrid
};
enum class UnitlessQuirk { kAllow, kForbid };
enum class AllowCalcSize {
  kAllowWithAuto,
  kAllowWithoutAuto,
  kAllowWithAutoAndContent,
  kForbid
};
enum class AllowedColors { kAll, kAbsolute };
enum class EmptyPathStringHandling { kFailure, kTreatAsNone };

using ConsumeAnimationItemValue = CSSValue* (*)(CSSPropertyID,
                                                CSSParserTokenStream&,
                                                const CSSParserContext&,
                                                bool use_legacy_parsing);
using IsResetOnlyFunction = bool (*)(CSSPropertyID);
using IsPositionKeyword = bool (*)(CSSValueID);

constexpr size_t kMaxNumAnimationLonghands = 12;
constexpr size_t kMaxNumAnimationTriggerLonghands = 6;

void Complete4Sides(CSSValue* side[4]);

// TODO(timloh): These should probably just be consumeComma and consumeSlash.
bool ConsumeCommaIncludingWhitespace(CSSParserTokenStream&);
bool ConsumeSlashIncludingWhitespace(CSSParserTokenStream&);

// https://drafts.csswg.org/css-syntax/#typedef-any-value
//
// Consumes component values until it reaches a token that is not allowed
// for <any-value>.
CORE_EXPORT void ConsumeAnyValue(CSSParserTokenStream&);

CSSPrimitiveValue* ConsumeInteger(
    CSSParserTokenStream&,
    const CSSParserContext&,
    double minimum_value = -std::numeric_limits<double>::max(),
    const bool is_percentage_allowed = true);
CSSPrimitiveValue* ConsumeIntegerOrNumberCalc(
    CSSParserTokenStream&,
    const CSSParserContext&,
    CSSPrimitiveValue::ValueRange = CSSPrimitiveValue::ValueRange::kInteger);
CSSPrimitiveValue* ConsumePositiveInteger(CSSParserTokenStream&,
                                          const CSSParserContext&);
CSSPrimitiveValue* ConsumeNumber(CSSParserTokenStream&,
                                 const CSSParserContext&,
                                 CSSPrimitiveValue::ValueRange);
CSSPrimitiveValue* ConsumeLength(CSSParserTokenStream&,
                                 const CSSParserContext&,
                                 CSSPrimitiveValue::ValueRange,
                                 UnitlessQuirk = UnitlessQuirk::kForbid);
CSSPrimitiveValue* ConsumePercent(CSSParserTokenStream&,
                                  const CSSParserContext&,
                                  CSSPrimitiveValue::ValueRange);

// Any percentages are converted to numbers.
CSSPrimitiveValue* ConsumeNumberOrPercent(
    CSSParserTokenStream& stream,
    const CSSParserContext& context,
    CSSPrimitiveValue::ValueRange value_range);

CSSPrimitiveValue* ConsumeAlphaValue(CSSParserTokenStream&,
                                     const CSSParserContext&);
CSSPrimitiveValue* ConsumeLengthOrPercent(
    CSSParserTokenStream&,
    const CSSParserContext&,
    CSSPrimitiveValue::ValueRange,
    UnitlessQuirk = UnitlessQuirk::kForbid,
    CSSAnchorQueryTypes = kCSSAnchorQueryTypesNone,
    AllowCalcSize = AllowCalcSize::kForbid);
CSSPrimitiveValue* ConsumeSVGGeometryPropertyLength(
    CSSParserTokenStream&,
    const CSSParserContext&,
    CSSPrimitiveValue::ValueRange);

CORE_EXPORT CSSPrimitiveValue* ConsumeAngle(
    CSSParserTokenStream&,
    const CSSParserContext&,
    std::optional<WebFeature> unitless_zero_feature);
CORE_EXPORT CSSPrimitiveValue* ConsumeAngle(
    CSSParserTokenStream&,
    const CSSParserContext&,
    std::optional<WebFeature> unitless_zero_feature,
    double minimum_value,
    double maximum_value);
CSSPrimitiveValue* ConsumeTime(CSSParserTokenStream&,
                               const CSSParserContext&,
                               CSSPrimitiveValue::ValueRange);
CSSPrimitiveValue* ConsumeResolution(CSSParserTokenStream&,
                                     const CSSParserContext&);
CSSValue* ConsumeRatio(CSSParserTokenStream&, const CSSParserContext&);
CSSIdentifierValue* ConsumeIdent(CSSParserTokenStream&);
CSSIdentifierValue* ConsumeIdentRange(CSSParserTokenStream&,
                                      CSSValueID lower,
                                      CSSValueID upper);
template <CSSValueID, CSSValueID...>
inline bool IdentMatches(CSSValueID id);

template <CSSValueID... allowedIdents>
bool PeekedIdentMatches(CSSParserTokenStream&);

template <CSSValueID... allowedIdents>
CSSIdentifierValue* ConsumeIdent(CSSParserTokenStream&);

template <CSSValueID... allowedIdents>
cssvalue::CSSScopedKeywordValue* ConsumeScopedKeywordValue(
    CSSParserTokenStream&);

CSSCustomIdentValue* ConsumeCustomIdent(CSSParserTokenStream&,
                                        const CSSParserContext&);
CSSCustomIdentValue* ConsumeDashedIdent(CSSParserTokenStream&,
                                        const CSSParserContext&);
cssvalue::CSSScopedKeywordValue* ConsumeScopedKeywordValue(
    CSSParserTokenStream&);
CSSStringValue* ConsumeString(CSSParserTokenStream&);
cssvalue::CSSURIValue* ConsumeUrl(CSSParserTokenStream&,
                                  const CSSParserContext&);

// Some properties accept non-standard colors, like rgb values without a
// preceding hash, in quirks mode.
CORE_EXPORT CSSValue* ConsumeColorMaybeQuirky(CSSParserTokenStream&,
                                              const CSSParserContext&);

// https://drafts.csswg.org/css-color-5/#typedef-color
CORE_EXPORT CSSValue* ConsumeColor(CSSParserTokenStream&,
                                   const CSSParserContext&);

// https://drafts.csswg.org/css-color-5/#absolute-color
CORE_EXPORT CSSValue* ConsumeAbsoluteColor(CSSParserTokenStream&,
                                           const CSSParserContext&);

CSSValue* ConsumeLineWidth(CSSParserTokenStream&,
                           const CSSParserContext&,
                           UnitlessQuirk);

CSSValuePair* ConsumePosition(CSSParserTokenStream&,
                              const CSSParserContext&,
                              UnitlessQuirk,
                              std::optional<WebFeature> three_value_position);
bool ConsumePosition(CSSParserTokenStream&,
                     const CSSParserContext&,
                     UnitlessQuirk,
                     std::optional<WebFeature> three_value_position,
                     CSSValue*& result_x,
                     CSSValue*& result_y);
bool ConsumeOneOrTwoValuedPosition(CSSParserTokenStream&,
                                   const CSSParserContext&,
                                   UnitlessQuirk,
                                   CSSValue*& result_x,
                                   CSSValue*& result_y);
bool ConsumeBorderShorthand(CSSParserTokenStream&,
                            const CSSParserContext&,
                            const CSSParserLocalContext&,
                            const CSSValue*& result_width,
                            const CSSValue*& result_style,
                            const CSSValue*& result_color);

enum class ConsumeGeneratedImagePolicy { kAllow, kForbid };
enum class ConsumeStringUrlImagePolicy { kAllow, kForbid };
enum class ConsumeImageSetImagePolicy { kAllow, kForbid };

CSSValue* ConsumeImage(
    CSSParserTokenStream&,
    const CSSParserContext&,
    const ConsumeGeneratedImagePolicy = ConsumeGeneratedImagePolicy::kAllow,
    const ConsumeStringUrlImagePolicy = ConsumeStringUrlImagePolicy::kForbid,
    const ConsumeImageSetImagePolicy = ConsumeImageSetImagePolicy::kAllow);
CSSValue* ConsumeImageOrNone(CSSParserTokenStream&, const CSSParserContext&);

CSSValue* ConsumeAxis(CSSParserTokenStream&, const CSSParserContext& context);

// Syntax: none | <length> | auto && <length> | auto && none
// If this returns a CSSIdentifierValue then it is "none"
// Otherwise, this returns a list of 1 or 2 elements for the rest of the syntax
CSSValue* ConsumeIntrinsicSizeLonghand(CSSParserTokenStream&,
                                       const CSSParserContext&);

CSSIdentifierValue* ConsumeShapeBox(CSSParserTokenStream&);
CSSIdentifierValue* ConsumeVisualBox(CSSParserTokenStream&);

CSSIdentifierValue* ConsumeCoordBox(CSSParserTokenStream&);

CSSIdentifierValue* ConsumeGeometryBox(CSSParserTokenStream&);

enum class IsImplicitProperty { kNotImplicit, kImplicit };

void AddProperty(CSSPropertyID resolved_property,
                 CSSPropertyID current_shorthand,
                 const CSSValue&,
                 bool important,
                 IsImplicitProperty,
                 HeapVector<CSSPropertyValue, 64>& properties);

void CountKeywordOnlyPropertyUsage(CSSPropertyID,
                                   const CSSParserContext&,
                                   CSSValueID);

void WarnInvalidKeywordPropertyUsage(CSSPropertyID,
                                     const CSSParserContext&,
                                     CSSValueID);

const CSSValue* ParseLonghand(CSSPropertyID unresolved_property,
                              CSSPropertyID current_shorthand,
                              const CSSParserContext&,
                              CSSParserTokenStream&);

bool ConsumeShorthandVia2Longhands(
    const StylePropertyShorthand&,
    bool important,
    const CSSParserContext&,
    CSSParserTokenStream&,
    HeapVector<CSSPropertyValue, 64>& properties);

bool ConsumeShorthandVia4Longhands(
    const StylePropertyShorthand&,
    bool important,
    const CSSParserContext&,
    CSSParserTokenStream&,
    HeapVector<CSSPropertyValue, 64>& properties);

bool ConsumeShorthandGreedilyViaLonghands(
    const StylePropertyShorthand&,
    bool important,
    const CSSParserContext&,
    CSSParserTokenStream&,
    HeapVector<CSSPropertyValue, 64>& properties,
    bool use_initial_value_function = false);

void AddExpandedPropertyForValue(CSSPropertyID prop_id,
                                 const CSSValue&,
                                 bool,
                                 HeapVector<CSSPropertyValue, 64>& properties);

CSSValue* ConsumeTransformValue(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeTransformList(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeFilterFunctionList(CSSParserTokenStream&,
                                    const CSSParserContext&);

bool IsBaselineKeyword(CSSValueID id);
bool IsSelfPositionKeyword(CSSValueID);
bool IsSelfPositionOrLeftOrRightKeyword(CSSValueID);
bool IsContentPositionKeyword(CSSValueID);
bool IsContentPositionOrLeftOrRightKeyword(CSSValueID);
CORE_EXPORT bool IsCSSWideKeyword(CSSValueID);
CORE_EXPORT bool IsCSSWideKeyword(StringView);
bool IsRevertKeyword(StringView);
bool IsDefaultKeyword(StringView);
bool IsHashIdentifier(const CSSParserToken&);
CORE_EXPORT bool IsDashedIdent(const CSSParserToken&);

// https://drafts.csswg.org/css-mixins-1/#function-rule
inline bool IsDashedFunctionName(const CSSParserToken& token) {
  return token.GetType() == kFunctionToken && token.Value().length() >= 3 &&
         token.Value()[0] == '-' && token.Value()[1] == '-';
}

CSSValue* ConsumeCSSWideKeyword(CSSParserTokenStream&);

// This function returns false for CSS-wide keywords, 'default', and any
// template parameters provided.
//
// https://drafts.csswg.org/css-values-4/#identifier-value
template <CSSValueID, CSSValueID...>
bool IsCustomIdent(CSSValueID);

// https://drafts.csswg.org/scroll-animations-1/#typedef-timeline-name
bool IsTimelineName(const CSSParserToken&);

CSSValue* ConsumeSelfPositionOverflowPosition(CSSParserTokenStream&,
                                              IsPositionKeyword);
CSSValue* ConsumeContentDistributionOverflowPosition(CSSParserTokenStream&,
                                                     IsPositionKeyword);

CSSValue* ConsumeAnimationIterationCount(CSSParserTokenStream&,
                                         const CSSParserContext&);
CSSValue* ConsumeAnimationName(CSSParserTokenStream&,
                               const CSSParserContext&,
                               bool allow_quoted_name);
CSSValue* ConsumeAnimationTimeline(CSSParserTokenStream&,
                                   const CSSParserContext&);
CSSValue* ConsumeAnimationTimingFunction(CSSParserTokenStream&,
                                         const CSSParserContext&);
CSSValue* ConsumeAnimationDuration(CSSParserTokenStream&,
                                   const CSSParserContext&);
// https://drafts.csswg.org/scroll-animations-1/#typedef-timeline-range-name
CSSValue* ConsumeTimelineRangeName(CSSParserTokenStream&);
CSSValue* ConsumeTimelineRangeNameAndPercent(CSSParserTokenStream&,
                                             const CSSParserContext&);
CSSValue* ConsumeAnimationDelay(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeAnimationRange(CSSParserTokenStream&,
                                const CSSParserContext&,
                                double default_offset_percent,
                                bool allow_auto);

bool ConsumeAnimationShorthand(
    const StylePropertyShorthand&,
    HeapVector<Member<CSSValueList>, kMaxNumAnimationLonghands>&,
    ConsumeAnimationItemValue,
    IsResetOnlyFunction,
    CSSParserTokenStream&,
    const CSSParserContext&,
    bool use_legacy_parsing);

CSSValue* ConsumeSingleTimelineAxis(CSSParserTokenStream&);
CSSValue* ConsumeSingleTimelineName(CSSParserTokenStream&,
                                    const CSSParserContext&);
CSSValue* ConsumeSingleTimelineInset(CSSParserTokenStream&,
                                     const CSSParserContext&);

bool ConsumeAnimationTriggerShorthand(const StylePropertyShorthand&,
                                      HeapVector<Member<CSSValueList>, kMaxNumAnimationTriggerLonghands>&,
                                      CSSParserTokenStream&,
                                      const CSSParserContext&);

CSSValue* GetImpliedRangeEnd(const CSSValue* start_range);

void AddBackgroundValue(CSSValue*& list, const CSSValue*);
CSSValue* ConsumeBackgroundAttachment(CSSParserTokenStream&);
CSSValue* ConsumeBackgroundBlendMode(CSSParserTokenStream&);
CSSValue* ConsumeBackgroundBox(CSSParserTokenStream&);
CSSValue* ConsumeBackgroundBoxOrText(CSSParserTokenStream&);
CSSValue* ConsumeMaskComposite(CSSParserTokenStream&);
CSSValue* ConsumePrefixedMaskComposite(CSSParserTokenStream&);
CSSValue* ConsumeMaskMode(CSSParserTokenStream&);
bool ConsumeBackgroundPosition(CSSParserTokenStream&,
                               const CSSParserContext&,
                               UnitlessQuirk,
                               std::optional<WebFeature> three_value_position,
                               const CSSValue*& result_x,
                               const CSSValue*& result_y);
CSSValue* ConsumePrefixedBackgroundBox(CSSParserTokenStream&, AllowTextValue);
CSSValue* ParseBackgroundBox(CSSParserTokenStream&,
                             const CSSParserLocalContext&,
                             AllowTextValue alias_allow_text_value);
CSSValue* ParseBackgroundSize(CSSParserTokenStream&,
                              const CSSParserContext&,
                              const CSSParserLocalContext&,
                              std::optional<WebFeature> negative_size);
CSSValue* ParseMaskSize(CSSParserTokenStream&,
                        const CSSParserContext&,
                        const CSSParserLocalContext&,
                        std::optional<WebFeature> negative_size);
bool ParseBackgroundOrMask(bool,
                           CSSParserTokenStream&,
                           const CSSParserContext&,
                           const CSSParserLocalContext&,
                           HeapVector<CSSPropertyValue, 64>&);

CORE_EXPORT CSSValue* ConsumeProgressType(CSSParserTokenStream&,
                                          const CSSParserContext&);

CSSValue* ConsumeCoordBoxOrNoClip(CSSParserTokenStream&);

CSSRepeatStyleValue* ConsumeRepeatStyleValue(CSSParserTokenStream& stream);
CSSValueList* ParseRepeatStyle(CSSParserTokenStream& stream);

CSSValue* ConsumeWebkitBorderImage(CSSParserTokenStream&,
                                   const CSSParserContext&);
bool ConsumeBorderImageComponents(CSSParserTokenStream&,
                                  const CSSParserContext&,
                                  CSSValue*& source,
                                  CSSValue*& slice,
                                  CSSValue*& width,
                                  CSSValue*& outset,
                                  CSSValue*& repeat,
                                  DefaultFill);
CSSValue* ConsumeBorderImageRepeat(CSSParserTokenStream&);
CSSValue* ConsumeBorderImageSlice(CSSParserTokenStream&,
                                  const CSSParserContext&,
                                  DefaultFill);
CSSValue* ConsumeBorderImageWidth(CSSParserTokenStream&,
                                  const CSSParserContext&);
CSSValue* ConsumeBorderImageOutset(CSSParserTokenStream&,
                                   const CSSParserContext&);

CSSValue* ParseBorderRadiusCorner(CSSParserTokenStream&,
                                  const CSSParserContext&);
CSSValue* ParseBorderWidthSide(CSSParserTokenStream&,
                               const CSSParserContext&,
                               const CSSParserLocalContext&);
const CSSValue* ParseBorderStyleSide(CSSParserTokenStream&,
                                     const CSSParserContext&);

CSSValue* ConsumeCornerShape(CSSParserTokenStream&, const CSSParserContext&);

CSSValue* ConsumeGapDecorationPropertyList(CSSParserTokenStream&,
                                           const CSSParserContext&,
                                           const CSSGapDecorationPropertyType);

CSSValue* ConsumeShadow(CSSParserTokenStream&,
                        const CSSParserContext&,
                        AllowInsetAndSpread);
CSSShadowValue* ParseSingleShadow(CSSParserTokenStream&,
                                  const CSSParserContext&,
                                  AllowInsetAndSpread);

CSSValue* ConsumeColumnCount(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeColumnLength(CSSParserTokenStream&, const CSSParserContext&);
bool ConsumeColumnWidthOrCount(CSSParserTokenStream&,
                               const CSSParserContext&,
                               CSSValue*&,
                               CSSValue*&);
CSSValue* ConsumeGapLength(CSSParserTokenStream&, const CSSParserContext&);

CSSValue* ConsumeCounter(CSSParserTokenStream&, const CSSParserContext&, int);

CSSValue* ConsumeFontSize(CSSParserTokenStream&,
                          const CSSParserContext&,
                          UnitlessQuirk = UnitlessQuirk::kForbid);

CSSValue* ConsumeLineHeight(CSSParserTokenStream&, const CSSParserContext&);

CSSValue* ConsumeMathDepth(CSSParserTokenStream& stream,
                           const CSSParserContext& context);

CSSValue* ConsumeFontPalette(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumePaletteMixFunction(CSSParserTokenStream&,
                                    const CSSParserContext&);
CSSValueList* ConsumeFontFamily(CSSParserTokenStream&);
CSSValueList* ConsumeNonGenericFamilyNameList(CSSParserTokenStream& stream);
CSSValue* ConsumeGenericFamily(CSSParserTokenStream&);
CSSValue* ConsumeFamilyName(CSSParserTokenStream&);
String ConcatenateFamilyName(CSSParserTokenStream&);
CSSIdentifierValue* ConsumeFontStretchKeywordOnly(CSSParserTokenStream&,
                                                  const CSSParserContext&);
CSSValue* ConsumeFontStretch(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeFontStyle(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeFontWeight(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeFontFeatureSettings(CSSParserTokenStream&,
                                     const CSSParserContext&);
cssvalue::CSSFontFeatureValue* ConsumeFontFeatureTag(CSSParserTokenStream&,
                                                     const CSSParserContext&);
CSSIdentifierValue* ConsumeFontVariantCSS21(CSSParserTokenStream&);
CSSIdentifierValue* ConsumeFontTechIdent(CSSParserTokenStream&);
CSSIdentifierValue* ConsumeFontFormatIdent(CSSParserTokenStream&);
CSSValueID FontFormatToId(String);
bool IsSupportedKeywordTech(CSSValueID keyword);
bool IsSupportedKeywordFormat(CSSValueID keyword);

CSSValue* ConsumeGridLine(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeGridTrackList(CSSParserTokenStream&,
                               const CSSParserContext&,
                               TrackListType);
bool ParseGridTemplateAreasRow(const WTF::String& grid_row_names,
                               NamedGridAreaMap&,
                               const wtf_size_t row_count,
                               wtf_size_t& column_count);
CSSValue* ConsumeGridTemplatesRowsOrColumns(CSSParserTokenStream&,
                                            const CSSParserContext&);
bool ConsumeGridItemPositionShorthand(bool important,
                                      CSSParserTokenStream&,
                                      const CSSParserContext&,
                                      CSSValue*& start_value,
                                      CSSValue*& end_value);
bool ConsumeGridTemplateShorthand(bool important,
                                  CSSParserTokenStream&,
                                  const CSSParserContext&,
                                  const CSSValue*& template_rows,
                                  const CSSValue*& template_columns,
                                  const CSSValue*& template_areas);

CSSValue* ConsumeItemTolerance(CSSParserTokenStream&, const CSSParserContext&);

CSSValue* ConsumeHyphenateLimitChars(CSSParserTokenStream&,
                                     const CSSParserContext&);

// The fragmentation spec says that page-break-(after|before|inside) are to be
// treated as shorthands for their break-(after|before|inside) counterparts.
// We'll do the same for the non-standard properties
// -webkit-column-break-(after|before|inside).
bool ConsumeFromPageBreakBetween(CSSParserTokenStream&, CSSValueID&);
bool ConsumeFromColumnBreakBetween(CSSParserTokenStream&, CSSValueID&);
bool ConsumeFromColumnOrPageBreakInside(CSSParserTokenStream&, CSSValueID&);

bool ValidWidthOrHeightKeyword(CSSValueID id, const CSSParserContext& context);

CSSValue* ConsumeMaxWidthOrHeight(CSSParserTokenStream&,
                                  const CSSParserContext&,
                                  UnitlessQuirk = UnitlessQuirk::kForbid);
CSSValue* ConsumeWidthOrHeight(CSSParserTokenStream&,
                               const CSSParserContext&,
                               UnitlessQuirk = UnitlessQuirk::kForbid);

CSSValue* ConsumeMarginOrOffset(CSSParserTokenStream&,
                                const CSSParserContext&,
                                UnitlessQuirk,
                                CSSAnchorQueryTypes = kCSSAnchorQueryTypesNone);
CSSValue* ConsumeScrollPadding(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeScrollStart(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeOffsetPath(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumePathOrNone(CSSParserTokenStream&);
CSSValue* ConsumeOffsetRotate(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeInitialLetter(CSSParserTokenStream&, const CSSParserContext&);

CSSValue* ConsumeBasicShape(
    CSSParserTokenStream&,
    const CSSParserContext&,
    AllowPathValue = AllowPathValue::kAllow,
    AllowShapeValue = AllowShapeValue::kAllow,
    AllowBasicShapeRectValue = AllowBasicShapeRectValue::kAllow,
    AllowBasicShapeXYWHValue = AllowBasicShapeXYWHValue::kAllow);
bool ConsumeRadii(std::array<CSSValue*, 4>& horizontal_radii,
                  std::array<CSSValue*, 4>& vertical_radii,
                  CSSParserTokenStream& stream,
                  const CSSParserContext& context,
                  bool use_legacy_parsing);
bool ConsumeCornerShapes(std::array<CSSValue*, 4>& shapes,
                         CSSParserTokenStream& stream,
                         const CSSParserContext& context);

CSSValue* ConsumeTextDecorationLine(CSSParserTokenStream&);
CSSValue* ConsumeTextBoxEdge(CSSParserTokenStream&);
CSSValue* ConsumeTextBoxTrim(CSSParserTokenStream&);

// Consume the `autospace` production.
// https://drafts.csswg.org/css-text-4/#typedef-autospace
CSSValue* ConsumeAutospace(CSSParserTokenStream&);
// Consume the `spacing-trim` production.
// https://drafts.csswg.org/css-text-4/#typedef-spacing-trim
CSSValue* ConsumeSpacingTrim(CSSParserTokenStream&);

CSSValue* ConsumeTransformValue(CSSParserTokenStream&,
                                const CSSParserContext&,
                                bool use_legacy_parsing);
CSSValue* ConsumeTransformList(CSSParserTokenStream&,
                               const CSSParserContext&,
                               const CSSParserLocalContext&);
CSSValue* ConsumeTransitionProperty(CSSParserTokenStream&,
                                    const CSSParserContext&);
bool IsValidPropertyList(const CSSValueList&);
bool IsValidTransitionBehavior(const CSSValueID&);
bool IsValidTransitionBehaviorList(const CSSValueList&);

CSSValue* ConsumeBorderColorSide(CSSParserTokenStream&,
                                 const CSSParserContext&,
                                 const CSSParserLocalContext&);
CSSValue* ConsumeBorderWidth(CSSParserTokenStream&,
                             const CSSParserContext&,
                             UnitlessQuirk);
CSSValue* ConsumeSVGPaint(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ParseSpacing(CSSParserTokenStream&, const CSSParserContext&);

CSSValue* ConsumeSingleContainerName(CSSParserTokenStream&,
                                     const CSSParserContext&);
CSSValue* ConsumeContainerName(CSSParserTokenStream&, const CSSParserContext&);
CSSValue* ConsumeContainerType(CSSParserTokenStream&, const CSSParserContext&);

UnitlessQuirk UnitlessUnlessShorthand(const CSSParserLocalContext&);

// https://drafts.csswg.org/css-counter-styles-3/#typedef-counter-style-name
CSSCustomIdentValue* ConsumeCounterStyleName(CSSParserTokenStream&,
                                             const CSSParserContext&);
AtomicString ConsumeCounterStyleNameInPrelude(CSSParserTokenStream&,
                                              const CSSParserContext&);

CSSValue* ConsumeFontSizeAdjust(CSSParserTokenStream&, const CSSParserContext&);

// When parsing a counter style name, it should be ASCII lowercased if it's an
// ASCII case-insensitive match of any predefined counter style name.
bool ShouldLowerCaseCounterStyleNameOnParse(const AtomicString&,
                                            const CSSParserContext&);

// https://drafts.csswg.org/css-anchor-position-1/#typedef-position-area
CSSValue* ConsumePositionArea(CSSParserTokenStream&);

// position-area can take one or two keywords. If the second is omitted, either
// the first is repeated, or the second is span-all. This method returns true if
// the omitted value should be the first one repeated.
bool IsRepeatedPositionAreaValue(CSSValueID value_id);

// Template implementations are at the bottom of the file for readability.

template <typename... emptyBaseCase>
inline bool IdentMatches(CSSValueID id) {
  return false;
}
template <CSSValueID head, CSSValueID... tail>
inline bool IdentMatches(CSSValueID id) {
  return id == head || IdentMatches<tail...>(id);
}

template <CSSValueID... allowedIdents>
bool PeekedIdentMatches(CSSParserTokenStream& stream) {
  return stream.Peek().GetType() == kIdentToken &&
         IdentMatches<allowedIdents...>(stream.Peek().Id());
}

template <typename...>
bool IsCustomIdent(CSSValueID id) {
  return !IsCSSWideKeyword(id) && id != CSSValueID::kDefault;
}

template <CSSValueID head, CSSValueID... tail>
bool IsCustomIdent(CSSValueID id) {
  return id != head && IsCustomIdent<tail...>(id);
}

template <CSSValueID... names>
CSSIdentifierValue* ConsumeIdent(CSSParserTokenStream& stream) {
  if (!PeekedIdentMatches<names...>(stream)) {
    return nullptr;
  }
  return CSSIdentifierValue::Create(stream.ConsumeIncludingWhitespace().Id());
}

template <CSSValueID... names>
cssvalue::CSSScopedKeywordValue* ConsumeScopedKeywordValue(
    CSSParserTokenStream& stream) {
  if (!PeekedIdentMatches<names...>(stream)) {
    return nullptr;
  }
  return ConsumeScopedKeywordValue(stream);
}

// ConsumeCommaSeparatedList and ConsumeSpaceSeparatedList take a callback
// function to call on each item in the list, followed by the arguments to pass
// to this callback.  The first argument to the callback must be the
// CSSParserTokenStream
template <typename Func, typename... Args>
CSSValueList* ConsumeCommaSeparatedList(Func callback,
                                        CSSParserTokenStream& stream,
                                        Args&&... args) {
  CSSValueList* list = CSSValueList::CreateCommaSeparated();
  do {
    CSSValue* value = callback(stream, std::forward<Args>(args)...);
    if (!value) {
      return nullptr;
    }
    list->Append(*value);
  } while (ConsumeCommaIncludingWhitespace(stream));
  DCHECK(list->length());
  return list;
}

template <typename Func, typename... Args>
CSSValueList* ConsumeSpaceSeparatedList(Func callback,
                                        CSSParserTokenStream& stream,
                                        Args&&... args) {
  CSSValueList* list = CSSValueList::CreateSpaceSeparated();
  do {
    CSSValue* value = callback(stream, std::forward<Args>(args)...);
    if (!value) {
      return list->length() > 0 ? list : nullptr;
    }
    list->Append(*value);
  } while (!stream.AtEnd());
  DCHECK(list->length());
  return list;
}

template <CSSValueID start, CSSValueID end>
CSSValue* ConsumePositionLonghand(CSSParserTokenStream& stream,
                                  const CSSParserContext& context) {
  if (stream.Peek().GetType() == kIdentToken) {
    CSSValueID id = stream.Peek().Id();
    int percent;
    if (id == start) {
      percent = 0;
    } else if (id == CSSValueID::kCenter) {
      percent = 50;
    } else if (id == end) {
      percent = 100;
    } else {
      return nullptr;
    }
    stream.ConsumeIncludingWhitespace();
    return CSSNumericLiteralValue::Create(
        percent, CSSPrimitiveValue::UnitType::kPercentage);
  }
  return ConsumeLengthOrPercent(stream, context,
                                CSSPrimitiveValue::ValueRange::kAll);
}

inline bool AtIdent(const CSSParserToken& token, const char* ident) {
  return token.GetType() == kIdentToken &&
         EqualIgnoringASCIICase(token.Value(), ident);
}

inline bool ConsumeIfIdent(CSSParserTokenStream& stream, const char* ident) {
  if (!AtIdent(stream.Peek(), ident)) {
    return false;
  }
  stream.ConsumeIncludingWhitespace();
  return true;
}

inline bool AtDelimiter(const CSSParserToken& token, UChar c) {
  return token.GetType() == kDelimiterToken && token.Delimiter() == c;
}

inline bool ConsumeIfDelimiter(CSSParserTokenStream& stream, UChar c) {
  if (!AtDelimiter(stream.Peek(), c)) {
    return false;
  }
  stream.ConsumeIncludingWhitespace();
  return true;
}

CORE_EXPORT CSSValue* ConsumeSinglePositionTryFallback(CSSParserTokenStream&,
                                                       const CSSParserContext&);
CORE_EXPORT CSSValue* ConsumePositionTryFallbacks(CSSParserTokenStream&,
                                                  const CSSParserContext&);

// If the stream starts with “!important”, consumes it and returns true.
// If the stream is at EOF, returns false.
// If parse error, also returns false, but the stream position is unchanged
// and thus guaranteed to not be at EOF.
//
// The typical usage pattern for this is: Call the function,
// then immediately check stream.AtEnd(). If stream.AtEnd(), then
// the parse succeeded and you can use the return value for whether
// the property is important or not. However, if !stream.AtEnd(),
// there has been a parse error (e.g. random junk that was not
// !important, or !important but with more tokens afterwards).
//
// If allow_important_annotation is false, just consumes whitespace
// and returns false. The same pattern as above holds.
bool MaybeConsumeImportant(CSSParserTokenStream& stream,
                           bool allow_important_annotation);

// Return true if env(safe-inset-area-bottom [, ...]) exists.
bool ContainsSafeAreaInsetBottom(StringView string);

// Return true if it's a simple sum of literals, e.g. "0px + 42em - 13cqw" or
// calc(10px + 42em - 13cqw).
bool IsSimpleSum(StringView string);

}  // namespace css_parsing_utils
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_PARSING_UTILS_H_
