// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CONSTRAINT_SPACE_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CONSTRAINT_SPACE_BUILDER_H_

#include <optional>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/constraint_space.h"
#include "third_party/blink/renderer/core/layout/floats_utils.h"
#include "third_party/blink/renderer/core/layout/geometry/bfc_offset.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/layout/space_utils.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ExclusionSpace;

class CORE_EXPORT ConstraintSpaceBuilder final {
  STACK_ALLOCATED();

 public:
  // The setters on this builder are in the writing mode of parent_space.
  ConstraintSpaceBuilder(const ConstraintSpace& parent_space,
                         WritingDirectionMode writing_direction,
                         bool is_new_fc,
                         bool adjust_inline_size_if_needed = true)
      : ConstraintSpaceBuilder(parent_space.GetWritingMode(),
                               writing_direction,
                               is_new_fc,
                               /* force_orthogonal_writing_mode_root */ false,
                               adjust_inline_size_if_needed) {
    if (parent_space.ShouldPropagateChildBreakValues())
      SetShouldPropagateChildBreakValues();
    if (parent_space.ShouldRepeat())
      SetShouldRepeat(true);
    SetIsInsideRepeatableContent(parent_space.IsInsideRepeatableContent());
  }

  // The setters on this builder are in the writing mode of parent_writing_mode.
  //
  // forced_orthogonal_writing_mode_root is set for constraint spaces created
  // directly from a LayoutObject. In this case parent_writing_mode isn't
  // actually the parent's, it's the same as out_writing_mode.
  // When this occurs we would miss setting the kOrthogonalWritingModeRoot flag
  // unless we force it.
  ConstraintSpaceBuilder(WritingMode parent_writing_mode,
                         WritingDirectionMode writing_direction,
                         bool is_new_fc,
                         bool force_orthogonal_writing_mode_root = false,
                         bool adjust_inline_size_if_needed = true)
      : space_(writing_direction),
        is_in_parallel_flow_(
            IsParallelWritingMode(parent_writing_mode,
                                  writing_direction.GetWritingMode())),
        is_new_fc_(is_new_fc),
        force_orthogonal_writing_mode_root_(force_orthogonal_writing_mode_root),
        adjust_inline_size_if_needed_(adjust_inline_size_if_needed) {
    space_.bitfields_.is_new_formatting_context = is_new_fc_;
    space_.bitfields_.is_orthogonal_writing_mode_root =
        !is_in_parallel_flow_ || force_orthogonal_writing_mode_root_;
  }

  // If inline size is indefinite, use the fallback size for available inline
  // size for orthogonal flow roots. See:
  // https://www.w3.org/TR/css-writing-modes-3/#orthogonal-auto
  void AdjustInlineSizeIfNeeded(LayoutUnit* inline_size) {
    DCHECK(!is_in_parallel_flow_);
    DCHECK(adjust_inline_size_if_needed_);
    if (*inline_size != kIndefiniteSize)
      return;
    DCHECK_NE(orthogonal_fallback_inline_size_, kIndefiniteSize);
    *inline_size = orthogonal_fallback_inline_size_;
    space_.EnsureRareData()->uses_orthogonal_fallback_inline_size = true;
  }

  // |available_size| is logical for the writing-mode of the container.
  void SetAvailableSize(LogicalSize available_size) {
#if DCHECK_IS_ON()
    is_available_size_set_ = true;
    DCHECK(!is_percentage_resolution_size_set_);
#endif

    if (is_in_parallel_flow_) [[likely]] {
      space_.available_size_ = space_.percentage_size_ = available_size;
    } else {
      if (adjust_inline_size_if_needed_) {
        AdjustInlineSizeIfNeeded(&available_size.block_size);
      }
      space_.available_size_ = space_.percentage_size_ = {
          available_size.block_size, available_size.inline_size};
    }
  }

  // Set percentage resolution size.
  void SetPercentageResolutionSize(LogicalSize percentage_size) {
#if DCHECK_IS_ON()
    is_percentage_resolution_size_set_ = true;
#endif
    if (is_in_parallel_flow_) [[likely]] {
      space_.percentage_size_ = percentage_size;
    } else {
      if (adjust_inline_size_if_needed_) {
        AdjustInlineSizeIfNeeded(&percentage_size.block_size);
      }
      space_.percentage_size_ = {percentage_size.block_size,
                                 percentage_size.inline_size};
    }
  }

  // Sets the percentage resolution size for a replaced child.
  // Replaced children within a table-cell have *slightly* different rules for
  // percentage resolution. This should only be used for passing this
  // information to inline-layout.
  void SetReplacedChildPercentageResolutionSize(
      LogicalSize replaced_child_percentage_resolution_size) {
#if DCHECK_IS_ON()
    DCHECK(is_percentage_resolution_size_set_);
    DCHECK(is_in_parallel_flow_);
#endif

    // We don't store the replaced percentage resolution inline size, so we
    // need it to be the same as the regular percentage resolution inline size.
    DCHECK_EQ(replaced_child_percentage_resolution_size.inline_size,
              space_.PercentageResolutionInlineSize());
    DCHECK(replaced_child_percentage_resolution_size.block_size !=
           kIndefiniteSize);
    DCHECK(replaced_child_percentage_resolution_size.block_size !=
           space_.PercentageResolutionBlockSize());

    space_.EnsureRareData()->replaced_child_percentage_resolution_block_size =
        replaced_child_percentage_resolution_size.block_size;
  }

  // Set the fallback available inline-size for an orthogonal child. The size is
  // the inline size in the writing mode of the orthogonal child.
  void SetOrthogonalFallbackInlineSize(LayoutUnit size) {
    orthogonal_fallback_inline_size_ = size;
  }

  void SetPageName(const AtomicString& name) {
    if (!name && !space_.rare_data_)
      return;
    space_.EnsureRareData()->page_name = name;
  }

  void SetFragmentainerBlockSize(LayoutUnit size) {
#if DCHECK_IS_ON()
    DCHECK(!is_fragmentainer_block_size_set_);
    is_fragmentainer_block_size_set_ = true;
#endif
    if (size != kIndefiniteSize)
      space_.EnsureRareData()->fragmentainer_block_size = size;
  }

  // This function may be called after having set available size (and thus
  // converted to the destination writing mode, if necessary).
  void SetFragmentainerBlockSizeFromAvailableSize() {
#if DCHECK_IS_ON()
    DCHECK(is_available_size_set_);
#endif
    SetFragmentainerBlockSize(space_.AvailableSize().block_size);
  }

  // Shrink the fragmentainer block-size, to reserve space for repeated table
  // headers and footers. If there's a repeated header, the argument to
  // SetFragmentainerOffset() also needs to be compensated for the block-size
  // taken up by the repeated header, so that offset 0 is exactly where the
  // non-repeated content starts / resumes after the repeated header.
  void ReserveSpaceInFragmentainer(LayoutUnit space) {
    if (!space_.HasBlockFragmentation()) {
      // It is possible to end up with a monolithic table section, even if
      // things like containment and overflow don't apply. -webkit-line-clamp
      // is at least one example.
      return;
    }
#if DCHECK_IS_ON()
    DCHECK(is_fragmentainer_block_size_set_);
#endif
    space_.rare_data_->fragmentainer_block_size -= space;
    space_.rare_data_->fragmentainer_block_size =
        space_.rare_data_->fragmentainer_block_size.ClampNegativeToZero();
  }

  void SetFragmentainerOffset(LayoutUnit offset) {
#if DCHECK_IS_ON()
    DCHECK(!is_fragmentainer_offset_set_);
    is_fragmentainer_offset_set_ = true;
#endif
    if (offset != LayoutUnit())
      space_.EnsureRareData()->fragmentainer_offset = offset;
  }

  void SetIsAtFragmentainerStart() {
    space_.EnsureRareData()->is_at_fragmentainer_start = true;
  }

  void SetShouldRepeat(bool b) { space_.EnsureRareData()->should_repeat = b; }

  void SetIsInsideRepeatableContent(bool b) {
    if (!b && !space_.rare_data_)
      return;
    space_.EnsureRareData()->is_inside_repeatable_content = b;
  }

  void DisableFurtherFragmentation() { space_.DisableFurtherFragmentation(); }
  void DisableMonolithicOverflowPropagation() {
    space_.DisableMonolithicOverflowPropagation();
  }

  void SetIsHiddenForPaint(bool is_hidden_for_paint) {
#if DCHECK_IS_ON()
    DCHECK(!is_hidden_for_paint_set_);
    is_hidden_for_paint_set_ = true;
#endif
    if (is_hidden_for_paint) {
      space_.bitfields_.is_hidden_for_paint = true;
    }
  }

  void SetIsFixedInlineSize(bool b) {
    if (is_in_parallel_flow_) [[likely]] {
      space_.bitfields_.is_fixed_inline_size = b;
    } else {
      space_.bitfields_.is_fixed_block_size = b;
    }
  }

  void SetIsFixedBlockSize(bool b) {
    if (is_in_parallel_flow_) [[likely]] {
      space_.bitfields_.is_fixed_block_size = b;
    } else {
      space_.bitfields_.is_fixed_inline_size = b;
    }
  }

  void SetIsInitialBlockSizeIndefinite(bool b) {
    if (is_in_parallel_flow_ || !force_orthogonal_writing_mode_root_)
        [[likely]] {
      space_.bitfields_.is_initial_block_size_indefinite = b;
    }
  }

  void SetInlineAutoBehavior(AutoSizeBehavior auto_behavior) {
    if (is_in_parallel_flow_) [[likely]] {
      space_.bitfields_.inline_auto_behavior =
          static_cast<unsigned>(auto_behavior);
    } else {
      space_.bitfields_.block_auto_behavior =
          static_cast<unsigned>(auto_behavior);
    }
  }

  void SetBlockAutoBehavior(AutoSizeBehavior auto_behavior) {
    if (is_in_parallel_flow_) [[likely]] {
      space_.bitfields_.block_auto_behavior =
          static_cast<unsigned>(auto_behavior);
    } else {
      space_.bitfields_.inline_auto_behavior =
          static_cast<unsigned>(auto_behavior);
    }
  }

  void SetIsPaintedAtomically(bool b) {
    space_.bitfields_.is_painted_atomically = b;
  }

  void SetFragmentationType(FragmentationType fragmentation_type) {
#if DCHECK_IS_ON()
    DCHECK(!is_block_direction_fragmentation_type_set_);
    is_block_direction_fragmentation_type_set_ = true;
#endif
    if (fragmentation_type != FragmentationType::kFragmentNone) {
      space_.EnsureRareData()->block_direction_fragmentation_type =
          fragmentation_type;
    }
  }

  void SetRequiresContentBeforeBreaking(bool b) {
    if (!b && !space_.rare_data_) {
      return;
    }
    space_.EnsureRareData()->requires_content_before_breaking = b;
  }

  void SetIsInsideBalancedColumns() {
    space_.EnsureRareData()->is_inside_balanced_columns = true;
  }

  void SetShouldIgnoreForcedBreaks() {
    space_.EnsureRareData()->should_ignore_forced_breaks = true;
  }

  void SetIsInColumnBfc() { space_.EnsureRareData()->is_in_column_bfc = true; }

  void SetIsPastBreak() { space_.EnsureRareData()->is_past_break = true; }

  void SetMinBlockSizeShouldEncompassIntrinsicSize() {
    space_.EnsureRareData()->min_block_size_should_encompass_intrinsic_size =
        true;
  }

  void SetMinBreakAppeal(BreakAppeal min_break_appeal) {
    if (!space_.rare_data_ && min_break_appeal == kBreakAppealLastResort) {
      return;
    }
    space_.EnsureRareData()->min_break_appeal = min_break_appeal;
  }

  void SetShouldPropagateChildBreakValues(
      bool propagate_child_break_values = true) {
    // Don't create rare data if `propagate_child_break_values` is already
    // false.
    if (!space_.rare_data_ && !propagate_child_break_values) {
      return;
    }
    space_.EnsureRareData()->propagate_child_break_values =
        propagate_child_break_values;
  }

  void SetIsTableCell(bool is_table_cell) {
    space_.EnsureRareData()->SetIsTableCell();
  }

  void SetIsRestrictedBlockSizeTableCell(bool b) {
    DCHECK(space_.IsTableCell());
    if (!b && !space_.rare_data_) {
      return;
    }
    space_.EnsureRareData()->is_restricted_block_size_table_cell = b;
  }

  void SetHideTableCellIfEmpty(bool b) {
    if (!b && !space_.rare_data_)
      return;
    space_.EnsureRareData()->hide_table_cell_if_empty = b;
  }

  void SetIsAnonymous(bool b) { space_.bitfields_.is_anonymous = b; }

  void SetUseFirstLineStyle(bool b) {
    space_.bitfields_.use_first_line_style = b;
  }

  void SetAdjoiningObjectTypes(AdjoiningObjectTypes adjoining_object_types) {
    if (!is_new_fc_) {
      space_.bitfields_.adjoining_object_types =
          static_cast<unsigned>(adjoining_object_types);
    }
  }

  void SetAncestorHasClearancePastAdjoiningFloats() {
    space_.bitfields_.ancestor_has_clearance_past_adjoining_floats = true;
  }

  void SetBaselineAlgorithmType(BaselineAlgorithmType type) {
    space_.bitfields_.baseline_algorithm_type = static_cast<unsigned>(type);
  }

  void SetCacheSlot(LayoutResultCacheSlot slot) {
    space_.bitfields_.cache_slot = static_cast<unsigned>(slot);
  }

  void SetBlockStartAnnotationSpace(LayoutUnit space) {
    if (space)
      space_.EnsureRareData()->SetBlockStartAnnotationSpace(space);
  }

  void SetIgnoreMarginsForStretch(WritingMode parent_mode,
                                  LineLogicalBoxSides sides) {
#if DCHECK_IS_ON()
    DCHECK(!is_ignored_margins_set_);
    is_ignored_margins_set_ = true;
#endif
    if (!sides.IsEmpty()) {
      space_.EnsureRareData()->ignore_margins_for_stretch =
          PhysicalBoxSides(sides, parent_mode)
              .ToLogical(space_.GetWritingDirection());
    }
  }

  void SetMarginStrut(const MarginStrut& margin_strut) {
#if DCHECK_IS_ON()
    DCHECK(!is_margin_strut_set_);
    is_margin_strut_set_ = true;
#endif
    if (!is_new_fc_ && margin_strut != MarginStrut()) {
      space_.EnsureRareData()->SetMarginStrut(margin_strut);
    }
  }

  void SetBfcOffset(const BfcOffset& bfc_offset) {
    if (!is_new_fc_) {
      space_.bfc_offset_ = bfc_offset;
    }
  }

  void SetOptimisticBfcBlockOffset(LayoutUnit optimistic_bfc_block_offset) {
#if DCHECK_IS_ON()
    DCHECK(!is_optimistic_bfc_block_offset_set_);
    is_optimistic_bfc_block_offset_set_ = true;
#endif
    if (!is_new_fc_) [[likely]] {
      space_.EnsureRareData()->SetOptimisticBfcBlockOffset(
          optimistic_bfc_block_offset);
    }
  }

  void SetForcedBfcBlockOffset(LayoutUnit forced_bfc_block_offset) {
#if DCHECK_IS_ON()
    DCHECK(!is_forced_bfc_block_offset_set_);
    is_forced_bfc_block_offset_set_ = true;
#endif
    DCHECK(!is_new_fc_);
    space_.EnsureRareData()->SetForcedBfcBlockOffset(forced_bfc_block_offset);
  }

  LayoutUnit ExpectedBfcBlockOffset() const {
    return space_.ExpectedBfcBlockOffset();
  }

  void SetClearanceOffset(LayoutUnit clearance_offset) {
#if DCHECK_IS_ON()
    DCHECK(!is_clearance_offset_set_);
    is_clearance_offset_set_ = true;
#endif
    if (!is_new_fc_ && clearance_offset != LayoutUnit::Min())
      space_.EnsureRareData()->SetClearanceOffset(clearance_offset);
  }

  void SetTableCellBorders(const BoxStrut& table_cell_borders,
                           WritingDirectionMode cell_writing_direction,
                           WritingDirectionMode table_writing_direction) {
#if DCHECK_IS_ON()
    DCHECK(!is_table_cell_borders_set_);
    is_table_cell_borders_set_ = true;
#endif
    if (table_cell_borders != BoxStrut()) {
      space_.EnsureRareData()->SetTableCellBorders(
          table_cell_borders.ConvertToPhysical(table_writing_direction)
              .ConvertToLogical(cell_writing_direction));
    }
  }

  void SetTableCellAlignmentBaseline(
      const std::optional<LayoutUnit>& table_cell_alignment_baseline) {
#if DCHECK_IS_ON()
    DCHECK(!is_table_cell_alignment_baseline_set_);
    is_table_cell_alignment_baseline_set_ = true;
#endif
    if (is_in_parallel_flow_ && table_cell_alignment_baseline) {
      space_.EnsureRareData()->SetTableCellAlignmentBaseline(
          *table_cell_alignment_baseline);
    }
  }

  void SetTableCellColumnIndex(wtf_size_t column_index) {
#if DCHECK_IS_ON()
    DCHECK(!is_table_cell_column_index_set_);
    is_table_cell_column_index_set_ = true;
#endif
    space_.EnsureRareData()->SetTableCellColumnIndex(column_index);
  }

  void SetIsTableCellWithCollapsedBorders(bool has_collapsed_borders) {
#if DCHECK_IS_ON()
    DCHECK(!is_table_cell_with_collapsed_borders_set_);
    is_table_cell_with_collapsed_borders_set_ = true;
#endif
    if (has_collapsed_borders) {
      space_.EnsureRareData()->SetIsTableCellWithCollapsedBorders(
          has_collapsed_borders);
    }
  }

  void SetIsTableCellChild(bool b) {
    space_.bitfields_.is_table_cell_child = b;
  }

  void SetIsRestrictedBlockSizeTableCellChild() {
    space_.bitfields_.is_restricted_block_size_table_cell_child = true;
  }

  void SetExclusionSpace(const ExclusionSpace& exclusion_space) {
    if (!is_new_fc_)
      space_.exclusion_space_ = exclusion_space;
  }

  void SetCustomLayoutData(
      scoped_refptr<SerializedScriptValue> custom_layout_data) {
#if DCHECK_IS_ON()
    DCHECK(!is_custom_layout_data_set_);
    is_custom_layout_data_set_ = true;
#endif
    if (custom_layout_data) {
      space_.EnsureRareData()->SetCustomLayoutData(
          std::move(custom_layout_data));
    }
  }

  void SetTableRowData(const TableConstraintSpaceData* table_data,
                       wtf_size_t row_index) {
#if DCHECK_IS_ON()
    DCHECK(!is_table_row_data_set_);
    is_table_row_data_set_ = true;
#endif
    space_.EnsureRareData()->SetTableRowData(std::move(table_data), row_index);
  }

  void SetTableSectionData(
      scoped_refptr<const TableConstraintSpaceData> table_data,
      wtf_size_t section_index) {
#if DCHECK_IS_ON()
    DCHECK(!is_table_section_data_set_);
    is_table_section_data_set_ = true;
#endif
    space_.EnsureRareData()->SetTableSectionData(std::move(table_data),
                                                 section_index);
  }

  void SetLineClampData(LineClampData data) {
#if DCHECK_IS_ON()
    DCHECK(!is_line_clamp_data_set_);
    is_line_clamp_data_set_ = true;
#endif
    DCHECK(!is_new_fc_);
    if (data.state != LineClampData::kDisabled) {
      space_.EnsureRareData()->SetLineClampData(data);
    }
  }

  void SetLineClampEndMarginStrut(MarginStrut end_margin_strut) {
#if DCHECK_IS_ON()
    DCHECK(!is_line_clamp_end_margin_strut_set_);
    is_line_clamp_end_margin_strut_set_ = true;
#endif
    DCHECK(!is_new_fc_);
    if (!end_margin_strut.IsEmpty()) {
      space_.EnsureRareData()->SetLineClampEndMarginStrut(end_margin_strut);
    }
  }

  void SetLineClampEndPadding(LayoutUnit end_padding) {
#if DCHECK_IS_ON()
    DCHECK(!is_line_clamp_end_padding_set_);
    is_line_clamp_end_padding_set_ = true;
#endif
    DCHECK(!is_new_fc_);
    if (end_padding) {
      space_.EnsureRareData()->SetLineClampEndPadding(end_padding);
    }
  }

  void SetShouldTextBoxTrimNodeStart(bool b) {
    if (b || space_.rare_data_) {
      space_.EnsureRareData()->should_text_box_trim_node_start = b;
    }
  }
  void SetShouldTextBoxTrimNodeEnd(bool b) {
    if (b || space_.rare_data_) {
      space_.EnsureRareData()->should_text_box_trim_node_end = b;
    }
  }
  void SetShouldTextBoxTrimFragmentainerStart(bool b) {
    if (b || space_.rare_data_) {
      space_.EnsureRareData()->should_text_box_trim_fragmentainer_start = b;
    }
  }
  void SetShouldTextBoxTrimFragmentainerEnd(bool b) {
    if (b || space_.rare_data_) {
      space_.EnsureRareData()->should_text_box_trim_fragmentainer_end = b;
    }
  }
  void SetShouldTextBoxTrimInsideWhenLineClamp(bool b) {
    if (b || space_.rare_data_) {
      space_.EnsureRareData()->should_text_box_trim_inside_when_line_clamp = b;
    }
  }

  void SetShouldForceTextBoxTrimEnd() { space_.SetShouldForceTextBoxTrimEnd(); }

  void SetDecorationPercentageResolutionType(
      DecorationPercentageResolutionType type) {
    space_.EnsureRareData()->decoration_percentage_resolution_type =
        static_cast<unsigned>(type);
  }

  void SetIsPushedByFloats() {
    space_.EnsureRareData()->is_pushed_by_floats = true;
  }

  void SetTargetStretchInlineSize(LayoutUnit target_stretch_inline_size) {
    DCHECK_GE(target_stretch_inline_size, LayoutUnit());
    space_.EnsureRareData()->SetTargetStretchInlineSize(
        target_stretch_inline_size);
  }

  void SetTargetStretchBlockSizes(
      ConstraintSpace::MathTargetStretchBlockSizes target_stretch_block_sizes) {
    DCHECK_GE(target_stretch_block_sizes.ascent, LayoutUnit());
    DCHECK_GE(target_stretch_block_sizes.descent, LayoutUnit());
    space_.EnsureRareData()->SetTargetStretchBlockSizes(
        target_stretch_block_sizes);
  }

  void SetGridLayoutSubtree(GridLayoutSubtree&& grid_layout_subtree) {
#if DCHECK_IS_ON()
    DCHECK(!is_grid_layout_subtree_set_);
    is_grid_layout_subtree_set_ = true;
#endif
    space_.EnsureRareData()->SetGridLayoutSubtree(
        std::move(grid_layout_subtree));
  }

  // Creates a new constraint space.
  const ConstraintSpace ToConstraintSpace() {
#if DCHECK_IS_ON()
    DCHECK(!to_constraint_space_called_)
        << "ToConstraintSpace should only be called once.";
    to_constraint_space_called_ = true;
#endif

    DCHECK(!is_new_fc_ || !space_.bitfields_.adjoining_object_types);
    DCHECK_EQ(space_.bitfields_.is_orthogonal_writing_mode_root,
              !is_in_parallel_flow_ || force_orthogonal_writing_mode_root_);

    DCHECK(!force_orthogonal_writing_mode_root_ || is_in_parallel_flow_)
        << "Forced and inferred orthogonal writing mode shouldn't happen "
           "simultaneously. Inferred means the constraints are in parent "
           "writing mode, forced means they are in child writing mode.";

    return std::move(space_);
  }

 private:
  ConstraintSpace space_;

  // Orthogonal writing mode roots may need a fallback, to prevent available
  // inline size from being indefinite, which isn't allowed. This is the
  // available inline size in the writing mode of the orthogonal child.
  LayoutUnit orthogonal_fallback_inline_size_ = kIndefiniteSize;

  bool is_in_parallel_flow_;
  bool is_new_fc_;
  bool force_orthogonal_writing_mode_root_;
  bool adjust_inline_size_if_needed_;

#if DCHECK_IS_ON()
  bool is_hidden_for_paint_set_ = false;
  bool is_available_size_set_ = false;
  bool is_percentage_resolution_size_set_ = false;
  bool is_fragmentainer_block_size_set_ = false;
  bool is_fragmentainer_offset_set_ = false;
  bool is_block_direction_fragmentation_type_set_ = false;
  bool is_margin_strut_set_ = false;
  bool is_optimistic_bfc_block_offset_set_ = false;
  bool is_forced_bfc_block_offset_set_ = false;
  bool is_clearance_offset_set_ = false;
  bool is_table_cell_borders_set_ = false;
  bool is_table_cell_alignment_baseline_set_ = false;
  bool is_table_cell_column_index_set_ = false;
  bool is_table_cell_with_collapsed_borders_set_ = false;
  bool is_custom_layout_data_set_ = false;
  bool is_line_clamp_data_set_ = false;
  bool is_line_clamp_end_padding_set_ = false;
  bool is_line_clamp_end_margin_strut_set_ = false;
  bool is_table_row_data_set_ = false;
  bool is_table_section_data_set_ = false;
  bool is_grid_layout_subtree_set_ = false;
  bool is_ignored_margins_set_ = false;

  bool to_constraint_space_called_ = false;
#endif
};

// This is a helper class for use in |LayoutAlgorithm::ComputeMinMaxSizes|.
// It exposes a subset of the |ConstraintSpace| builder methods. Additionally
// it sets the orthogonal fallback inline-size if needed.
class CORE_EXPORT MinMaxConstraintSpaceBuilder final {
  STACK_ALLOCATED();

 public:
  MinMaxConstraintSpaceBuilder(const ConstraintSpace& parent_space,
                               const ComputedStyle& parent_style,
                               const LayoutInputNode& child,
                               bool is_new_fc)
      : delegate_(parent_space,
                  child.Style().GetWritingDirection(),
                  is_new_fc) {
    SetOrthogonalFallbackInlineSizeIfNeeded(parent_style, child, &delegate_);
    delegate_.SetCacheSlot(LayoutResultCacheSlot::kMeasure);
    if (parent_space.IsInColumnBfc() && !child.CreatesNewFormattingContext())
      delegate_.SetIsInColumnBfc();
  }

  void SetAvailableBlockSize(LayoutUnit block_size) {
    delegate_.SetAvailableSize({kIndefiniteSize, block_size});
  }

  void SetPercentageResolutionBlockSize(LayoutUnit block_size) {
    delegate_.SetPercentageResolutionSize({kIndefiniteSize, block_size});
  }

  void SetReplacedChildPercentageResolutionBlockSize(LayoutUnit block_size) {
    delegate_.SetReplacedChildPercentageResolutionSize(
        {kIndefiniteSize, block_size});
  }

  void SetBlockAutoBehavior(AutoSizeBehavior auto_behavior) {
    delegate_.SetBlockAutoBehavior(auto_behavior);
  }

  const ConstraintSpace ToConstraintSpace() {
    return delegate_.ToConstraintSpace();
  }

 private:
  ConstraintSpaceBuilder delegate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CONSTRAINT_SPACE_BUILDER_H_
