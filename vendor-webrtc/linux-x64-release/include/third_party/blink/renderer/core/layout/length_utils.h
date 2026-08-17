// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LENGTH_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LENGTH_UTILS_H_

#include <optional>

#include "base/functional/function_ref.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/constraint_space.h"
#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/core/layout/geometry/fragment_geometry.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/layout/min_max_sizes.h"
#include "third_party/blink/renderer/core/layout/table/table_node.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"

namespace blink {

class ComputedStyle;
class ConstraintSpace;
class Length;

// min/max-content take the CSS aspect-ratio property into account.
// In some cases that's undesirable; this enum lets you choose not
// to do that using |kIntrinsic|.
enum class SizeType { kContent, kIntrinsic };

// Multiple functions in this file use MinMaxSizesFunctionRef callbacks, which
// should have the following form:
//
// auto MinMaxSizesFunc = [](SizeType) -> MinMaxSizesResult { };
//
// This is used for computing the min/max content or intrinsic sizes on-demand
// rather than determining if a length resolving function will require these
// sizes ahead of time.
using MinMaxSizesFunctionRef = base::FunctionRef<MinMaxSizesResult(SizeType)>;

using BlockSizeFunctionRef = base::FunctionRef<LayoutUnit(SizeType)>;

inline bool NeedMinMaxSize(const ComputedStyle& style) {
  return style.LogicalWidth().HasContentOrIntrinsic() ||
         style.LogicalMinWidth().HasContentOrIntrinsic() ||
         style.LogicalMaxWidth().HasContentOrIntrinsic();
}

CORE_EXPORT LayoutUnit
InlineSizeFromAspectRatio(const BoxStrut& border_padding,
                          const LogicalSize& aspect_ratio,
                          EBoxSizing box_sizing,
                          LayoutUnit block_size);
LayoutUnit BlockSizeFromAspectRatio(const BoxStrut& border_padding,
                                    const LogicalSize& aspect_ratio,
                                    EBoxSizing box_sizing,
                                    LayoutUnit inline_size);

// Used to distinguish between the different length classes.
enum class LengthTypeInternal { kMin, kMain, kMax };

// How fit-content should resolve if the available-size is indefinite.
enum class FitContentMode {
  kNormal,  // fit-content will resolve as min-content for the min-size, and
            // max-content for the max-size.
  kMinContribution,  // fit-content will resolve as min-content.
  kMaxContribution   // fit-content will resolve as max-content.
};

// Resolve means translate a Length to a LayoutUnit.
//  - |ConstraintSpace| the information given by the parent, e.g. the
//    available-size.
//  - |ComputedStyle| the style of the node.
//  - |border_padding| the resolved border, and padding of the node.
//  - |MinMaxSizes| is only used when the length is intrinsic (fit-content).
//  - |Length| is the length to resolve.
//  - |override_available_size| overrides the available-size. This is used when
//    computing the size of an OOF-positioned element, accounting for insets
//    and the static position.
CORE_EXPORT LayoutUnit
ResolveInlineLengthInternal(const ConstraintSpace&,
                            const ComputedStyle&,
                            const BoxStrut& border_padding,
                            MinMaxSizesFunctionRef,
                            const Length&,
                            const Length* auto_length,
                            LengthTypeInternal length_type,
                            FitContentMode fit_content_mode,
                            LayoutUnit override_available_size,
                            CalcSizeKeywordBehavior calc_size_keyword_behavior);

// Same as ResolveInlineLengthInternal, except here |intrinsic_size| roughly
// plays the part of |MinMaxSizes|.
CORE_EXPORT LayoutUnit ResolveBlockLengthInternal(
    const ConstraintSpace&,
    const ComputedStyle&,
    const BoxStrut& border_padding,
    const Length&,
    const Length* auto_length,
    LengthTypeInternal length_type,
    LayoutUnit override_available_size,
    const LayoutUnit* override_percentage_resolution_size,
    BlockSizeFunctionRef block_size_func);

// Used for resolving min inline lengths, (|ComputedStyle::MinLogicalWidth|).
inline LayoutUnit ResolveMinInlineLength(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style,
    const BoxStrut& border_padding,
    MinMaxSizesFunctionRef min_max_sizes_func,
    const Length& length,
    const Length* auto_length = nullptr,
    LayoutUnit override_available_size = kIndefiniteSize,
    FitContentMode fit_content_mode = FitContentMode::kNormal) {
  const LayoutUnit result = ResolveInlineLengthInternal(
      constraint_space, style, border_padding, min_max_sizes_func, length,
      auto_length, LengthTypeInternal::kMin, fit_content_mode,
      override_available_size, CalcSizeKeywordBehavior::kAsSpecified);
  return result == kIndefiniteSize ? border_padding.InlineSum() : result;
}

// Used for resolving max inline lengths, (|ComputedStyle::MaxLogicalWidth|).
inline LayoutUnit ResolveMaxInlineLength(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style,
    const BoxStrut& border_padding,
    MinMaxSizesFunctionRef min_max_sizes_func,
    const Length& length,
    LayoutUnit override_available_size = kIndefiniteSize,
    FitContentMode fit_content_mode = FitContentMode::kNormal) {
  const LayoutUnit result = ResolveInlineLengthInternal(
      constraint_space, style, border_padding, min_max_sizes_func, length,
      /* auto_length */ nullptr, LengthTypeInternal::kMax, fit_content_mode,
      override_available_size, CalcSizeKeywordBehavior::kAsSpecified);
  return result == kIndefiniteSize ? LayoutUnit::Max() : result;
}

// Used for resolving main inline lengths, (|ComputedStyle::LogicalWidth|).
inline LayoutUnit ResolveMainInlineLength(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style,
    const BoxStrut& border_padding,
    MinMaxSizesFunctionRef min_max_sizes_func,
    const Length& length,
    const Length* auto_length,
    LayoutUnit override_available_size = kIndefiniteSize,
    CalcSizeKeywordBehavior calc_size_keyword_behavior =
        CalcSizeKeywordBehavior::kAsSpecified) {
  return ResolveInlineLengthInternal(
      constraint_space, style, border_padding, min_max_sizes_func, length,
      auto_length, LengthTypeInternal::kMain, FitContentMode::kNormal,
      override_available_size, calc_size_keyword_behavior);
}

// Used for resolving min block lengths, (|ComputedStyle::MinLogicalHeight|).
inline LayoutUnit ResolveInitialMinBlockLength(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style,
    const BoxStrut& border_padding,
    const Length& length,
    LayoutUnit override_available_size = kIndefiniteSize) {
  const LayoutUnit result = ResolveBlockLengthInternal(
      constraint_space, style, border_padding, length,
      /* auto_length */ &Length::Auto(), LengthTypeInternal::kMin,
      override_available_size,
      /* override_percentage_resolution_size */ nullptr,
      [](SizeType) { return kIndefiniteSize; });
  return result == kIndefiniteSize ? border_padding.BlockSum() : result;
}
inline LayoutUnit ResolveMinBlockLength(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style,
    const BoxStrut& border_padding,
    BlockSizeFunctionRef block_size_func,
    const Length& length,
    const Length* auto_length = nullptr,
    LayoutUnit override_available_size = kIndefiniteSize,
    const LayoutUnit* override_percentage_resolution_size = nullptr) {
  const LayoutUnit result = ResolveBlockLengthInternal(
      constraint_space, style, border_padding, length, auto_length,
      LengthTypeInternal::kMin, override_available_size,
      override_percentage_resolution_size, block_size_func);
  return result == kIndefiniteSize ? border_padding.BlockSum() : result;
}

// Used for resolving max block lengths, (|ComputedStyle::MaxLogicalHeight|).
inline LayoutUnit ResolveInitialMaxBlockLength(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style,
    const BoxStrut& border_padding,
    const Length& length,
    LayoutUnit override_available_size = kIndefiniteSize) {
  const LayoutUnit result = ResolveBlockLengthInternal(
      constraint_space, style, border_padding, length,
      /* auto_length */ &Length::Auto(), LengthTypeInternal::kMax,
      override_available_size,
      /* override_percentage_resolution_size */ nullptr,
      [](SizeType) { return kIndefiniteSize; });
  return result == kIndefiniteSize ? LayoutUnit::Max() : result;
}
inline LayoutUnit ResolveMaxBlockLength(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style,
    const BoxStrut& border_padding,
    const Length& length,
    BlockSizeFunctionRef block_size_func,
    LayoutUnit override_available_size = kIndefiniteSize,
    const LayoutUnit* override_percentage_resolution_size = nullptr) {
  const LayoutUnit result = ResolveBlockLengthInternal(
      constraint_space, style, border_padding, length,
      /* auto_length */ &Length::Auto(), LengthTypeInternal::kMax,
      override_available_size, override_percentage_resolution_size,
      block_size_func);
  return result == kIndefiniteSize ? LayoutUnit::Max() : result;
}

// Used for resolving main block lengths, (|ComputedStyle::LogicalHeight|).
inline LayoutUnit ResolveMainBlockLength(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style,
    const BoxStrut& border_padding,
    const Length& length,
    const Length* auto_length,
    LayoutUnit intrinsic_size,
    LayoutUnit override_available_size = kIndefiniteSize) {
  return ResolveBlockLengthInternal(
      constraint_space, style, border_padding, length, auto_length,
      LengthTypeInternal::kMain, override_available_size,
      /* override_percentage_resolution_size */ nullptr,
      [intrinsic_size](SizeType) { return intrinsic_size; });
}

inline LayoutUnit ResolveMainBlockLength(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style,
    const BoxStrut& border_padding,
    const Length& length,
    const Length* auto_length,
    BlockSizeFunctionRef block_size_func,
    LayoutUnit override_available_size = kIndefiniteSize) {
  return ResolveBlockLengthInternal(
      constraint_space, style, border_padding, length, auto_length,
      LengthTypeInternal::kMain, override_available_size,
      /* override_percentage_resolution_size */ nullptr, block_size_func);
}

// Computes the min-block-size and max-block-size values for a node.
//
// The initial variant of this function won't try and resolve
// "min-block-size: min-content" and similar.
MinMaxSizes ComputeInitialMinMaxBlockSizes(
    const ConstraintSpace&,
    const BlockNode&,
    const BoxStrut& border_padding,
    LayoutUnit override_available_size = kIndefiniteSize);
MinMaxSizes ComputeMinMaxBlockSizes(
    const ConstraintSpace&,
    const BlockNode&,
    const BoxStrut& border_padding,
    const Length* auto_min_length,
    BlockSizeFunctionRef,
    LayoutUnit override_available_size = kIndefiniteSize);

MinMaxSizes ComputeTransferredMinMaxInlineSizes(
    const LogicalSize& ratio,
    const MinMaxSizes& block_min_max,
    const BoxStrut& border_padding,
    const EBoxSizing sizing);
MinMaxSizes ComputeTransferredMinMaxBlockSizes(const LogicalSize& ratio,
                                               const MinMaxSizes& block_min_max,
                                               const BoxStrut& border_padding,
                                               const EBoxSizing sizing);

// Computes the transferred min/max inline sizes from the min/max block
// sizes and the aspect ratio.
// This will compute the min/max block sizes for you, but it only works with
// styles that have a LogicalAspectRatio. It doesn't work if the aspect ratio is
// coming from a replaced element.
CORE_EXPORT MinMaxSizes
ComputeMinMaxInlineSizesFromAspectRatio(const ConstraintSpace&,
                                        const BlockNode&,
                                        const BoxStrut& border_padding);

enum class TransferredSizesMode {
  kNormal,  // Apply the transferred min/max sizes.
  kIgnore   // Ignore the transferred min/max sizes.
};

MinMaxSizes ComputeMinMaxInlineSizes(
    const ConstraintSpace& space,
    const BlockNode& node,
    const BoxStrut& border_padding,
    const Length* auto_min_length,
    MinMaxSizesFunctionRef min_max_sizes_func,
    TransferredSizesMode transferred_sizes_mode = TransferredSizesMode::kNormal,
    FitContentMode fit_content_mode = FitContentMode::kNormal,
    LayoutUnit override_available_size = kIndefiniteSize);

// Returns block size of the node's border box by resolving the computed value
// in `style.logicalHeight` to a `LayoutUnit`, adding border and padding, then
// constraining the result by the resolved min and max logical height from the
// `ComputedStyle` object.
//
// `inline_size` is necessary when an aspect ratio is in use.
// `override_available_size` is needed for <table> layout: when a table is under
// an extrinsic constraint (e.g., being stretched by its parent, or forced to a
// fixed block-size), we need to subtract the block size of all the <caption>
// elements from the available block size.
CORE_EXPORT LayoutUnit ComputeBlockSizeForFragment(
    const ConstraintSpace&,
    const BlockNode&,
    const BoxStrut& border_padding,
    LayoutUnit intrinsic_size,
    LayoutUnit inline_size,
    LayoutUnit override_available_size = kIndefiniteSize);

LayoutUnit ComputeInlineSizeForFragmentInternal(
    const ConstraintSpace& space,
    const BlockNode& node,
    const BoxStrut& border_padding,
    MinMaxSizesFunctionRef min_max_sizes_func);

CORE_EXPORT LayoutUnit
ComputeInlineSizeForFragment(const ConstraintSpace& space,
                             const BlockNode& node,
                             const BoxStrut& border_padding,
                             MinMaxSizesFunctionRef min_max_sizes_func);

// Returns inline size of the node's border box by resolving the computed value
// in `style.logicalWidth` to a `LayoutUnit`, adding border and padding, then
// constraining the result by the resolved min and max logical width from the
// `ComputedStyle` object. Calls `ComputeMinMaxSizes` if needed.
//
// `override_min_max_sizes_for_test` is provided *solely* for use by unit tests.
inline LayoutUnit ComputeInlineSizeForFragment(
    const ConstraintSpace& space,
    const BlockNode& node,
    const BoxStrut& border_padding,
    const MinMaxSizes* override_min_max_sizes_for_test = nullptr) {
  auto MinMaxSizesFunc = [&](SizeType type) -> MinMaxSizesResult {
    if (override_min_max_sizes_for_test) [[unlikely]] {
      return MinMaxSizesResult(*override_min_max_sizes_for_test,
                               /* depends_on_block_constraints */ false);
    }
    return node.ComputeMinMaxSizes(space.GetWritingMode(), type, space);
  };

  return ComputeInlineSizeForFragment(space, node, border_padding,
                                      MinMaxSizesFunc);
}

// Similar to |ComputeInlineSizeForFragment| but for determining the "used"
// inline-size for a table fragment. See:
// https://drafts.csswg.org/css-tables-3/#used-width-of-table
CORE_EXPORT LayoutUnit ComputeUsedInlineSizeForTableFragment(
    const ConstraintSpace& space,
    const BlockNode&,
    const BoxStrut& border_padding,
    const MinMaxSizes& table_grid_min_max_sizes);

LayoutUnit ComputeInitialBlockSizeForFragment(
    const ConstraintSpace&,
    const BlockNode&,
    const BoxStrut& border_padding,
    LayoutUnit intrinsic_size,
    LayoutUnit inline_size,
    LayoutUnit override_available_size = kIndefiniteSize);

// Calculates default content size for html and body elements in quirks mode.
// Returns |kIndefiniteSize| in all other cases.
CORE_EXPORT LayoutUnit
CalculateDefaultBlockSize(const ConstraintSpace& space,
                          const BlockNode& node,
                          const BlockBreakToken* break_token,
                          const BoxStrut& border_scrollbar_padding);

// Flex layout is interested in ignoring lengths in a particular axis. This
// enum is used to control this behaviour.
enum class ReplacedSizeMode {
  kNormal,
  kIgnoreInlineLengths,  // Used for determining the min/max content size.
  kIgnoreBlockLengths    // Used for determining the "intrinsic" block-size.
};

// Computes the size for a replaced element. See:
// https://www.w3.org/TR/CSS2/visudet.html#inline-replaced-width
// https://www.w3.org/TR/CSS2/visudet.html#inline-replaced-height
// https://www.w3.org/TR/CSS22/visudet.html#min-max-widths
// https://drafts.csswg.org/css-sizing-3/#intrinsic-sizes
//
// This will handle both intrinsic, and layout calculations depending on the
// space provided. (E.g. if the available inline-size is indefinite it will
// return the intrinsic size).
CORE_EXPORT LogicalSize
ComputeReplacedSize(const BlockNode&,
                    const ConstraintSpace&,
                    const BoxStrut& border_padding,
                    ReplacedSizeMode = ReplacedSizeMode::kNormal);

// Based on available inline size, CSS computed column-width, CSS computed
// column-count and CSS used column-gap, return CSS used column-count.
// If computed column-count is auto, pass 0 as |computed_count|.
CORE_EXPORT int ResolveUsedColumnCount(int computed_count,
                                       LayoutUnit computed_size,
                                       LayoutUnit used_gap,
                                       LayoutUnit available_size);
CORE_EXPORT int ResolveUsedColumnCount(const ComputedStyle&,
                                       LayoutUnit available_size);

// Based on available inline size, CSS computed column-width, CSS computed
// column-count and CSS used column-gap, return CSS used column-width.
CORE_EXPORT LayoutUnit ResolveUsedColumnInlineSize(int computed_count,
                                                   LayoutUnit computed_size,
                                                   LayoutUnit used_gap,
                                                   LayoutUnit available_size);
CORE_EXPORT LayoutUnit ResolveUsedColumnInlineSize(const ComputedStyle&,
                                                   LayoutUnit available_size);

// Return the used value of `column-gap` if it is a `<length-percentage>`.
// Otherwise, if it's `normal`, whose resolution is algorithm-specific,
// std::nullopt is returned.
std::optional<LayoutUnit> ResolveColumnGapLength(const ComputedStyle&,
                                                 LayoutUnit available_size);

CORE_EXPORT LayoutUnit ResolveColumnGapForMulticol(const ComputedStyle&,
                                                   LayoutUnit available_size);

// Return the used value of `row-gap` if it is a `<length-percentage>`.
// Otherwise, if it's `normal`, whose resolution is algorithm-specific,
// std::nullopt is returned.
std::optional<LayoutUnit> ResolveRowGapLength(const ComputedStyle&,
                                              LayoutUnit available_size);

LayoutUnit ResolveRowGapForMulticol(const ComputedStyle&,
                                    LayoutUnit available_size);

CORE_EXPORT LayoutUnit ColumnInlineProgression(const ComputedStyle&,
                                               LayoutUnit available_size);

// Compute physical margins.
CORE_EXPORT PhysicalBoxStrut
ComputePhysicalMargins(const ComputedStyle&,
                       PhysicalSize percentage_resolution_size);

inline PhysicalBoxStrut ComputePhysicalMargins(
    const ComputedStyle& style,
    LogicalSize percentage_resolution_size) {
  if (!style.MayHaveMargin()) {
    return PhysicalBoxStrut();
  }

  // This function may be called for determining intrinsic margins, clamp
  // indefinite %-sizes to zero. See:
  // https://drafts.csswg.org/css-sizing-3/#min-percentage-contribution
  percentage_resolution_size =
      percentage_resolution_size.ClampIndefiniteToZero();

  PhysicalSize physical_resolution_size =
      ToPhysicalSize(percentage_resolution_size, style.GetWritingMode());
  return ComputePhysicalMargins(style, physical_resolution_size);
}

inline PhysicalBoxStrut ComputePhysicalMargins(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style) {
  LogicalSize percentage_resolution_size =
      constraint_space.MarginPaddingPercentageResolutionSize();
  return ComputePhysicalMargins(style, percentage_resolution_size);
}

// Compute margins for the specified ConstraintSpace.
CORE_EXPORT BoxStrut ComputeMarginsFor(const ConstraintSpace&,
                                       const ComputedStyle&,
                                       const ConstraintSpace& compute_for);

inline BoxStrut ComputeMarginsFor(
    const ComputedStyle& style,
    LogicalSize percentage_resolution_size,
    WritingDirectionMode container_writing_direction) {
  return ComputePhysicalMargins(style, percentage_resolution_size)
      .ConvertToLogical(container_writing_direction);
}

inline BoxStrut ComputeMarginsFor(
    const ComputedStyle& style,
    LayoutUnit percentage_resolution_inline_size,
    WritingDirectionMode container_writing_direction) {
  // Regular CSS boxes resolve all margin percentages against the inline-size of
  // the containing block.
  const LogicalSize resolution_size(percentage_resolution_inline_size,
                                    percentage_resolution_inline_size);
  return ComputePhysicalMargins(style, resolution_size)
      .ConvertToLogical(container_writing_direction);
}

inline BoxStrut ComputeMarginsFor(
    const ConstraintSpace& space,
    const ComputedStyle& style,
    WritingDirectionMode container_writing_direction) {
  return ComputePhysicalMargins(space, style)
      .ConvertToLogical(container_writing_direction);
}

// Compute margins for the style owner.
inline BoxStrut ComputeMarginsForSelf(const ConstraintSpace& constraint_space,
                                      const ComputedStyle& style) {
  if (!style.MayHaveMargin() || constraint_space.IsAnonymous())
    return BoxStrut();
  LogicalSize percentage_resolution_size =
      constraint_space.MarginPaddingPercentageResolutionSize();
  return ComputePhysicalMargins(style, percentage_resolution_size)
      .ConvertToLogical(style.GetWritingDirection());
}

// Compute line logical margins for the style owner.
//
// The "line" versions compute line-relative logical values. See LineBoxStrut
// for more details.
inline LineBoxStrut ComputeLineMarginsForSelf(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style) {
  if (!style.MayHaveMargin() || constraint_space.IsAnonymous())
    return LineBoxStrut();
  LogicalSize percentage_resolution_size =
      constraint_space.MarginPaddingPercentageResolutionSize();
  return ComputePhysicalMargins(style, percentage_resolution_size)
      .ConvertToLineLogical(style.GetWritingDirection());
}

// Compute line logical margins for the constraint space, in the visual order
// (always assumes LTR, ignoring the direction) for inline layout algorithm.
inline LineBoxStrut ComputeLineMarginsForVisualContainer(
    const ConstraintSpace& constraint_space,
    const ComputedStyle& style) {
  if (!style.MayHaveMargin() || constraint_space.IsAnonymous())
    return LineBoxStrut();
  LogicalSize percentage_resolution_size =
      constraint_space.MarginPaddingPercentageResolutionSize();
  return ComputePhysicalMargins(style, percentage_resolution_size)
      .ConvertToLineLogical(
          {constraint_space.GetWritingMode(), TextDirection::kLtr});
}

CORE_EXPORT BoxStrut ComputeBorders(const ConstraintSpace&, const BlockNode&);

CORE_EXPORT BoxStrut ComputeBordersForInline(const ComputedStyle&);

CORE_EXPORT BoxStrut ComputeNonCollapsedTableBorders(const ComputedStyle&);

inline LineBoxStrut ComputeLineBorders(const ComputedStyle& style) {
  return LineBoxStrut(ComputeBordersForInline(style),
                      style.IsFlippedLinesWritingMode());
}

CORE_EXPORT BoxStrut ComputeBordersForTest(const ComputedStyle& style);

CORE_EXPORT BoxStrut ComputePadding(const ConstraintSpace&,
                                    const ComputedStyle&);

inline LineBoxStrut ComputeLinePadding(const ConstraintSpace& constraint_space,
                                       const ComputedStyle& style) {
  return LineBoxStrut(ComputePadding(constraint_space, style),
                      style.IsFlippedLinesWritingMode());
}

// Compute the scrollbars and scrollbar gutters.
CORE_EXPORT BoxStrut ComputeScrollbarsForNonAnonymous(const BlockNode&);

inline BoxStrut ComputeScrollbars(const ConstraintSpace& space,
                                  const BlockNode& node) {
  if (space.IsAnonymous())
    return BoxStrut();

  return ComputeScrollbarsForNonAnonymous(node);
}

// Resolves any 'auto' margins in the inline dimension. All arguments are in
// the containing-block's writing-mode.
CORE_EXPORT void ResolveInlineAutoMargins(
    const ComputedStyle& child_style,
    const ComputedStyle& containing_block_style,
    LayoutUnit available_inline_size,
    LayoutUnit inline_size,
    BoxStrut* margins);

// Resolve auto margins in one dimension. May result in negative margins, if
// `additional_space` is negative.
// `start_result` and `end_result` will be left unmodified for non-auto margins.
void ResolveAutoMargins(Length start_length,
                        Length end_length,
                        LayoutUnit additional_space,
                        LayoutUnit* start_result,
                        LayoutUnit* end_result);

// Resolve all auto margins. May result in negative margins, if
// `additional_inline_space` or `additional_block_space` is negative. Non-auto
// strut values will be left unmodified.
void ResolveAutoMargins(Length inline_start_length,
                        Length inline_end_length,
                        Length block_start_length,
                        Length block_end_length,
                        LayoutUnit additional_inline_space,
                        LayoutUnit additional_block_space,
                        BoxStrut* margins);

// Calculate the adjustment needed for the line's left position, based on
// text-align, direction and amount of unused space.
CORE_EXPORT LayoutUnit LineOffsetForTextAlign(ETextAlign,
                                              TextDirection,
                                              LayoutUnit space_left);

inline LayoutUnit ConstrainByMinMax(LayoutUnit length,
                                    LayoutUnit min,
                                    LayoutUnit max) {
  return std::max(min, std::min(length, max));
}

CORE_EXPORT FragmentGeometry
CalculateInitialFragmentGeometry(const ConstraintSpace& space,
                                 const BlockNode& node,
                                 const BlockBreakToken* break_token,
                                 MinMaxSizesFunctionRef min_max_sizes_func,
                                 bool is_intrinsic = false);

// Calculates the initial (pre-layout) fragment geometry given a node, and a
// constraint space.
// The "pre-layout" block-size may be indefinite, as we'll only have enough
// information to determine this post-layout.
// Setting |is_intrinsic| to true will avoid calculating the inline-size, and
// is typically used within the |BlockNode::ComputeMinMaxSizes| pass (as to
// determine the inline-size, we'd need to compute the min/max sizes, which in
// turn would call this function).
CORE_EXPORT FragmentGeometry
CalculateInitialFragmentGeometry(const ConstraintSpace&,
                                 const BlockNode&,
                                 const BlockBreakToken*,
                                 bool is_intrinsic = false);

// Shrinks the logical |size| by |insets|.
LogicalSize ShrinkLogicalSize(LogicalSize size, const BoxStrut& insets);

// Calculates the available size that children of the node should use.
LogicalSize CalculateChildAvailableSize(
    const ConstraintSpace&,
    const BlockNode& node,
    const LogicalSize border_box_size,
    const BoxStrut& border_scrollbar_padding);

// Calculates the percentage resolution size that children of the node should
// use.
LogicalSize CalculateChildPercentageSize(
    const ConstraintSpace&,
    const BlockNode node,
    const LogicalSize child_available_size);

// Calculates the percentage resolution size that replaced children of the node
// should use.
LogicalSize CalculateReplacedChildPercentageSize(
    const ConstraintSpace&,
    const BlockNode node,
    const LogicalSize child_available_size,
    const BoxStrut& border_scrollbar_padding,
    const BoxStrut& border_padding);

// The following function clamps the calculated size based on the node
// requirements. Specifically, this adjusts the size based on size containment
// and display locking status.
LayoutUnit ClampIntrinsicBlockSize(
    const ConstraintSpace&,
    const BlockNode&,
    const BlockBreakToken* break_token,
    const BoxStrut& border_scrollbar_padding,
    LayoutUnit current_intrinsic_block_size,
    std::optional<LayoutUnit> body_margin_block_sum = std::nullopt);

MinMaxSizesResult ComputeMinAndMaxContentContributionInternal(
    WritingMode parent_writing_mode,
    const BlockNode& child,
    const ConstraintSpace& space,
    MinMaxSizesFunctionRef min_max_sizes_func);

// For the given |child|, computes the min and max content contribution
// (https://drafts.csswg.org/css-sizing/#contributions).
//
// This is similar to `ComputeInlineSizeForFragment` except that it does not
// require a constraint space (percentage sizes as well as auto margins compute
// to zero) and an auto inline-size resolves to the respective min/max content
// size.
//
// Note that if the writing mode of the child is orthogonal to that of the
// parent, we'll still return the inline min/max contribution in the writing
// mode of the parent (i.e. typically something based on the preferred *block*
// size of the child).
MinMaxSizesResult ComputeMinAndMaxContentContribution(
    const ComputedStyle& parent_style,
    const BlockNode& child,
    const ConstraintSpace& space,
    const MinMaxSizesFloatInput float_input = MinMaxSizesFloatInput());

// Similar to `ComputeMinAndMaxContentContribution` but ignores the writing mode
// of the parent, and instead computes the contribution relative to the child's
// own writing mode.
MinMaxSizesResult ComputeMinAndMaxContentContributionForSelf(
    const BlockNode& child,
    const ConstraintSpace& space);

// Same as above, but allows a custom function to compute min/max sizes.
MinMaxSizesResult ComputeMinAndMaxContentContributionForSelf(
    const BlockNode& child,
    const ConstraintSpace& space,
    MinMaxSizesFunctionRef min_max_sizes_func);

// Used for unit-tests.
CORE_EXPORT MinMaxSizes
ComputeMinAndMaxContentContributionForTest(WritingMode writing_mode,
                                           const BlockNode&,
                                           const ConstraintSpace&,
                                           const MinMaxSizes&);

// This function checks if the inline size of this node has to be calculated
// without considering children. If so, it returns the calculated size.
// Otherwise, it returns std::nullopt and the caller has to compute the size
// itself.
std::optional<MinMaxSizesResult> CalculateMinMaxSizesIgnoringChildren(
    const BlockNode&,
    const BoxStrut& border_scrollbar_padding);

// Determine which scrollbars to freeze in the next layout pass. Scrollbars that
// appear will be frozen (while scrollbars that disappear will not). Input is
// the scrollbar situation before and after the previous layout pass, and the
// current freeze state (|freeze_horizontal|, |freeze_vertical|). Output is the
// new freeze state (|freeze_horizontal|, |freeze_vertical|). A scrollbar that
// was previously frozen will not become unfrozen.
void AddScrollbarFreeze(const BoxStrut& scrollbars_before,
                        const BoxStrut& scrollbars_after,
                        WritingDirectionMode,
                        bool* freeze_horizontal,
                        bool* freeze_vertical);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LENGTH_UTILS_H_
