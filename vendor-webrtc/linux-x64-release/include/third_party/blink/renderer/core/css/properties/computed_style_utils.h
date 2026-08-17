// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_COMPUTED_STYLE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_COMPUTED_STYLE_UTILS_H_

#include <optional>

#include "cc/input/scroll_snap_data.h"
#include "third_party/blink/renderer/core/animation/timeline_offset.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_border_image_slice_value.h"
#include "third_party/blink/renderer/core/css/css_function_value.h"
#include "third_party/blink/renderer/core/css/css_identifier_value.h"
#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css/css_value_pair.h"
#include "third_party/blink/renderer/core/css/zoom_adjusted_pixel_value.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/superellipse.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ComputedStyle;
class CSSNumericLiteralValue;
class CSSStyleValue;
class CSSValue;
class FontFamily;
class PositionArea;
class StyleColor;
class StyleIntrinsicLength;
class StylePropertyShorthand;
class StyleTimeline;

namespace cssvalue {
class CSSContentDistributionValue;
}

class CORE_EXPORT ComputedStyleUtils {
  STATIC_ONLY(ComputedStyleUtils);

 public:
  inline static CSSValue* ZoomAdjustedPixelValueOrAuto(
      const Length& length,
      const ComputedStyle& style) {
    if (length.IsAuto()) {
      return CSSIdentifierValue::Create(CSSValueID::kAuto);
    }
    DCHECK(length.IsFixed());
    return ZoomAdjustedPixelValue(length.Pixels(), style);
  }

  static const CSSValue* ValueForColor(const StyleColor&,
                                       const ComputedStyle&,
                                       const Color* override_current_color,
                                       CSSValuePhase);
  static const CSSValue* CurrentColorOrValidColor(const ComputedStyle&,
                                                  const StyleColor&,
                                                  CSSValuePhase);
  static const blink::Color BorderSideColor(const ComputedStyle&,
                                            const StyleColor&,
                                            EBorderStyle,
                                            bool visited_link,
                                            bool* is_current_color);
  static CSSValue* ZoomAdjustedPixelValueForLength(const Length&,
                                                   const ComputedStyle&);
  static const CSSValue* BackgroundImageOrMaskImage(const ComputedStyle&,
                                                    bool allow_visited_style,
                                                    const FillLayer&,
                                                    CSSValuePhase value_phase);
  static const CSSValue* ValueForFillSize(const FillSize&,
                                          const ComputedStyle&);
  static const CSSValue* BackgroundImageOrMaskSize(const ComputedStyle&,
                                                   const FillLayer&);
  static const CSSValueList* CreatePositionListForLayer(const CSSProperty&,
                                                        const FillLayer&,
                                                        const ComputedStyle&);
  static const CSSValue* ValueForFillRepeat(const FillLayer* curr_layer);
  static const CSSValue* MaskMode(const FillLayer* curr_layer);
  static const CSSValue* RepeatStyle(const FillLayer* curr_layer);
  static const CSSValueList* ValuesForBackgroundShorthand(
      const ComputedStyle&,
      const LayoutObject*,
      bool allow_visited_style,
      CSSValuePhase value_phase);
  static const CSSValueList* ValuesForMaskShorthand(
      const StylePropertyShorthand&,
      const ComputedStyle&,
      const LayoutObject*,
      bool allow_visited_style,
      CSSValuePhase value_phase);
  static const CSSValue* BackgroundPositionOrMaskPosition(const CSSProperty&,
                                                          const ComputedStyle&,
                                                          const FillLayer*);
  static const CSSValue* BackgroundPositionXOrWebkitMaskPositionX(
      const ComputedStyle&,
      const FillLayer*);
  static const CSSValue* BackgroundPositionYOrWebkitMaskPositionY(
      const ComputedStyle&,
      const FillLayer*);
  static cssvalue::CSSBorderImageSliceValue* ValueForNinePieceImageSlice(
      const NinePieceImage&);
  static CSSQuadValue* ValueForNinePieceImageQuad(const BorderImageLengthBox&,
                                                  const ComputedStyle&);
  static CSSValue* ValueForNinePieceImageRepeat(const NinePieceImage&);
  static CSSValue* ValueForNinePieceImage(const NinePieceImage&,
                                          const ComputedStyle&,
                                          bool allow_visited_style,
                                          CSSValuePhase value_phase);
  static CSSValue* ValueForReflection(const StyleReflection*,
                                      const ComputedStyle&,
                                      bool allow_visited_style,
                                      CSSValuePhase value_phase);
  static CSSValue* ValueForPosition(const LengthPoint& position,
                                    const ComputedStyle&);

  static CSSValue* ValueForOffset(const ComputedStyle&,
                                  const LayoutObject*,
                                  bool allow_visited_style,
                                  CSSValuePhase value_phase);
  static CSSValue* MinWidthOrMinHeightAuto(const ComputedStyle&);
  static CSSValue* ValueForPositionOffset(const ComputedStyle&,
                                          const CSSProperty&,
                                          const LayoutObject*,
                                          CSSValuePhase);
  static CSSValue* ValueForItemPositionWithOverflowAlignment(
      const StyleSelfAlignmentData&);
  static cssvalue::CSSContentDistributionValue*
  ValueForContentPositionAndDistributionWithOverflowAlignment(
      const StyleContentAlignmentData&);
  static CSSValue* ValueForLineHeight(const ComputedStyle&);
  static CSSValue* ComputedValueForLineHeight(const ComputedStyle&);
  static CSSValueList* ValueForFontFamily(const FontFamily&);
  static CSSValueList* ValueForFontFamily(const ComputedStyle&);
  static CSSPrimitiveValue* ValueForFontSize(const ComputedStyle&);
  static CSSValue* ValueForFontSizeAdjust(const ComputedStyle&);
  static CSSPrimitiveValue* ValueForFontStretch(const ComputedStyle&);
  static CSSValue* ValueForFontStyle(const ComputedStyle&);
  static CSSNumericLiteralValue* ValueForFontWeight(const ComputedStyle&);
  static CSSIdentifierValue* ValueForFontVariantCaps(const ComputedStyle&);
  static CSSValue* ValueForFontVariantLigatures(const ComputedStyle&);
  static CSSValue* ValueForFontVariantNumeric(const ComputedStyle&);
  static CSSValue* ValueForFont(const ComputedStyle&);
  static CSSValue* ValueForFontVariantEastAsian(const ComputedStyle&);
  static CSSValue* ValueForFontVariantAlternates(const ComputedStyle&);
  static CSSIdentifierValue* ValueForFontVariantPosition(const ComputedStyle&);
  static CSSIdentifierValue* ValueForFontKerning(const ComputedStyle&);
  static CSSIdentifierValue* ValueForFontOpticalSizing(const ComputedStyle&);
  static CSSValue* ValueForFontFeatureSettings(const ComputedStyle&);
  static CSSValue* ValueForFontVariationSettings(const ComputedStyle&);
  static const CSSValue* ValueForFontPalette(const ComputedStyle&);
  static CSSValue* SpecifiedValueForGridTrackSize(const GridTrackSize&,
                                                  const ComputedStyle&);
  static CSSValue* ValueForGridAutoTrackList(const NGGridTrackList&,
                                             const LayoutObject*,
                                             const ComputedStyle&);
  static CSSValue* ValueForGridTrackList(GridTrackSizingDirection,
                                         const LayoutObject*,
                                         const ComputedStyle&,
                                         bool force_computed_value = false);
  static CSSValue* ValueForGridPosition(const GridPosition&);
  static CSSValue* ValueForItemTolerance(const std::optional<Length>&,
                                         const ComputedStyle&);
  static CSSValue* ValueForMasonryTrackList(const LayoutObject*,
                                            const ComputedStyle&);
  static gfx::SizeF UsedBoxSize(const LayoutObject&);
  static CSSValue* RenderTextDecorationFlagsToCSSValue(TextDecorationLine);
  static CSSValue* ValueForTextDecorationStyle(ETextDecorationStyle);
  static CSSValue* ValueForTextDecorationSkipInk(ETextDecorationSkipInk);
  static CSSValue* TouchActionFlagsToCSSValue(TouchAction);
  static CSSValue* ValueForWillChange(const Vector<CSSPropertyID>&,
                                      bool will_change_contents,
                                      bool will_change_scroll_position);

  static CSSValue* ValueForAnimationDelay(const Timing::Delay& delay);
  static CSSValue* ValueForAnimationDirection(Timing::PlaybackDirection);
  static CSSValue* ValueForAnimationDuration(const std::optional<double>&,
                                             bool resolve_auto_to_zero);
  static CSSValue* ValueForAnimationFillMode(Timing::FillMode);
  static CSSValue* ValueForAnimationIterationCount(double iteration_count);
  static CSSValue* ValueForAnimationPlayState(EAnimPlayState);
  static CSSValue* ValueForAnimationRangeList(
      const Vector<std::optional<TimelineOffset>>& range_list,
      const CSSAnimationData* animation_data,
      const ComputedStyle& style,
      const Length& default_offset);
  static CSSValue* ValueForAnimationTimingFunction(
      const scoped_refptr<TimingFunction>&);
  static CSSValue* ValueForAnimationTimeline(const StyleTimeline&);

  static CSSValue* ValueForAnimationDelayList(const CSSTimingData*);
  static CSSValue* ValueForAnimationDirectionList(const CSSAnimationData*);
  static CSSValue* ValueForAnimationDurationList(const CSSAnimationData*,
                                                 CSSValuePhase phase);
  static CSSValue* ValueForAnimationDurationList(const CSSTransitionData*);
  static CSSValue* ValueForAnimationFillModeList(const CSSAnimationData*);
  static CSSValue* ValueForAnimationIterationCountList(const CSSAnimationData*);
  static CSSValue* ValueForAnimationPlayStateList(const CSSAnimationData*);
  static CSSValue* ValueForAnimationRange(
      const std::optional<TimelineOffset>& offset,
      const ComputedStyle& style,
      const Length& default_offset);
  static CSSValue* ValueForAnimationRangeStartList(const CSSAnimationData*,
                                                   const ComputedStyle&);
  static CSSValue* ValueForAnimationRangeEndList(const CSSAnimationData*,
                                                 const ComputedStyle&);
  static CSSValue* ValueForAnimationTimingFunctionList(const CSSTimingData*);
  static CSSValue* ValueForAnimationTimelineList(const CSSAnimationData*);

  static CSSValue* ValueForTimelineInset(const TimelineInset&,
                                         const ComputedStyle&);
  static CSSValue* SingleValueForTimelineShorthand(const ScopedCSSName* name,
                                                   TimelineAxis,
                                                   std::optional<TimelineInset>,
                                                   const ComputedStyle&);
  static CSSValue* ValueForAnimationTriggerRangeStartList(
      const CSSAnimationData* animation_data,
      const ComputedStyle& style);
  static CSSValue* ValueForAnimationTriggerRangeEndList(
      const CSSAnimationData* animation_data,
      const ComputedStyle& style);
  static CSSValue* ValueForAnimationRangeOrAuto(
      const TimelineOffsetOrAuto& offset,
      const ComputedStyle& style,
      const Length& default_offset);
  static CSSValue* ValueForAnimationTriggerExitRangeList(
      const Vector<TimelineOffsetOrAuto>& range_list,
      const CSSAnimationData* animation_data,
      const ComputedStyle& style,
      const Length& default_offset);
  static CSSValue* ValueForAnimationTriggerExitRangeStartList(
      const CSSAnimationData* animation_data,
      const ComputedStyle& style);
  static CSSValue* ValueForAnimationTriggerExitRangeEndList(
      const CSSAnimationData* animation_data,
      const ComputedStyle& style);
  static CSSValue* ValueForAnimationTriggerType(const EAnimationTriggerType);
  static CSSValue* ValueForAnimationTriggerTypeList(const CSSAnimationData*);
  static CSSValue* ValueForAnimationTriggerTimelineList(
      const CSSAnimationData*);
  static CSSValueList* ValuesForBorderRadiusCorner(const LengthSize&,
                                                   const ComputedStyle&);
  static CSSValue* ValueForBorderRadiusCorner(const LengthSize&,
                                              const ComputedStyle&);
  static CSSValue* ValueForCornerShape(const Superellipse&);

  // Serializes a gfx::Transform into a matrix() or matrix3d() transform
  // function value. If force_matrix3d is true, it will always give a matrix3d
  // value (for serializing a matrix3d in a transform list), otherwise it
  // will give a matrix() where possible (for serializing matrix in transform
  // lists or resolved transformation matrices).
  static CSSFunctionValue* ValueForTransform(const gfx::Transform&,
                                             float zoom,
                                             bool force_matrix3d);
  // Values unreperesentable in CSS will be converted to an equivalent matrix()
  // value. The box_size parameter is used for deferred, layout-dependent
  // interpolations and is not needed in the absence of animations.
  static CSSFunctionValue* ValueForTransformOperation(
      const TransformOperation&,
      float zoom,
      gfx::SizeF box_size = gfx::SizeF(0, 0));
  // Serialize a transform list.
  static CSSValue* ValueForTransformList(const TransformOperations&,
                                         float zoom,
                                         gfx::SizeF box_size = gfx::SizeF(0,
                                                                          0));
  static CSSValue* ValueForTransformFunction(const TransformOperations&);
  static gfx::RectF ReferenceBoxForTransform(const LayoutObject&);
  // The LayoutObject parameter is only used for converting unreperesentable
  // relative transforms into matrix() values, with a default box size of 0x0.
  static CSSValue* ComputedTransformList(const ComputedStyle&,
                                         const LayoutObject* = nullptr);
  static CSSValue* ResolvedTransform(const LayoutObject*, const ComputedStyle&);
  static CSSValue* CreateTransitionPropertyValue(
      const CSSTransitionData::TransitionProperty&);
  static CSSValue* CreateTransitionBehaviorValue(
      const CSSTransitionData::TransitionBehavior&);
  static CSSValue* ValueForTransitionProperty(const CSSTransitionData*);
  static CSSValue* ValueForTransitionBehavior(const CSSTransitionData*);
  static CSSValue* ValueForContentData(const ComputedStyle&,
                                       bool allow_visited_style,
                                       CSSValuePhase value_phase);

  static CSSValue* ValueForCounterDirectives(
      const ComputedStyle&,
      CountersAttachmentContext::Type type);
  static CSSValue* ValueForShape(const ComputedStyle&,
                                 bool allow_visited_style,
                                 ShapeValue*,
                                 CSSValuePhase value_phase);
  static CSSValueList* ValueForBorderRadiusShorthand(const ComputedStyle&);
  static CSSValueList* ValueForCornerShapeShorthand(const ComputedStyle&);
  static CSSValue* StrokeDashArrayToCSSValueList(const SVGDashArray&,
                                                 const ComputedStyle&);
  static const CSSValue* ValueForSVGPaint(const SVGPaint&,
                                          const ComputedStyle&);
  static CSSValue* ValueForSVGResource(const StyleSVGResource*);
  static const CSSValue* ValueForGapDecorationColorDataList(
      const GapDataList<StyleColor>&,
      const ComputedStyle&,
      CSSValuePhase);
  static const CSSValue* ValueForGapDecorationWidthDataList(
      const GapDataList<int>&,
      const ComputedStyle&,
      CSSValuePhase);
  static const CSSValue* ValueForGapDecorationStyleDataList(
      const GapDataList<EBorderStyle>&,
      const ComputedStyle&,
      CSSValuePhase);
  static CSSValue* ValueForShadowData(const ShadowData&,
                                      const ComputedStyle&,
                                      bool use_spread,
                                      CSSValuePhase);
  static CSSValue* ValueForShadowList(const ShadowList*,
                                      const ComputedStyle&,
                                      bool use_spread,
                                      CSSValuePhase);
  static CSSValue* ValueForFilter(const ComputedStyle&,
                                  const FilterOperations&);
  static CSSValue* ValueForScrollSnapType(const cc::ScrollSnapType&,
                                          const ComputedStyle&);
  static CSSValue* ValueForScrollSnapAlign(const cc::ScrollSnapAlign&,
                                           const ComputedStyle&);
  static CSSValue* ValueForPageBreakBetween(EBreakBetween);
  static CSSValue* ValueForWebkitColumnBreakBetween(EBreakBetween);
  static CSSValue* ValueForPageBreakInside(EBreakInside);
  static CSSValue* ValueForWebkitColumnBreakInside(EBreakInside);
  static bool WidthOrHeightShouldReturnUsedValue(const LayoutObject*);
  static CSSValueList* ValuesForShorthandProperty(const StylePropertyShorthand&,
                                                  const ComputedStyle&,
                                                  const LayoutObject*,
                                                  bool allow_visited_style,
                                                  CSSValuePhase value_phase);
  static CSSValuePair* ValuesForGapShorthand(const StylePropertyShorthand&,
                                             const ComputedStyle&,
                                             const LayoutObject*,
                                             bool allow_visited_style,
                                             CSSValuePhase value_phase);
  static CSSValueList* ValuesForGridShorthand(const StylePropertyShorthand&,
                                              const ComputedStyle&,
                                              const LayoutObject*,
                                              bool allow_visited_style,
                                              CSSValuePhase value_phase);
  static CSSValueList* ValuesForGridAreaShorthand(const StylePropertyShorthand&,
                                                  const ComputedStyle&,
                                                  const LayoutObject*,
                                                  bool allow_visited_style,
                                                  CSSValuePhase value_phase);
  static CSSValueList* ValuesForGridLineShorthand(const StylePropertyShorthand&,
                                                  const ComputedStyle&,
                                                  const LayoutObject*,
                                                  bool allow_visited_style,
                                                  CSSValuePhase value_phase);
  static CSSValueList* ValuesForGridTemplateShorthand(
      const StylePropertyShorthand&,
      const ComputedStyle&,
      const LayoutObject*,
      bool allow_visited_style,
      CSSValuePhase value_phase);

  static const CSSValue* ValuesForBidirectionalGapRuleShorthand(
      const StylePropertyShorthand&,
      const ComputedStyle&,
      const LayoutObject*,
      bool allow_visited_style,
      CSSValuePhase value_phase);

  static CSSValueList* ValuesForSidesShorthand(const StylePropertyShorthand&,
                                               const ComputedStyle&,
                                               const LayoutObject*,
                                               bool allow_visited_style,
                                               CSSValuePhase value_phase);
  static CSSValuePair* ValuesForInlineBlockShorthand(
      const StylePropertyShorthand&,
      const ComputedStyle&,
      const LayoutObject*,
      bool allow_visited_style,
      CSSValuePhase value_phase);
  static CSSValuePair* ValuesForPlaceShorthand(const StylePropertyShorthand&,
                                               const ComputedStyle&,
                                               const LayoutObject*,
                                               bool allow_visited_style,
                                               CSSValuePhase value_phase);
  static CSSValue* ValuesForInterestTargetDelayShorthand(
      const ComputedStyle&,
      const LayoutObject*,
      bool allow_visited_style,
      CSSValuePhase value_phase);
  static CSSValue* ValuesForFontVariantProperty(const ComputedStyle&,
                                                const LayoutObject*,
                                                bool allow_visited_style,
                                                CSSValuePhase value_phase);
  static CSSValue* ValuesForFontSynthesisProperty(const ComputedStyle&,
                                                  const LayoutObject*,
                                                  bool allow_visited_style,
                                                  CSSValuePhase value_phase);
  static CSSValueList* ValuesForContainerShorthand(const ComputedStyle&,
                                                   const LayoutObject*,
                                                   bool allow_visited_style,
                                                   CSSValuePhase value_phase);
  static CSSValue* ValueForGapLength(const std::optional<Length>&,
                                     const ComputedStyle&);
  static CSSValue* ValueForStyleName(const StyleName&);
  static CSSValue* ValueForStyleNameOrKeyword(const StyleNameOrKeyword&);
  static CSSValue* ValueForCustomIdentOrNone(const AtomicString&);
  static CSSValue* ValueForCustomIdentOrNone(const ScopedCSSName*);
  static const CSSValue* ValueForStyleAutoColor(const ComputedStyle&,
                                                const StyleAutoColor&,
                                                CSSValuePhase);
  static CSSValue* ValueForIntrinsicLength(const ComputedStyle&,
                                           const StyleIntrinsicLength&);
  static CSSValue* ValueForScrollStart(const ComputedStyle&,
                                       const ScrollStartData&);
  static CSSValue* ValueForPositionArea(const blink::PositionArea&);
  static CSSValue* ValueForPositionTryFallbacks(const PositionTryFallbacks&);
  static std::unique_ptr<CrossThreadStyleValue>
  CrossThreadStyleValueFromCSSStyleValue(CSSStyleValue* style_value);

  // Returns the computed CSSValue of the given property from the style,
  // which may different than the resolved value returned by
  // CSSValueFromComputedStyle().
  // see https://drafts.csswg.org/cssom/#resolved-values
  //
  // In most, but not all, cases, the resolved value involves layout-dependent
  // calculations, and the computed value is used as a fallback when there is
  // no layout object (display: none, etc). In those cases, this calls
  // CSSValueFromComputedStyle(layout_object=nullptr), with the exceptions
  // (transform and line-height currently) having their own logic here.
  //
  // The LayoutObject parameter is only used for converting unreperesentable
  // relative transforms into matrix() values, with a default box size of 0x0.
  static const CSSValue* ComputedPropertyValue(const CSSProperty&,
                                               const ComputedStyle&,
                                               const LayoutObject* = nullptr);

 private:
  // Returns the CSSValueID for a scale transform operation.
  static CSSValueID CSSValueIDForScaleOperation(
      const TransformOperation::OperationType);
  // Returns the CSSValueID for a translate transform operation.

  static CSSValueID CSSValueIDForTranslateOperation(
      const TransformOperation::OperationType);
  // Returns the CSSValueID for a rotate transform operation.
  static CSSValueID CSSValueIDForRotateOperation(
      const TransformOperation::OperationType);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_COMPUTED_STYLE_UTILS_H_
