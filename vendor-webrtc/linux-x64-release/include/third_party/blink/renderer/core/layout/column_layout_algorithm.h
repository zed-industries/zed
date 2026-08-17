// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_COLUMN_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_COLUMN_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"

namespace blink {

class BlockBreakToken;
class BlockNode;
class ColumnSpannerPath;
class ConstraintSpace;
enum class BreakStatus;
struct LogicalSize;
struct MarginStrut;

class CORE_EXPORT ColumnLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  explicit ColumnLayoutAlgorithm(const LayoutAlgorithmParams& params);

  const LayoutResult* Layout();

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&);

  // Create an empty column fragment, modeled after an existing column. The
  // resulting column may then be used and mutated by the out-of-flow layout
  // code, to add out-of-flow descendants.
  static const PhysicalBoxFragment& CreateEmptyColumn(
      const BlockNode&,
      const ConstraintSpace& parent_space,
      const PhysicalBoxFragment& previous_column);

 private:
  MinMaxSizesResult ComputeSpannersMinMaxSizes(
      const BlockNode& search_parent) const;

  // Lay out as many children as we can. If |kNeedsEarlierBreak| is returned, it
  // means that we ran out of space at an unappealing location, and need to
  // relayout and break earlier (because we have a better breakpoint there). If
  // |kBrokeBefore| is returned, it means that we need to break before the
  // multicol container, and retry in the next fragmentainer.
  BreakStatus LayoutChildren();

  // Lay out and fragment content into columns. Keep going until done, out of
  // space in any outer fragmentation context, or until a column spanner is
  // found.
  const LayoutResult* LayoutFragmentationContext(
      const BlockBreakToken* next_column_token,
      MarginStrut*);

  // Lay out one row of columns. The layout result returned is for the last
  // column that was laid out. The rows themselves don't create fragments. If
  // we're in a nested fragmentation context, and a break is inserted before the
  // row, nullptr is returned.
  const LayoutResult* LayoutRow(const BlockBreakToken* next_column_token,
                                LayoutUnit row_offset,
                                LayoutUnit miminum_column_block_size,
                                MarginStrut*);

  // Lay out a column spanner. The return value will tell whether to break
  // before the spanner or not. If |BreakStatus::kContinue| is returned, and
  // no break token was set, it means that we can proceed to the next row of
  // columns.
  BreakStatus LayoutSpanner(BlockNode spanner_node,
                            const BlockBreakToken* break_token,
                            MarginStrut*);

  // Attempt to position the list-item marker (if any) beside the child
  // fragment. This requires the fragment to have a baseline. If it doesn't,
  // we'll keep the unpositioned marker around, so that we can retry with a
  // later fragment (if any). If we reach the end of layout and still have an
  // unpositioned marker, it can be placed by calling
  // PositionAnyUnclaimedListMarker().
  void AttemptToPositionListMarker(const PhysicalBoxFragment& child_fragment,
                                   LayoutUnit block_offset);

  // At the end of layout, if no column or spanner were able to position the
  // list-item marker, position the marker at the beginning of the multicol
  // container.
  void PositionAnyUnclaimedListMarker();

  // Propagate the baseline from the given |child| if needed.
  void PropagateBaselineFromChild(const PhysicalBoxFragment& child,
                                  LayoutUnit block_offset);

  // Calculate the smallest possible block-size for columns, based on the
  // content. For column balancing this will be the initial size we'll try with
  // when actually lay out the columns (and then stretch the columns and re-lay
  // out until the desired result is achieved). For column-fill:auto and
  // unconstrained block-size, we also need to go through this, since we need to
  // know the column block-size before performing "real" layout, since all
  // columns in a row need to have the same block-size.
  LayoutUnit ResolveColumnAutoBlockSize(
      const LogicalSize& column_size,
      LayoutUnit row_offset,
      LayoutUnit available_outer_space,
      const BlockBreakToken* child_break_token,
      bool balance_columns);

  LayoutUnit ResolveColumnAutoBlockSizeInternal(
      const LogicalSize& column_size,
      LayoutUnit row_offset,
      LayoutUnit available_outer_space,
      const BlockBreakToken* child_break_token,
      bool balance_columns);

  LayoutUnit ConstrainColumnBlockSize(LayoutUnit size,
                                      LayoutUnit row_offset,
                                      LayoutUnit available_outer_space) const;
  LayoutUnit CurrentContentBlockOffset(LayoutUnit border_box_row_offset) const {
    return border_box_row_offset - BorderScrollbarPadding().block_start;
  }

  // Get the percentage resolution size to use for column content (i.e. not
  // spanners).
  LogicalSize ColumnPercentageResolutionSize() const {
    // Percentage block-size on children is resolved against the content-box of
    // the multicol container (just like in regular block layout), while
    // percentage inline-size is restricted by the columns.
    return LogicalSize(column_inline_size_, ChildAvailableSize().block_size);
  }

  bool ShouldWrapColumns() const {
    if (Style().ColumnWrap() == EColumnWrap::kWrap) {
      return true;
    }
    if (Style().ColumnWrap() == EColumnWrap::kNowrap) {
      return false;
    }
    DCHECK_EQ(Style().ColumnWrap(), EColumnWrap::kAuto);
    return !Style().HasAutoColumnHeight();
  }

  ConstraintSpace CreateConstraintSpaceForBalancing(
      const LogicalSize& column_size) const;
  ConstraintSpace CreateConstraintSpaceForSpanner(
      const BlockNode& spanner,
      LayoutUnit block_offset) const;
  ConstraintSpace CreateConstraintSpaceForMinMax() const;

  // The sum of all the current column children's block-sizes, as if they were
  // stacked, including any block-size that is added as a result of
  // ClampedToValidFragmentainerCapacity().
  LayoutUnit TotalColumnBlockSize() const;

  const ColumnSpannerPath* spanner_path_ = nullptr;

  int used_column_count_;
  LayoutUnit column_inline_size_;
  LayoutUnit column_inline_progression_;

  // The remaining space available to columns in the multicol container, if
  // block-size isn't auto.
  LayoutUnit remaining_content_block_size_;

  LayoutUnit intrinsic_block_size_;
  LayoutUnit tallest_unbreakable_block_size_;
  bool is_constrained_by_outer_fragmentation_context_ = false;

  // This will be set during (outer) block fragmentation once we've processed
  // the first piece of content of the multicol container. It is used to check
  // if we're at a valid class A  breakpoint (between block-level siblings).
  bool has_processed_first_child_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_COLUMN_LAYOUT_ALGORITHM_H_
