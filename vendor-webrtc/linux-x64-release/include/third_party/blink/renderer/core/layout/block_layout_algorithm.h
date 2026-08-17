// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BLOCK_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BLOCK_LAYOUT_ALGORITHM_H_

#include <optional>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_break_token.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/exclusions/exclusion_space.h"
#include "third_party/blink/renderer/core/layout/floats_utils.h"
#include "third_party/blink/renderer/core/layout/geometry/margin_strut.h"
#include "third_party/blink/renderer/core/layout/inline/inline_child_layout_context.h"
#include "third_party/blink/renderer/core/layout/inline/inline_node.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"
#include "third_party/blink/renderer/core/layout/layout_result.h"
#include "third_party/blink/renderer/core/layout/line_clamp_data.h"
#include "third_party/blink/renderer/core/layout/unpositioned_float.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

class ConstraintSpace;
class LogicalFragment;
enum class BreakStatus;

// This struct is used for communicating to a child the position of the previous
// inflow child. This will be used to calculate the position of the next child.
struct PreviousInflowPosition {
  LayoutUnit logical_block_offset;
  MarginStrut margin_strut;
  // > 0: Block-end annotation space of the previous line
  // < 0: Block-end annotation overflow of the previous line
  LayoutUnit block_end_annotation_space;
  bool self_collapsing_child_had_clearance;
};

// This struct holds information for the current inflow child. The data is not
// useful outside of handling this single inflow child.
struct InflowChildData {
  InflowChildData(BfcOffset bfc_offset_estimate,
                  const MarginStrut& margin_strut,
                  const BoxStrut& margins,
                  bool is_pushed_by_floats = false)
      : bfc_offset_estimate(bfc_offset_estimate),
        margin_strut(margin_strut),
        margins(margins),
        is_pushed_by_floats(is_pushed_by_floats) {}

  InflowChildData(const InflowChildData&) = default;

  BfcOffset bfc_offset_estimate;
  MarginStrut margin_strut;
  BoxStrut margins;
  bool is_pushed_by_floats = false;
};

struct BlockLineClampData {
  DISALLOW_NEW();

  explicit BlockLineClampData(LineClampData line_clamp_data)
      : data(line_clamp_data) {
    if (data.state == LineClampData::kClampByLines) {
      initial_lines_until_clamp = data.lines_until_clamp;
    }
  }

  std::optional<int> LinesUntilClamp(bool show_measured_lines = false) const {
    return data.LinesUntilClamp(show_measured_lines);
  }

  bool IsPastClampPoint() const { return data.IsPastClampPoint(); }

  bool ShouldHideForPaint() const { return data.ShouldHideForPaint(); }

  bool ShouldRelayoutWithNoForcedTruncate() const {
    if (!previous_inflow_position_when_clamped.has_value()) {
      return false;
    }
    DCHECK_EQ(data.state, LineClampData::kClampByLines);
    return data.lines_until_clamp == 0;
  }

  void UpdateClampOffsetFromStyle(LayoutUnit clamp_bfc_offset,
                                  LayoutUnit content_edge) {
    if (ignore_line_clamp) {
      DCHECK_EQ(data.state, LineClampData::kDisabled);
      return;
    }

    if (data.state == LineClampData::kMeasureLinesUntilBfcOffset) {
      // We're doing relayout with a different BFC offset which we obtained from
      // the previous layout. This offset must be less than the one we get from
      // style.
      DCHECK_LT(data.clamp_bfc_offset, clamp_bfc_offset);
      return;
    }

    DCHECK_EQ(data.state, LineClampData::kDisabled);
    if (clamp_bfc_offset == kIndefiniteSize) {
      data.state = LineClampData::kDisabled;
    } else {
      data.state = LineClampData::kMeasureLinesUntilBfcOffset;
      data.lines_until_clamp = 0;
      data.clamp_bfc_offset = clamp_bfc_offset;
    }
  }

  void UpdateLinesFromStyle(int lines_until_clamp) {
    if (ignore_line_clamp) {
      DCHECK_EQ(data.state, LineClampData::kDisabled);
      return;
    }

    DCHECK_EQ(data.state, LineClampData::kDisabled);
    data.state = LineClampData::kClampByLines;
    data.lines_until_clamp = lines_until_clamp;
  }

  // Returns false if we need to relayout with a different clamp BFC offset.
  bool UpdateAfterLayout(const LayoutResult* layout_result,
                         LayoutUnit bfc_block_offset,
                         const PreviousInflowPosition& previous_inflow_position,
                         LayoutUnit block_end_padding) {
    if (data.state == LineClampData::kClampByLines) {
      if (!layout_result->GetPhysicalFragment().IsFormattingContextRoot() &&
          !ignore_further_lines) {
        data.lines_until_clamp = layout_result->LinesUntilClamp();

        if (layout_result->WouldBeLastLineIfNotForEllipsis()) {
          DCHECK(layout_result->GetPhysicalFragment().IsLineBox());
          DCHECK_EQ(data.lines_until_clamp, 0);
          ignore_further_lines = true;
        }
      }

      if (IsPastClampPoint() &&
          !previous_inflow_position_when_clamped.has_value()) {
        previous_inflow_position_when_clamped = previous_inflow_position;
      }
    }

    if (data.state == LineClampData::kMeasureLinesUntilBfcOffset) {
      // We compute the margin strut we'd have after this block if we were to
      // clamp here.
      MarginStrut collapsed_strut = previous_inflow_position.margin_strut;
      collapsed_strut.positive_margin = std::max(
          collapsed_strut.positive_margin, end_margin_strut.positive_margin);
      collapsed_strut.quirky_positive_margin =
          std::max(collapsed_strut.quirky_positive_margin,
                   end_margin_strut.quirky_positive_margin);
      collapsed_strut.negative_margin = std::max(
          collapsed_strut.negative_margin, end_margin_strut.negative_margin);

      // The extra space after the current box that would be added by ruby
      // annotations, considering that the annotations eat into the following
      // padding if it exists, and that we have already subtracted the block end
      // padding from the clamp BFC offset.
      LayoutUnit padding_annotation_overflow;
      if (previous_inflow_position.block_end_annotation_space < LayoutUnit()) {
        padding_annotation_overflow =
            std::max(previous_inflow_position.block_end_annotation_space,
                     -block_end_padding);
      }

      LayoutUnit bfc_offset = bfc_block_offset +
                              previous_inflow_position.logical_block_offset +
                              padding_annotation_overflow +
                              (collapsed_strut.Sum() - end_margin_strut.Sum());

      if (bfc_offset > data.clamp_bfc_offset) {
        return false;
      }

      if (!layout_result->GetPhysicalFragment().IsFormattingContextRoot()) {
        data.lines_until_clamp = layout_result->LinesUntilClamp();
      }
    }

    return true;
  }

  LineClampData data;

  // Set to true when we relayout ignoring line clamp.
  bool ignore_line_clamp = false;

  // TODO(abotella): Make the following fields into a union.

  // The initial number of lines until clamp from the start of this layout.
  // Needed when relayouting due to factors other than line-clamp. Only relevant
  // if data.state == kClampByLines.
  int initial_lines_until_clamp = 0;

  // Only relevant if data.state == kMeasureLinesUntilBfcOffset.
  MarginStrut end_margin_strut;

  // If set, the box was clamped, and this is the previous inflow position after
  // the last line or box before clamp. Can only be set if
  // data.state == kClampByLines.
  std::optional<PreviousInflowPosition> previous_inflow_position_when_clamped;

  // If set, any lines added by any further layout results are ignored when
  // decreasing the number of lines until clamp. Used when we know that the
  // remaining lines in this block box would not exist if we weren't
  // ellipsizing. Can only be set if data.state == kClampByLines.
  bool ignore_further_lines = false;
};

// A class for general block layout (e.g. a <div> with no special style).
// Lays out the children in sequence.
class CORE_EXPORT BlockLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  explicit BlockLayoutAlgorithm(const LayoutAlgorithmParams& params);

  ~BlockLayoutAlgorithm();

  void SetupRelayoutData(const BlockLayoutAlgorithm& previous, RelayoutType);
  void SetBoxType(PhysicalFragment::BoxType type);

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&);
  const LayoutResult* Layout();

 private:
  NOINLINE const LayoutResult* HandleNonsuccessfulLayoutResult(
      const LayoutResult*);

  NOINLINE const LayoutResult* LayoutInlineChild(const InlineNode& child);
  template <wtf_size_t capacity>
  NOINLINE const LayoutResult* LayoutWithOptimalInlineChildLayoutContext(
      const InlineNode& child);

  NOINLINE const LayoutResult* RelayoutIgnoringLineClamp();
  NOINLINE const LayoutResult* RelayoutWithLineClampBlockSize(
      int lines_until_clamp);
  NOINLINE const LayoutResult* RelayoutForTextBoxTrimEnd();

  inline const LayoutResult* Layout(
      InlineChildLayoutContext* inline_child_layout_context);

  const LayoutResult* FinishLayout(PreviousInflowPosition*,
                                   InlineChildLayoutContext*);

  // Return the BFC block offset of this block.
  LayoutUnit BfcBlockOffset() const {
    // If we have resolved our BFC block offset, use that.
    if (container_builder_.BfcBlockOffset())
      return *container_builder_.BfcBlockOffset();
    // Otherwise fall back to the BFC block offset assigned by the parent
    // algorithm.
    return GetConstraintSpace().GetBfcOffset().block_offset;
  }

  // Return the BFC block offset of the next block-start border edge (for some
  // child) we'd get if we commit pending margins.
  LayoutUnit NextBorderEdge(
      const PreviousInflowPosition& previous_inflow_position) const {
    return BfcBlockOffset() + previous_inflow_position.logical_block_offset +
           previous_inflow_position.margin_strut.Sum();
  }

  LogicalSize PercentageSizeForChild(const LayoutInputNode& child) {
    return child.IsReplaced() ? replaced_child_percentage_size_
                              : child_percentage_size_;
  }

  BoxStrut CalculateMargins(LayoutInputNode child,
                            bool is_new_fc,
                            LayoutUnit* additional_line_offset);

  // Creates a new constraint space for the current child.
  ConstraintSpace CreateConstraintSpaceForChild(
      const LayoutInputNode child,
      const BreakToken* child_break_token,
      const InflowChildData& child_data,
      const LogicalSize child_available_size,
      bool is_new_fc,
      const std::optional<LayoutUnit> bfc_block_offset = std::nullopt,
      bool has_clearance_past_adjoining_floats = false,
      LayoutUnit block_start_annotation_space = LayoutUnit());

  // @return Estimated BFC block offset for the "to be layout" child.
  InflowChildData ComputeChildData(const PreviousInflowPosition&,
                                   LayoutInputNode,
                                   const BreakToken* child_break_token,
                                   bool is_new_fc);

  PreviousInflowPosition ComputeInflowPosition(
      const PreviousInflowPosition&,
      const LayoutInputNode child,
      const InflowChildData&,
      const std::optional<LayoutUnit>& child_bfc_block_offset,
      const LogicalOffset&,
      const LayoutResult&,
      const LogicalFragment&,
      bool self_collapsing_child_had_clearance);

  // Position an self-collapsing child using the parent BFC block-offset.
  // The fragment doesn't know its offset, but we can still calculate its BFC
  // position because the parent fragment's BFC is known.
  // Example:
  //   BFC Offset is known here because of the padding.
  //   <div style="padding: 1px">
  //     <div id="zero" style="margin: 1px"></div>
  LayoutUnit PositionSelfCollapsingChildWithParentBfc(
      const LayoutInputNode& child,
      const ConstraintSpace& child_space,
      const InflowChildData& child_data,
      const LayoutResult&) const;

  // Try to reuse part of cached fragments. When reusing is possible, this
  // function adds part of cached fragments to |container_builder_|, update
  // |break_token_| to continue layout from the last reused fragment, and
  // returns |true|. Otherwise returns |false|.
  bool TryReuseFragmentsFromCache(
      InlineNode child,
      PreviousInflowPosition*,
      const InlineBreakToken** inline_break_token_out);

  void HandleOutOfFlowPositioned(const PreviousInflowPosition&, BlockNode);
  void HandleFloat(const PreviousInflowPosition&,
                   BlockNode,
                   const BlockBreakToken*);

  // This uses the LayoutOpporunityIterator to position the fragment.
  //
  // An element that establishes a new formatting context must not overlap the
  // margin box of any floats within the current BFC.
  //
  // Example:
  // <div id="container">
  //   <div id="float"></div>
  //   <div id="new-fc" style="margin-top: 20px;"></div>
  // </div>
  // 1) If #new-fc is small enough to fit the available space right from #float
  //    then it will be placed there and we collapse its margin.
  // 2) If #new-fc is too big then we need to clear its position and place it
  //    below #float ignoring its vertical margin.
  //
  // Returns false if we need to abort layout, because a previously unknown BFC
  // block offset has now been resolved.
  LayoutResult::EStatus HandleNewFormattingContext(
      LayoutInputNode child,
      const BlockBreakToken* child_break_token,
      PreviousInflowPosition*);

  // Performs the actual layout of a new formatting context. This may be called
  // multiple times from HandleNewFormattingContext.
  const LayoutResult* LayoutNewFormattingContext(
      LayoutInputNode child,
      const BlockBreakToken* child_break_token,
      const InflowChildData&,
      BfcOffset origin_offset,
      bool abort_if_cleared,
      BfcOffset* out_child_bfc_offset,
      BoxStrut* out_resolved_margins);

  // Handle an in-flow child.
  // Returns false if we need to abort layout, because a previously unknown BFC
  // block offset has now been resolved. (Same as HandleNewFormattingContext).
  LayoutResult::EStatus HandleInflow(
      LayoutInputNode child,
      const BreakToken* child_break_token,
      PreviousInflowPosition*,
      InlineChildLayoutContext*,
      const InlineBreakToken** previous_inline_break_token);

  LayoutResult::EStatus FinishInflow(
      LayoutInputNode child,
      const BreakToken* child_break_token,
      const ConstraintSpace&,
      bool has_clearance_past_adjoining_floats,
      const LayoutResult*,
      InflowChildData*,
      PreviousInflowPosition*,
      InlineChildLayoutContext*,
      const InlineBreakToken** previous_inline_break_token);

  // Update text box trim state after child layout.
  void UpdateTextBoxTrim(LayoutInputNode child,
                         const BreakToken* incoming_child_break_token,
                         const InlineBreakToken* outgoing_inline_break_token,
                         const LayoutResult*,
                         PreviousInflowPosition*);

  // Consume all remaining fragmentainer space. This happens when we decide to
  // break before a child.
  //
  // https://www.w3.org/TR/css-break-3/#box-splitting
  void ConsumeRemainingFragmentainerSpace(PreviousInflowPosition*);

  // Final adjustments before fragment creation. We need to prevent the fragment
  // from crossing fragmentainer boundaries, and rather create a break token if
  // we're out of space. As part of finalizing we may also discover that we need
  // to abort layout, because we've run out of space at a less-than-ideal
  // location, or that we need to relayout without block fragmentation (when a
  // clipped box gets overflowed past the fragmentation line). The return value
  // can be checked for this. Only if kContinue is returned, can a fragment be
  // created.
  BreakStatus FinalizeForFragmentation();

  // Insert a fragmentainer break before the child if necessary.
  // See |::blink::BreakBeforeChildIfNeeded()| for more documentation.
  BreakStatus BreakBeforeChildIfNeeded(LayoutInputNode child,
                                       const LayoutResult&,
                                       PreviousInflowPosition*,
                                       LayoutUnit bfc_block_offset,
                                       bool has_container_separation);

  // Look for a better breakpoint (than we already have) between lines (i.e. a
  // class B breakpoint), and store it.
  void UpdateEarlyBreakBetweenLines();

  // Propagates the baseline from the given |child| if needed.
  void PropagateBaselineFromLineBox(const PhysicalFragment& child,
                                    LayoutUnit block_offset);
  void PropagateBaselineFromBlockChild(const PhysicalFragment& child,
                                       const BoxStrut& margins,
                                       LayoutUnit block_offset);

  // If still unresolved, resolve the fragment's BFC block offset.
  //
  // This includes applying clearance, so the |bfc_block_offset| passed won't
  // be the final BFC block-offset, if it wasn't large enough to get past all
  // relevant floats. The updated BFC block-offset can be read out with
  // |ContainerBfcBlockOffset()|.
  //
  // If the |forced_bfc_block_offset| has a value, it will override the given
  // |bfc_block_offset|. Typically this comes from the input constraints, when
  // the current node has clearance past adjoining floats, or has a re-layout
  // due to a child resolving the BFC block-offset.
  //
  // In addition to resolving our BFC block offset, this will also position
  // pending floats, and update our in-flow layout state.
  //
  // Returns false if resolving the BFC block-offset resulted in needing to
  // abort layout. It will always return true otherwise. If the BFC
  // block-offset was already resolved, this method does nothing (and returns
  // true).
  bool ResolveBfcBlockOffset(
      PreviousInflowPosition*,
      LayoutUnit bfc_block_offset,
      const std::optional<LayoutUnit> forced_bfc_block_offset);

  // This passes in the |forced_bfc_block_offset| from the input constraints,
  // which is almost always desired.
  bool ResolveBfcBlockOffset(PreviousInflowPosition* previous_inflow_position,
                             LayoutUnit bfc_block_offset) {
    return ResolveBfcBlockOffset(previous_inflow_position, bfc_block_offset,
                                 GetConstraintSpace().ForcedBfcBlockOffset());
  }

  // A very common way to resolve the BFC block offset is to simply commit the
  // pending margin, so here's a convenience overload for that.
  bool ResolveBfcBlockOffset(PreviousInflowPosition* previous_inflow_position) {
    return ResolveBfcBlockOffset(previous_inflow_position,
                                 NextBorderEdge(*previous_inflow_position));
  }

  // Mark this fragment as modifying its incoming margin-strut if it hasn't
  // resolved its BFC block-offset yet.
  void SetSubtreeModifiedMarginStrutIfNeeded(const Length* margin = nullptr) {
    if (container_builder_.BfcBlockOffset())
      return;

    if (margin && margin->IsZero())
      return;

    container_builder_.SetSubtreeModifiedMarginStrut();
  }

  // Return true if the BFC block offset has changed and this means that we
  // need to abort layout.
  bool NeedsAbortOnBfcBlockOffsetChange() const;

  // Positions a list marker for the specified block content.
  // Return false if it aborts when resolving BFC block offset for LI.
  bool PositionOrPropagateListMarker(const LayoutResult&,
                                     LogicalOffset*,
                                     PreviousInflowPosition*);

  // Positions a list marker when the block does not have any line boxes.
  // Return false if it aborts when resolving BFC block offset for LI.
  bool PositionListMarkerWithoutLineBoxes(PreviousInflowPosition*);

  // Calculates logical offset for the current fragment using either {@code
  // intrinsic_block_size_} when the fragment doesn't know it's offset or
  // {@code known_fragment_offset} if the fragment knows it's offset
  // @return Fragment's offset relative to the fragment's parent.
  LogicalOffset CalculateLogicalOffset(
      const LogicalFragment& fragment,
      LayoutUnit child_bfc_line_offset,
      const std::optional<LayoutUnit>& child_bfc_block_offset);

  // In quirks mode the body element will stretch to fit the viewport.
  //
  // In order to determine the final block-size we need to take the available
  // block-size minus the total block-direction margin.
  //
  // This block-direction margin is non-trivial to calculate for the body
  // element, and is computed upfront for the |ClampIntrinsicBlockSize|
  // function.
  std::optional<LayoutUnit> CalculateQuirkyBodyMarginBlockSum(
      const MarginStrut& end_margin_strut);

  // Return true if this is a list-item that may have to place a marker.
  bool ShouldPlaceUnpositionedListMarker() const {
    if (!node_.IsListItem())
      return false;
    // Also need to check if the constraint space is anonymous, which is the
    // case for columns (the list item marker should be placed by the multicol
    // container then, not the individual columns).
    if (!GetConstraintSpace().IsAnonymous()) {
      return true;
    }
    // Ensure we're really a column box. We can't use |BoxType| to call this
    // from the constructor.
    DCHECK(node_.GetLayoutBox()->SlowFirstChild()->IsLayoutFlowThread());
    return false;
  }

  // Layout |placeholder| content, and decide the location of |placeholder|.
  // This is called only if |this| is a text control.
  // This function returns a new value for `PreviousInflowPosition::
  // logical_block_offset`.
  LayoutUnit HandleTextControlPlaceholder(
      BlockNode placeholder,
      const PreviousInflowPosition& previous_inflow_position);
  // A helper for HandleTextControlPlaceholder().
  // This function returns a new value for `PreviousInflowPosition::
  // logical_block_offset`.
  LayoutUnit FinishTextControlPlaceholder(
      const LayoutResult* result,
      const LogicalOffset& offset,
      bool apply_fixed_size,
      const PreviousInflowPosition& previous_inflow_position);

  // Adjusts the inline offset of the slider thumb box from the value of
  // HTMLInputElement.
  LogicalOffset AdjustSliderThumbInlineOffset(
      const LogicalFragment& fragment,
      const LogicalOffset& logical_offset);

  LogicalSize child_percentage_size_;
  LogicalSize replaced_child_percentage_size_;

  const LayoutResult* previous_result_ = nullptr;

  const ColumnSpannerPath* column_spanner_path_ = nullptr;

  // The last non-empty inflow child. Currently this is used only when
  // `should_text_box_trim_end_` and when the last child was empty. Thus this is
  // updated only in that case.
  InlineNode last_non_empty_inflow_child_ = nullptr;
  // The break token of the last non-empty line.
  const BreakToken* last_non_empty_break_token_ = nullptr;

  // `text-box-trim: end` should be applied to this child.
  InlineNode override_text_box_trim_end_child_ = nullptr;
  const BreakToken* override_text_box_trim_end_break_token_ = nullptr;

  // Intrinsic block size based on child layout and containment.
  LayoutUnit intrinsic_block_size_;

  BlockLineClampData line_clamp_data_;

  // The line box index at which we ran out of space. This where we'll actually
  // end up breaking, unless we determine that we should break earlier in order
  // to satisfy the widows request.
  int first_overflowing_line_ = 0;

  // Set if we should fit as many lines as there's room for, i.e. no early
  // break. In that case we'll break before first_overflowing_line_. In this
  // case there'll either be enough widows for the next fragment, or we have
  // determined that we're unable to fulfill the widows request.
  bool fit_all_lines_ : 1;

  // Set if we're resuming layout of a node that has already produced fragments.
  bool is_resuming_ : 1;

  // Set when we're to abort if the BFC block offset gets resolved or updated.
  // Sometimes we walk past elements (i.e. floats) that depend on the BFC block
  // offset being known (in order to position and lay themselves out properly).
  // When this happens, and we finally manage to resolve (or update) the BFC
  // block offset at some subsequent element, we need to check if this flag is
  // set, and abort layout if it is.
  bool abort_when_bfc_block_offset_updated_ : 1;

  // This will be set during block fragmentation, normally once we've processed
  // the first in-flow child of a container (but there are some exceptions to
  // this). It is used to check if we're at a valid class A or B breakpoint
  // (between block-level siblings or line box siblings).
  bool has_break_opportunity_before_next_child_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BLOCK_LAYOUT_ALGORITHM_H_
