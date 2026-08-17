// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/flex/flex_break_token_data.h"
#include "third_party/blink/renderer/core/layout/flex/flex_item.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"

namespace blink {

class BlockBreakToken;
class BlockNode;
struct DevtoolsFlexInfo;
struct FlexItemData;

class CORE_EXPORT FlexLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  explicit FlexLayoutAlgorithm(
      const LayoutAlgorithmParams& params,
      const HashMap<wtf_size_t, LayoutUnit>* cross_size_adjustments = nullptr);
  ~FlexLayoutAlgorithm() { flex_items_.clear(); }

  void SetupRelayoutData(const FlexLayoutAlgorithm& previous, RelayoutType);

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&);
  const LayoutResult* Layout();

  const GapGeometry* GetGapGeometryForTest() {
    return container_builder_.GetGapGeometryForTest();
  }

 private:
  const LayoutResult* LayoutInternal();

  void PlaceFlexItems(
      FlexLineVector* flex_lines,
      HeapVector<Member<LayoutBox>>* oof_children,
      LayoutUnit* total_intrinsic_block_size,
      bool is_computing_multiline_column_intrinsic_size = false);

  bool DoesItemComputedCrossSizeHaveAuto(const BlockNode& child) const;
  bool DoesItemStretch(const BlockNode& child, ItemPosition alignment) const;
  // This checks for one of the scenarios where a flex-item box has a definite
  // size that would be indefinite if the box weren't a flex item.
  // See https://drafts.csswg.org/css-flexbox/#definite-sizes
  bool WillChildCrossSizeBeContainerCrossSize(const BlockNode& child,
                                              ItemPosition alignment) const;

  bool IsContainerCrossSizeDefinite() const;

  enum class Phase { kLayout, kRowIntrinsicSize, kColumnWrapIntrinsicSize };
  ConstraintSpace BuildSpaceForIntrinsicInlineSize(
      const BlockNode& flex_item,
      ItemPosition alignment) const;
  ConstraintSpace BuildSpaceForFlexBasis(const BlockNode& flex_item) const;
  ConstraintSpace BuildSpaceForIntrinsicBlockSize(
      const BlockNode& flex_item,
      ItemPosition alignment,
      std::optional<LayoutUnit> override_inline_size) const;
  // |line_cross_size_for_stretch| should only be set when running the final
  // layout pass for stretch, when the line cross size is definite.
  // |block_offset_for_fragmentation| should only be set when running the final
  // layout pass for fragmentation. Both may be set at the same time.
  ConstraintSpace BuildSpaceForLayout(
      const BlockNode& flex_item_node,
      ItemPosition alignment,
      LayoutUnit item_main_axis_final_size,
      bool is_initial_block_size_indefinite,
      std::optional<LayoutUnit> override_inline_size = std::nullopt,
      std::optional<LayoutUnit> line_cross_size_for_stretch = std::nullopt,
      std::optional<LayoutUnit> block_offset_for_fragmentation = std::nullopt,
      bool min_block_size_should_encompass_intrinsic_size = false) const;

  void ConstructAndAppendFlexItems(
      Phase phase,
      HeapVector<Member<LayoutBox>>* oof_children = nullptr);
  void ApplyReversals(FlexLineVector* flex_lines);
  LayoutResult::EStatus GiveItemsFinalPositionAndSize(
      FlexLineVector* flex_lines,
      Vector<EBreakBetween>* row_break_between_outputs);
  LayoutResult::EStatus GiveItemsFinalPositionAndSizeForFragmentation(
      FlexLineVector* flex_lines,
      Vector<EBreakBetween>* row_break_between_outputs,
      FlexBreakTokenData::FlexBreakBeforeRow* break_before_row,
      LayoutUnit* total_intrinsic_block_size);
  LayoutResult::EStatus PropagateFlexItemInfo(
      const FlexItem&,
      const PhysicalBoxFragment&,
      const PhysicalBoxStrut& physical_margins,
      wtf_size_t flex_line_idx,
      LogicalOffset offset);

  StyleContentAlignmentData ResolvedJustifyContent() const;

  ItemPosition ResolvedAlignSelf(const ComputedStyle& child_style,
                                 bool is_out_of_flow = false) const;

  // This is same method as FlexItem but we need that logic before FlexItem is
  // constructed.
  LayoutUnit MainAxisContentExtent(LayoutUnit sum_hypothetical_main_size) const;

  // Returns the position of the baseline, given a physical fragment.
  LayoutUnit BaselineAscent(const FlexItem&, const PhysicalBoxFragment&) const;

  // If we should apply the automatic minimum size, see:
  // See: https://drafts.csswg.org/css-flexbox/#min-size-auto
  bool ShouldApplyAutoMinSize(const BlockNode&) const;

  void HandleOutOfFlowPositionedItems(
      LayoutUnit total_intrinsic_block_size,
      HeapVector<Member<LayoutBox>>& oof_children);

  // Set reading flow so they can be accessed by LayoutBox.
  void SetReadingFlowNodes(const FlexLineVector& flex_lines);

  MinMaxSizesResult ComputeMinMaxSizeOfRowContainer();
  MinMaxSizesResult ComputeMinMaxSizeOfMultilineColumnContainer();

  // Return the amount of block space available in the current fragmentainer
  // for the node being laid out by this algorithm.
  LayoutUnit FragmentainerSpaceAvailable(LayoutUnit block_offset) const;

  // Consume all remaining fragmentainer space. This happens when we decide to
  // break before a child.
  //
  // https://www.w3.org/TR/css-break-3/#box-splitting
  void ConsumeRemainingFragmentainerSpace(
      LayoutUnit offset_in_stitched_container,
      FlexLine* flex_line,
      const FlexColumnBreakInfo* column_break_info = nullptr);

  BreakStatus BreakBeforeChildIfNeeded(
      LayoutInputNode child,
      const LayoutResult& layout_result,
      LayoutUnit fragmentainer_block_offset,
      bool has_container_separation,
      bool is_row_item,
      FlexColumnBreakInfo* flex_column_break_info) {
    return ::blink::BreakBeforeChildIfNeeded(
        GetConstraintSpace(), child, layout_result, fragmentainer_block_offset,
        FragmentainerCapacityForChildren(), has_container_separation,
        &container_builder_, is_row_item, flex_column_break_info);
  }

  // Insert a fragmentainer break before a row if necessary. Rows do not produce
  // a layout result, so when breaking before a row, we will insert a
  // fragmentainer break before the first child in a row. |child| should be
  // those associated with the first child in the row. |row|,
  // |row_block_offset|, |row_break_between|, |row_index|,
  // |has_container_separation| and |is_first_for_row| are specific to the row
  // itself. See
  // |::blink::BreakBeforeChildIfNeeded()| for more documentation.
  BreakStatus BreakBeforeRowIfNeeded(const FlexLine& row,
                                     LayoutUnit row_block_offset,
                                     EBreakBetween row_break_between,
                                     wtf_size_t row_index,
                                     LayoutInputNode child,
                                     bool has_container_separation,
                                     bool is_first_for_row);

  // Move past the breakpoint before the row, if possible, and return true. Also
  // update the appeal of breaking before the row (if we're not going
  // to break before it). If false is returned, it means that we need to break
  // before the row (or even earlier). See |::blink::MovePastBreakpoint()| for
  // more documentation.
  bool MovePastRowBreakPoint(BreakAppeal appeal_before,
                             LayoutUnit fragmentainer_block_offset,
                             LayoutUnit row_block_size,
                             wtf_size_t row_index,
                             bool has_container_separation,
                             bool breakable_at_start_of_container);

  // Add an early break for the column at the provided |index|.
  void AddColumnEarlyBreak(EarlyBreak* breakpoint, wtf_size_t index);

  // Add the amount an item expanded by to the item offset adjustment of the
  // flex line at the index directly after |flex_line_idx|, if there is one.
  void AdjustOffsetForNextLine(FlexLineVector* flex_lines,
                               wtf_size_t flex_line_idx,
                               LayoutUnit item_expansion) const;

  // If a flex item expands past the row cross-size as a result of
  // fragmentation, we will abort and re-run layout with the appropriate row
  // cross-size adjustments.
  const LayoutResult* RelayoutWithNewRowSizes();

  // Used to determine when to allow an item to expand as a result of
  // fragmentation.
  bool MinBlockSizeShouldEncompassIntrinsicSize(const FlexItemData& item) const;

  HeapVector<FlexItem, 4> flex_items_;

  // Used when determining the max-content width of a column-wrap flex
  // container.
  LayoutUnit largest_min_content_contribution_;

  const bool is_webkit_box_;
  const bool is_column_;
  const bool is_wrap_reverse_;
  const bool is_reverse_direction_;
  const bool is_multi_line_;
  const bool is_horizontal_flow_;
  const bool is_cross_size_definite_;
  const LogicalSize child_percentage_size_;

  const LayoutUnit gap_between_items_;
  const LayoutUnit gap_between_lines_;

  bool has_column_percent_flex_basis_ = false;
  bool ignore_child_scrollbar_changes_ = false;

  // This will be set during block fragmentation once we've processed the first
  // flex item in a given line. It is used to check if we're at a valid class A
  // or C breakpoint within a column flex container.
  wtf_size_t last_line_idx_to_process_first_child_ = kNotFound;
  // This will be set during block fragmentation once we've processed the first
  // flex line. It is used to check if we're at a valid class A or C breakpoint
  // within a row flex container.
  bool has_processed_first_line_ = false;

  std::unique_ptr<DevtoolsFlexInfo> layout_info_for_devtools_;

  // The block size of the entire flex container (ignoring any fragmentation).
  LayoutUnit total_block_size_;
  // This will be the intrinsic block size in the current fragmentainer, if
  // inside a fragmentation context. Otherwise, it will represent the intrinsic
  // block size for the entire flex container.
  LayoutUnit intrinsic_block_size_;

  // Only one early break is supported per container. However, we may need to
  // return to an early break within multiple flex columns. This stores the
  // early breaks per column to be used when aborting layout.
  HeapVector<Member<EarlyBreak>> column_early_breaks_;

  // If an item expands past the row block-end, we will re-run layout with the
  // new cross size. Keep track of each such flex line index mapped to how much
  // it should expand by in the next layout pass.
  HashMap<wtf_size_t, LayoutUnit> row_cross_size_updates_;

  // If set, that means that we are re-running layout with updated row
  // cross-sizes. See |row_cross_size_updates_| for what this maps to.
  const HashMap<wtf_size_t, LayoutUnit>* cross_size_adjustments_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_LAYOUT_ALGORITHM_H_
