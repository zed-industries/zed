/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_BUILDER_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_BUILDER_CONVERTER_H_

#include <optional>

#include "base/check_op.h"
#include "base/memory/scoped_refptr.h"
#include "base/notreached.h"
#include "cc/input/scroll_snap_data.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_function_value.h"
#include "third_party/blink/renderer/core/css/css_identifier_value.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_string_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css/css_value_pair.h"
#include "third_party/blink/renderer/core/css/css_variable_data.h"
#include "third_party/blink/renderer/core/css/resolver/style_resolver_state.h"
#include "third_party/blink/renderer/core/css/style_sheet_contents.h"
#include "third_party/blink/renderer/core/style/basic_shapes.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/grid_area.h"
#include "third_party/blink/renderer/core/style/named_grid_lines_map.h"
#include "third_party/blink/renderer/core/style/ordered_named_grid_lines.h"
#include "third_party/blink/renderer/core/style/shadow_list.h"
#include "third_party/blink/renderer/core/style/style_anchor_scope.h"
#include "third_party/blink/renderer/core/style/style_offset_rotation.h"
#include "third_party/blink/renderer/core/style/style_overflow_clip_margin.h"
#include "third_party/blink/renderer/core/style/style_reflection.h"
#include "third_party/blink/renderer/core/style/style_view_transition_group.h"
#include "third_party/blink/renderer/core/style/style_view_transition_name.h"
#include "third_party/blink/renderer/core/style/transform_origin.h"
#include "third_party/blink/renderer/platform/fonts/font_description.h"
#include "third_party/blink/renderer/platform/fonts/font_variant_emoji.h"
#include "third_party/blink/renderer/platform/geometry/length_size.h"
#include "third_party/blink/renderer/platform/graphics/image_orientation.h"
#include "third_party/blink/renderer/platform/text/quotes_data.h"
#include "third_party/blink/renderer/platform/text/tab_size.h"
#include "third_party/blink/renderer/platform/transforms/rotation.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"

namespace blink {

class ClipPathOperation;
class CSSToLengthConversionData;
class Font;
class FontBuilder;
class OffsetPathOperation;
class RotateTransformOperation;
class ScaleTransformOperation;
class ScopedCSSName;
class StyleAutoColor;
class StylePath;
class StyleResolverState;
class StyleSVGResource;
class TextSizeAdjust;
class TranslateTransformOperation;
class UnzoomedLength;
struct ComputedGridTrackList;

class StyleBuilderConverterBase {
  STATIC_ONLY(StyleBuilderConverterBase);

 public:
  static FontSelectionValue ConvertFontStretch(const CSSLengthResolver&,
                                               const CSSValue&);
  static FontSelectionValue ConvertFontStyle(const CSSLengthResolver&,
                                             const CSSValue&);
  static FontSelectionValue ConvertFontWeight(const CSSLengthResolver&,
                                              const CSSValue&,
                                              FontSelectionValue);
  static FontDescription::FontVariantCaps ConvertFontVariantCaps(
      const CSSValue&);
  static FontDescription::FamilyDescription ConvertFontFamily(
      const CSSValue&,
      FontBuilder*,
      const Document* document_for_count);
  static FontDescription::Size ConvertFontSize(
      const CSSValue&,
      const CSSToLengthConversionData&,
      FontDescription::Size parent_size,
      const Document*);
  static FontSizeAdjust ConvertFontSizeAdjust(const StyleResolverState&,
                                              const CSSValue&);
  static scoped_refptr<FontPalette> ConvertFontPalette(const CSSLengthResolver&,
                                                       const CSSValue&);
  static scoped_refptr<FontPalette> ConvertPaletteMix(const CSSLengthResolver&,
                                                      const CSSValue&);
};

// Note that we assume the parser only allows valid CSSValue types.
class StyleBuilderConverter {
  STATIC_ONLY(StyleBuilderConverter);

 public:
  static StyleReflection* ConvertBoxReflect(StyleResolverState&,
                                            const CSSValue&);
  template <typename T>
  static T ConvertComputedLength(const StyleResolverState&, const CSSValue&);
  static LengthBox ConvertClip(StyleResolverState&, const CSSValue&);
  static ClipPathOperation* ConvertClipPath(StyleResolverState&,
                                            const CSSValue&);
  static DynamicRangeLimit ConvertDynamicRangeLimit(const StyleResolverState&,
                                                    const CSSValue&);
  static StyleSVGResource* ConvertElementReference(StyleResolverState&,
                                                   const CSSValue&,
                                                   CSSPropertyID);
  static FilterOperations ConvertFilterOperations(StyleResolverState&,
                                                  const CSSValue&,
                                                  CSSPropertyID);
  static FilterOperations ConvertOffscreenFilterOperations(const CSSValue&,
                                                           const Font*);
  // The template parameter ZeroValue indicates which CSSValueID should be
  // converted to zero.
  template <typename T, CSSValueID ZeroValue = CSSValueID::kNone>
  static T ConvertFlags(StyleResolverState&, const CSSValue&);
  static FontDescription::FamilyDescription ConvertFontFamily(
      StyleResolverState&,
      const CSSValue&);
  static scoped_refptr<FontFeatureSettings> ConvertFontFeatureSettings(
      StyleResolverState&,
      const CSSValue&);
  static scoped_refptr<FontVariationSettings> ConvertFontVariationSettings(
      const StyleResolverState&,
      const CSSValue&);
  static scoped_refptr<FontPalette> ConvertFontPalette(
      StyleResolverState& state,
      const CSSValue& value);
  static FontDescription::Size ConvertFontSize(StyleResolverState&,
                                               const CSSValue&);
  static FontSizeAdjust ConvertFontSizeAdjust(StyleResolverState&,
                                              const CSSValue&);

  static std::optional<FontSelectionValue> ConvertFontStretchKeyword(
      const CSSValue&);
  static FontSelectionValue ConvertFontStretch(StyleResolverState&,
                                               const CSSValue&);
  static FontSelectionValue ConvertFontStyle(StyleResolverState&,
                                             const CSSValue&);
  static FontSelectionValue ConvertFontWeight(StyleResolverState&,
                                              const CSSValue&);

  static FontDescription::FontVariantCaps ConvertFontVariantCaps(
      StyleResolverState&,
      const CSSValue&);
  static FontDescription::VariantLigatures ConvertFontVariantLigatures(
      StyleResolverState&,
      const CSSValue&);
  static FontVariantNumeric ConvertFontVariantNumeric(StyleResolverState&,
                                                      const CSSValue&);
  static scoped_refptr<FontVariantAlternates> ConvertFontVariantAlternates(
      StyleResolverState&,
      const CSSValue&);
  static FontVariantEastAsian ConvertFontVariantEastAsian(StyleResolverState&,
                                                          const CSSValue&);
  static FontDescription::FontVariantPosition ConvertFontVariantPosition(
      StyleResolverState&,
      const CSSValue&);
  static FontVariantEmoji ConvertFontVariantEmoji(StyleResolverState&,
                                                  const CSSValue&);
  static FontDescription::Kerning ConvertFontKerning(StyleResolverState&,
                                                     const CSSValue&);
  static OpticalSizing ConvertFontOpticalSizing(StyleResolverState&,
                                                const CSSValue&);
  static StyleSelfAlignmentData ConvertSelfOrDefaultAlignmentData(
      StyleResolverState&,
      const CSSValue&);
  static StyleContentAlignmentData ConvertContentAlignmentData(
      StyleResolverState&,
      const CSSValue&);
  static GridAutoFlow ConvertGridAutoFlow(StyleResolverState&, const CSSValue&);
  static GridPosition ConvertGridPosition(StyleResolverState&, const CSSValue&);
  static ComputedGridTemplateAreas* ConvertGridTemplateAreas(
      StyleResolverState&,
      const CSSValue&);
  static GridTrackSize ConvertGridTrackSize(StyleResolverState&,
                                            const CSSValue&);
  static NGGridTrackList ConvertGridTrackSizeList(StyleResolverState&,
                                                  const CSSValue&);
  static std::optional<Length> ConvertItemTolerance(const StyleResolverState&,
                                                    const CSSValue&);
  static StyleHyphenateLimitChars ConvertHyphenateLimitChars(
      StyleResolverState&,
      const CSSValue&);
  template <typename T>
  static T ConvertLineWidth(StyleResolverState&, const CSSValue&);
  static int ConvertBorderWidth(StyleResolverState&, const CSSValue&);
  static uint16_t ConvertColumnRuleWidth(StyleResolverState&, const CSSValue&);
  static Superellipse ConvertCornerShape(const StyleResolverState&,
                                         const CSSValue&);
  static LayoutUnit ConvertLayoutUnit(const StyleResolverState&,
                                      const CSSValue&);
  static std::optional<Length> ConvertGapLength(const StyleResolverState&,
                                                const CSSValue&);
  static Length ConvertLength(const StyleResolverState&, const CSSValue&);
  static UnzoomedLength ConvertUnzoomedLength(StyleResolverState&,
                                              const CSSValue&);
  static float ConvertZoom(const StyleResolverState&, const CSSValue&);
  static TimelineInset ConvertSingleTimelineInset(StyleResolverState&,
                                                  const CSSValue&);
  static Length ConvertLengthOrAuto(const StyleResolverState&, const CSSValue&);
  static Length ConvertLengthSizing(StyleResolverState&, const CSSValue&);
  static Length ConvertLengthMaxSizing(StyleResolverState&, const CSSValue&);
  static TabSize ConvertLengthOrTabSpaces(StyleResolverState&, const CSSValue&);
  static Length ConvertLineHeight(StyleResolverState&, const CSSValue&);
  static float ConvertNumberOrPercentage(StyleResolverState&, const CSSValue&);
  static int ConvertInteger(StyleResolverState&, const CSSValue&);
  template <int NoneValue = 0>
  static int ConvertIntegerOrNone(StyleResolverState&, const CSSValue&);
  static ScrollStartData ConvertScrollStart(const StyleResolverState&,
                                            const CSSValue&);
  static float ConvertAlpha(StyleResolverState&,
                            const CSSValue&);  // clamps to [0,1]
  static ScopedCSSName* ConvertNoneOrCustomIdent(StyleResolverState&,
                                                 const CSSValue&);
  static ScopedCSSName* ConvertNormalOrCustomIdent(StyleResolverState&,
                                                   const CSSValue&);
  static ScopedCSSName* ConvertCustomIdent(StyleResolverState&,
                                           const CSSValue&);
  static ScopedCSSName* ConvertPositionAnchor(StyleResolverState&,
                                              const CSSValue&);
  static PositionVisibility ConvertPositionVisibility(StyleResolverState& state,
                                                      const CSSValue& value);
  static ScopedCSSNameList* ConvertAnchorName(StyleResolverState&,
                                              const CSSValue&);
  static StyleAnchorScope ConvertAnchorScope(StyleResolverState&,
                                             const CSSValue&);
  static StyleInitialLetter ConvertInitialLetter(StyleResolverState&,
                                                 const CSSValue&);
  static StyleOffsetRotation ConvertOffsetRotate(StyleResolverState&,
                                                 const CSSValue&);
  static LengthPoint ConvertPosition(const StyleResolverState&,
                                     const CSSValue&);
  static LengthPoint ConvertPositionOrAuto(StyleResolverState&,
                                           const CSSValue&);
  static LengthPoint ConvertOffsetPosition(StyleResolverState&,
                                           const CSSValue&);
  static float ConvertPerspective(StyleResolverState&, const CSSValue&);
  static Length ConvertQuirkyLength(StyleResolverState&, const CSSValue&);
  static scoped_refptr<QuotesData> ConvertQuotes(StyleResolverState&,
                                                 const CSSValue&);
  static LengthSize ConvertRadius(const StyleResolverState&, const CSSValue&);
  static EPaintOrder ConvertPaintOrder(StyleResolverState&, const CSSValue&);
  static GapDataList<StyleColor> ConvertGapDecorationColorDataList(
      StyleResolverState&,
      const CSSValue&,
      bool for_visited_link = false);
  static GapDataList<int> ConvertGapDecorationWidthDataList(StyleResolverState&,
                                                            const CSSValue&);
  static GapDataList<EBorderStyle> ConvertGapDecorationStyleDataList(
      StyleResolverState&,
      const CSSValue&);
  static ShadowData ConvertShadow(const CSSToLengthConversionData&,
                                  StyleResolverState*,
                                  const CSSValue&);
  static ShadowList* ConvertShadowList(StyleResolverState&, const CSSValue&);
  static ShapeValue* ConvertShapeValue(StyleResolverState&, const CSSValue&);
  static float ConvertSpacing(StyleResolverState&, const CSSValue&);
  template <CSSValueID IdForNone>
  static AtomicString ConvertString(StyleResolverState&, const CSSValue&);
  static scoped_refptr<SVGDashArray> ConvertStrokeDasharray(StyleResolverState&,
                                                            const CSSValue&);
  static StyleColor ConvertStyleColor(StyleResolverState&,
                                      const CSSValue&,
                                      bool for_visited_link = false);
  static StyleAutoColor ConvertStyleAutoColor(StyleResolverState&,
                                              const CSSValue&,
                                              bool for_visited_link = false);
  static SVGPaint ConvertSVGPaint(StyleResolverState&,
                                  const CSSValue&,
                                  bool for_visited_link,
                                  CSSPropertyID = CSSPropertyID::kInvalid);
  static TextBoxEdge ConvertTextBoxEdge(StyleResolverState&, const CSSValue&);
  static TextDecorationThickness ConvertTextDecorationThickness(
      StyleResolverState&,
      const CSSValue&);
  static TextEmphasisPosition ConvertTextTextEmphasisPosition(
      StyleResolverState&,
      const CSSValue&);
  static float ConvertTextStrokeWidth(StyleResolverState&, const CSSValue&);
  static TextSizeAdjust ConvertTextSizeAdjust(StyleResolverState&,
                                              const CSSValue&);
  static TextUnderlinePosition ConvertTextUnderlinePosition(
      StyleResolverState& state,
      const CSSValue& value);
  static Length ConvertTextUnderlineOffset(StyleResolverState& state,
                                           const CSSValue& value);
  static TransformOperations ConvertTransformOperations(StyleResolverState&,
                                                        const CSSValue&);
  static TransformOrigin ConvertTransformOrigin(StyleResolverState&,
                                                const CSSValue&);

  static void ConvertGridTrackList(const CSSValue&,
                                   ComputedGridTrackList&,
                                   StyleResolverState&);

  static cc::ScrollSnapType ConvertSnapType(StyleResolverState&,
                                            const CSSValue&);
  static cc::ScrollSnapAlign ConvertSnapAlign(StyleResolverState&,
                                              const CSSValue&);
  static TranslateTransformOperation* ConvertTranslate(StyleResolverState&,
                                                       const CSSValue&);
  static RotateTransformOperation* ConvertRotate(StyleResolverState&,
                                                 const CSSValue&);
  static ScaleTransformOperation* ConvertScale(StyleResolverState&,
                                               const CSSValue&);
  static RespectImageOrientationEnum ConvertImageOrientation(
      StyleResolverState&,
      const CSSValue&);
  static StylePath* ConvertPathOrNone(StyleResolverState&, const CSSValue&);
  static BasicShape* ConvertObjectViewBox(StyleResolverState&, const CSSValue&);
  static OffsetPathOperation* ConvertOffsetPath(StyleResolverState&,
                                                const CSSValue&);
  static StyleOffsetRotation ConvertOffsetRotate(const CSSLengthResolver&,
                                                 const CSSValue&);
  template <CSSValueID cssValueFor0, CSSValueID cssValueFor100>
  static Length ConvertPositionLength(const StyleResolverState&,
                                      const CSSValue&);
  static Rotation ConvertRotation(const CSSLengthResolver&, const CSSValue&);

  static const CSSValue& ConvertRegisteredPropertyInitialValue(Document&,
                                                               const CSSValue&);
  static const CSSValue& ConvertRegisteredPropertyValue(
      const StyleResolverState&,
      const CSSValue&,
      const CSSParserContext*);

  static CSSVariableData* ConvertRegisteredPropertyVariableData(
      const CSSValue&,
      bool is_animation_tainted,
      bool is_attr_tainted);

  static StyleAspectRatio ConvertAspectRatio(const StyleResolverState&,
                                             const CSSValue&);

  static bool ConvertInternalAlignContentBlock(StyleResolverState& state,
                                               const CSSValue& value);
  static bool ConvertInternalEmptyLineHeight(StyleResolverState& state,
                                             const CSSValue& value);

  static AtomicString ConvertPage(StyleResolverState&, const CSSValue&);

  static RubyPosition ConvertRubyPosition(StyleResolverState& state,
                                          const CSSValue& value);

  static StyleScrollbarColor* ConvertScrollbarColor(StyleResolverState& state,
                                                    const CSSValue& value);

  static ScrollbarGutter ConvertScrollbarGutter(StyleResolverState& state,
                                                const CSSValue& value);

  static ScopedCSSNameList* ConvertContainerName(StyleResolverState&,
                                                 const CSSValue&);

  static StyleIntrinsicLength ConvertIntrinsicDimension(
      const StyleResolverState&,
      const CSSValue&);

  static StyleViewTransitionName* ConvertViewTransitionName(StyleResolverState&,
                                                            const CSSValue&);
  static ScopedCSSNameList* ConvertViewTransitionClass(StyleResolverState&,
                                                       const CSSValue&);
  static StyleViewTransitionGroup ConvertViewTransitionGroup(
      StyleResolverState&,
      const CSSValue&);

  // Take a list value for a specified color-scheme, extract flags for known
  // color-schemes and the 'only' modifier, and push the list items into a
  // vector for storing the computed value on a ComputedStyle, if the passed in
  // vector is non-null.
  static ColorSchemeFlags ExtractColorSchemes(
      const Document&,
      const CSSValueList& scheme_list,
      Vector<AtomicString>* color_schemes);

  static double ConvertTimeValue(const StyleResolverState& state,
                                 const CSSValue& value);

  static std::optional<StyleOverflowClipMargin> ConvertOverflowClipMargin(
      StyleResolverState&,
      const CSSValue&);

  static Vector<TimelineAxis> ConvertViewTimelineAxis(StyleResolverState&,
                                                      const CSSValue&);
  static Vector<TimelineInset> ConvertViewTimelineInset(StyleResolverState&,
                                                        const CSSValue&);
  static ScopedCSSNameList* ConvertViewTimelineName(StyleResolverState&,
                                                    const CSSValue&);
  static ScopedCSSNameList* ConvertTimelineScope(StyleResolverState&,
                                                 const CSSValue&);

  static PositionArea ConvertPositionArea(StyleResolverState&, const CSSValue&);
};

template <typename T>
T StyleBuilderConverter::ConvertComputedLength(const StyleResolverState& state,
                                               const CSSValue& value) {
  return To<CSSPrimitiveValue>(value).ComputeLength<T>(
      state.CssToLengthConversionData());
}

template <typename T, CSSValueID ZeroValue>
T StyleBuilderConverter::ConvertFlags(StyleResolverState& state,
                                      const CSSValue& value) {
  T flags = static_cast<T>(0);
  auto* identifier_value = DynamicTo<CSSIdentifierValue>(value);
  if (identifier_value && identifier_value->GetValueID() == ZeroValue) {
    return flags;
  }
  for (auto& flag_value : To<CSSValueList>(value)) {
    flags |= To<CSSIdentifierValue>(*flag_value).ConvertTo<T>();
  }
  return flags;
}

template <typename T>
T StyleBuilderConverter::ConvertLineWidth(StyleResolverState& state,
                                          const CSSValue& value) {
  double result = 0;
  if (auto* identifier_value = DynamicTo<CSSIdentifierValue>(value)) {
    switch (identifier_value->GetValueID()) {
      case CSSValueID::kThin:
        result = 1;
        break;
      case CSSValueID::kMedium:
        result = 3;
        break;
      case CSSValueID::kThick:
        result = 5;
        break;
      default:
        NOTREACHED();
    }
    result = state.CssToLengthConversionData().ZoomedComputedPixels(
        result, CSSPrimitiveValue::UnitType::kPixels);
  } else {
    result = To<CSSPrimitiveValue>(value).ComputeLength<double>(
        state.CssToLengthConversionData());
  }
  // TODO(crbug.com/485650, crbug.com/382483): We are moving to use the full
  // page zoom implementation to handle high-dpi.  In that case specyfing a
  // border-width of less than 1px would result in a border that is one device
  // pixel thick.  With this change that would instead be rounded up to 2
  // device pixels.  Consider clamping it to device pixels or zoom adjusted CSS
  // pixels instead of raw CSS pixels.
  double zoomed_result = state.StyleBuilder().EffectiveZoom() * result;
  if (zoomed_result > 0.0 && zoomed_result < 1.0) {
    return 1.0;
  }
  return ClampTo<T>(RoundForImpreciseConversion<T>(result),
                    DefaultMinimumForClamp<T>(), DefaultMaximumForClamp<T>());
}

template <CSSValueID cssValueFor0, CSSValueID cssValueFor100>
Length StyleBuilderConverter::ConvertPositionLength(
    const StyleResolverState& state,
    const CSSValue& value) {
  if (const auto* pair = DynamicTo<CSSValuePair>(value)) {
    Length length = StyleBuilderConverter::ConvertLength(state, pair->Second());
    if (To<CSSIdentifierValue>(pair->First()).GetValueID() == cssValueFor0) {
      return length;
    }
    DCHECK_EQ(To<CSSIdentifierValue>(pair->First()).GetValueID(),
              cssValueFor100);
    return length.SubtractFromOneHundredPercent();
  }

  if (auto* identifier_value = DynamicTo<CSSIdentifierValue>(value)) {
    switch (identifier_value->GetValueID()) {
      case cssValueFor0:
        return Length::Percent(0);
      case cssValueFor100:
        return Length::Percent(100);
      case CSSValueID::kCenter:
        return Length::Percent(50);
      default:
        NOTREACHED();
    }
  }

  return StyleBuilderConverter::ConvertLength(state,
                                              To<CSSPrimitiveValue>(value));
}

template <CSSValueID IdForNone>
AtomicString StyleBuilderConverter::ConvertString(StyleResolverState&,
                                                  const CSSValue& value) {
  if (auto* string_value = DynamicTo<CSSStringValue>(value)) {
    return AtomicString(string_value->Value());
  }
  DCHECK_EQ(To<CSSIdentifierValue>(value).GetValueID(), IdForNone);
  return g_null_atom;
}

template <int NoneValue>
int StyleBuilderConverter::ConvertIntegerOrNone(StyleResolverState& state,
                                                const CSSValue& value) {
  if (IsA<CSSPrimitiveValue>(value)) {
    return ConvertInteger(state, value);
  }
  DCHECK_EQ(To<CSSIdentifierValue>(value).GetValueID(), CSSValueID::kNone);
  return NoneValue;
}

// Parameter bag for `ResolveColorValue()`. Typical usage will be to construct
// an instance from a Document just prior to calling that function. Not intended
// for other uses.
struct ResolveColorValueContext {
  STACK_ALLOCATED();

 public:
  const CSSLengthResolver& length_resolver;
  const TextLinkColors& text_link_colors;
  const mojom::blink::ColorScheme used_color_scheme =
      mojom::blink::ColorScheme::kLight;
  const ui::ColorProvider* color_provider = nullptr;
  const bool is_in_web_app_scope = false;
  const bool for_visited_link = false;
};

// Returns the computed <color> value for `value`. Note that it's expected that
// `value` is the result of parsing a <color> value.
// See: https://drafts.csswg.org/css-color/#resolving-color-values
CORE_EXPORT StyleColor
ResolveColorValue(const CSSValue& value,
                  const ResolveColorValueContext& context);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_BUILDER_CONVERTER_H_
