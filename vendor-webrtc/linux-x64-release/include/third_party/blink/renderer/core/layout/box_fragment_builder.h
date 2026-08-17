// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BOX_FRAGMENT_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BOX_FRAGMENT_BUILDER_H_

#include <optional>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_break_token.h"
#include "third_party/blink/renderer/core/layout/break_token.h"
#include "third_party/blink/renderer/core/layout/constraint_space.h"
#include "third_party/blink/renderer/core/layout/flex/devtools_flex_info.h"
#include "third_party/blink/renderer/core/layout/fragment_builder.h"
#include "third_party/blink/renderer/core/layout/frame_set_layout_data.h"
#include "third_party/blink/renderer/core/layout/gap_fragment_data.h"
#include "third_party/blink/renderer/core/layout/geometry/box_sides.h"
#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/core/layout/geometry/fragment_geometry.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/inline/fragment_items_builder.h"
#include "third_party/blink/renderer/core/layout/layout_result.h"
#include "third_party/blink/renderer/core/layout/length_utils.h"
#include "third_party/blink/renderer/core/layout/mathml/mathml_paint_info.h"
#include "third_party/blink/renderer/core/layout/scrollable_overflow_calculator.h"
#include "third_party/blink/renderer/core/layout/table/table_borders.h"
#include "third_party/blink/renderer/core/layout/table/table_fragment_data.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class PhysicalFragment;

class CORE_EXPORT BoxFragmentBuilder final : public FragmentBuilder {
  STACK_ALLOCATED();

 public:
  BoxFragmentBuilder(LayoutInputNode node,
                     const ComputedStyle* style,
                     const ConstraintSpace& space,
                     WritingDirectionMode writing_direction,
                     const BlockBreakToken* previous_break_token)
      : FragmentBuilder(node,
                        style,
                        space,
                        writing_direction,
                        previous_break_token),
        is_inline_formatting_context_(node.IsInline()) {}

  // Build a fragment for LayoutObject without LayoutInputNode. LayoutInline
  // has InlineItem but does not have corresponding LayoutInputNode.
  BoxFragmentBuilder(LayoutObject* layout_object,
                     const ComputedStyle* style,
                     const ConstraintSpace& space,
                     WritingDirectionMode writing_direction)
      : FragmentBuilder(/*node=*/nullptr,
                        std::move(style),
                        space,
                        writing_direction,
                        /*previous_break_token=*/nullptr),
        is_inline_formatting_context_(true) {
    layout_object_ = layout_object;
  }

  void SetInitialFragmentGeometry(
      const FragmentGeometry& initial_fragment_geometry) {
    initial_fragment_geometry_ = &initial_fragment_geometry;
    size_ = initial_fragment_geometry_->border_box_size;
    is_initial_block_size_indefinite_ = size_.block_size == kIndefiniteSize;

    border_padding_ =
        initial_fragment_geometry.border + initial_fragment_geometry.padding;

    // Box decorations don't take up layout space in table rows / sections.
    if (!node_ || (!node_.IsTableSection() && !node_.IsTableRow())) {
      border_scrollbar_padding_ = initial_fragment_geometry.border +
                                  initial_fragment_geometry.scrollbar;
      // Padding doesn't take up layout space in fieldset containers (that's
      // done inside the anonymous child wrapper).
      if (!node_ || !node_.IsFieldsetContainer()) {
        border_scrollbar_padding_ += initial_fragment_geometry.padding;
      }
    }
    original_border_scrollbar_padding_block_start_ =
        border_scrollbar_padding_.block_start;
    if (node_) {
      child_available_size_ = CalculateChildAvailableSize(
          space_, To<BlockNode>(node_), size_,
          border_padding_ + initial_fragment_geometry.scrollbar);
    }
  }

  const FragmentGeometry& InitialFragmentGeometry() const {
    DCHECK(initial_fragment_geometry_);
    return *initial_fragment_geometry_;
  }

  // Set up text box trimming state, based on the constraint space and computed
  // style. To be called by all algorithms that implement text box trimming.
  void SetInitialTextBoxTrim();

  bool ShouldTextBoxTrimStart() const {
    return should_text_box_trim_node_start_ ||
           should_text_box_trim_fragmentainer_start_;
  }

  bool ShouldTextBoxTrimEnd() const {
    return should_text_box_trim_node_end_ ||
           should_text_box_trim_fragmentainer_end_;
  }

  bool ShouldTextBoxTrim() const {
    return ShouldTextBoxTrimStart() || ShouldTextBoxTrimEnd();
  }

  void ClearShouldTextBoxTrimEnd() {
    should_text_box_trim_node_end_ = false;
    should_text_box_trim_fragmentainer_end_ = false;
  }

  void ClearShouldTextBoxTrimNodeStart() {
    should_text_box_trim_node_start_ = false;
  }
  bool ShouldTextBoxTrimNodeStart() const {
    return should_text_box_trim_node_start_;
  }
  void SetShouldTextBoxTrimNodeEnd(bool b) {
    should_text_box_trim_node_end_ = b;
  }
  bool ShouldTextBoxTrimNodeEnd() const {
    return should_text_box_trim_node_end_;
  }

  void ClearShouldTextBoxTrimFragmentainerStart() {
    should_text_box_trim_fragmentainer_start_ = false;
  }
  bool ShouldTextBoxTrimFragmentainerStart() const {
    return should_text_box_trim_fragmentainer_start_;
  }
  bool ShouldTextBoxTrimFragmentainerEnd() const {
    return should_text_box_trim_fragmentainer_end_;
  }

  const BlockBreakToken* PreviousBreakToken() const {
    return To<BlockBreakToken>(previous_break_token_);
  }

  // Use the block-size setters/getters further down instead of the inherited
  // ones.
  LayoutUnit BlockSize() const = delete;
  void SetBlockSize(LayoutUnit block_size) = delete;

  // Set the total border-box block-size of all the fragments to be generated
  // from this node (as if we stitched them together). Layout algorithms are
  // expected to pass this value, and at the end of layout (if block
  // fragmentation is needed), the fragmentation machinery will be invoked to
  // adjust the block-size to the correct size, ensuring that we break at the
  // best location.
  void SetFragmentsTotalBlockSize(LayoutUnit block_size) {
#if DCHECK_IS_ON()
    // Note that we just store the block-size in a shared field. We have a flag
    // for debugging, to assert that we know what we're doing when attempting to
    // access the data.
    block_size_is_for_all_fragments_ = true;
#endif
    size_.block_size = block_size;
  }
  LayoutUnit FragmentsTotalBlockSize() const {
#if DCHECK_IS_ON()
    if (has_block_fragmentation_)
      DCHECK(block_size_is_for_all_fragments_);
    DCHECK(size_.block_size != kIndefiniteSize);
#endif
    return size_.block_size;
  }

  // Set the final block-size of this fragment.
  void SetFragmentBlockSize(LayoutUnit block_size) {
#if DCHECK_IS_ON()
    // Note that we just store the block-size in a shared field. We have a flag
    // for debugging, to assert that we know what we're doing when attempting to
    // access the data.
    block_size_is_for_all_fragments_ = false;
#endif
    size_.block_size = block_size;
  }

  LayoutUnit FragmentBlockSize() const {
#if DCHECK_IS_ON()
    DCHECK(!block_size_is_for_all_fragments_ || !has_block_fragmentation_ ||
           IsInitialColumnBalancingPass());
    DCHECK(size_.block_size != kIndefiniteSize);
#endif
    return size_.block_size;
  }

  LogicalSize SizeForAnchorQueries() const {
    // TODO(layout-dev): This isn't great. But sometimes anchor queries are
    // evaluated in the middle of layout of an OOF container. This happens when
    // the OOF container is a multicol container, and column layout gets
    // interrupted by a column spanner. We should probably provide the multicol
    // block size we have at the point of being interrupted by the spanner,
    // rather than using 0.
    LogicalSize logical_size(InlineSize(), LayoutUnit());
    if (HasBlockSize()) {
      logical_size.block_size = FragmentBlockSize();
    }
    return logical_size;
  }

  void SetIntrinsicBlockSize(LayoutUnit intrinsic_block_size) {
    intrinsic_block_size_ = intrinsic_block_size;
  }
  LayoutUnit IntrinsicBlockSize() const { return intrinsic_block_size_; }
  const BoxStrut& Borders() const {
    DCHECK(initial_fragment_geometry_);
    DCHECK_NE(GetBoxType(), PhysicalFragment::kInlineBox);
    return initial_fragment_geometry_->border;
  }
  const BoxStrut& Scrollbar() const {
    DCHECK(initial_fragment_geometry_);
    return initial_fragment_geometry_->scrollbar;
  }
  const BoxStrut& Padding() const {
    DCHECK(initial_fragment_geometry_);
    return initial_fragment_geometry_->padding;
  }
  const LogicalSize& InitialBorderBoxSize() const {
    DCHECK(initial_fragment_geometry_);
    return initial_fragment_geometry_->border_box_size;
  }

  BoxStrut ExcludedSidesTruncated(const BoxStrut& strut) const {
    // Note that this only truncates along the block axis for now. When it comes
    // to the inline axis, BoxStrut has inline_start/inline_end, whereas
    // LineLogicalBoxSides has line_left/line_right, so it's a bit more work.
    //
    // TODO(layout-dev): It's rather straight-forward to fix the above now, if
    // we want to, since we have a "well-behaving" LogicalBoxSides struct.
    return BoxStrut(
        strut.inline_start, strut.inline_end,
        sides_to_include_.block_start ? strut.block_start : LayoutUnit(),
        sides_to_include_.block_end ? strut.block_end : LayoutUnit());
  }

  BoxStrut ApplicableBorders() const {
    DCHECK(initial_fragment_geometry_);
    return ExcludedSidesTruncated(initial_fragment_geometry_->border);
  }
  BoxStrut ApplicableScrollbar() const {
    DCHECK(initial_fragment_geometry_);
    return ExcludedSidesTruncated(initial_fragment_geometry_->scrollbar);
  }
  BoxStrut ApplicablePadding() const {
    DCHECK(initial_fragment_geometry_);
    return ExcludedSidesTruncated(initial_fragment_geometry_->padding);
  }

  // Get border+padding for each box side.
  //
  // This value is node-specific (not for an individual fragment), and is used
  // to resolve the final box size, but is not used to position descendants.
  // This distinction matters for block fragmentation. Resolving the final box
  // size means the "stitched" box size (sum of the block-size of all
  // fragments). If box decorations are to be cloned, it must be reflected in
  // this value, meaning that computed border+padding is multiplied by the
  // number of fragments (so that e.g. a <div style="padding:20px;
  // height:100px;"> split into two fragments get a stitched border-box size of
  // 180px).
  const BoxStrut& BorderPadding() const {
    DCHECK(initial_fragment_geometry_);
    return border_padding_;
  }

  // Get border+padding+scrollbar for each box side.
  //
  // This value is fragment-specific, and is used to position descendants and to
  // calculate the intrinsic block-size, but not to resolve the final box
  // size. This distinction matters for block fragmentation. When box
  // decorations are to be sliced (i.e. not cloned), the block-start
  // border+padding size is truncated to 0 after fragmentation breaks, and this
  // will be reflected here, so that we don't make room for block-start
  // border+padding at the beginning of each fragment (only the first).
  const BoxStrut& BorderScrollbarPadding() const {
    DCHECK(initial_fragment_geometry_);
    return border_scrollbar_padding_;
  }

  LayoutUnit OriginalBorderScrollbarPaddingBlockStart() const {
    return original_border_scrollbar_padding_block_start_;
  }

  void ClearBorderScrollbarPaddingBlockStart() {
    border_scrollbar_padding_.block_start = LayoutUnit();
  }
  void ClearBorderScrollbarPaddingBlockEnd() {
    border_scrollbar_padding_.block_end = LayoutUnit();
  }
  void UpdateBorderPaddingForClonedBoxDecorations();

  // The child available-size is subtly different from the content-box size of
  // an element. For an anonymous-block the child available-size is equal to
  // its non-anonymous parent (similar to percentages).
  const LogicalSize& ChildAvailableSize() const {
    DCHECK(initial_fragment_geometry_);
    return child_available_size_;
  }
  const BlockNode& Node() const {
    DCHECK(node_);
    return To<BlockNode>(node_);
  }

  // Be sure to use the layout result that's relevant for propagation and block
  // fragmentation considerations. This will normally just be the layout result
  // that's passed to the function, but if this is a line box with a block
  // inside (aka. block-in-inline), it will return the layout result for the
  // block instead.
  const LayoutResult& LayoutResultForPropagation(const LayoutResult&) const;

  // Add a break token for a child that doesn't yet have any fragments, because
  // its first fragment is to be produced in the next fragmentainer. This will
  // add a break token for the child, but no fragment. Break appeal should
  // always be provided for regular in-flow children. For other types of
  // children it may be omitted, if the break shouldn't affect the appeal of
  // breaking inside this container.
  void AddBreakBeforeChild(LayoutInputNode child,
                           std::optional<BreakAppeal> appeal,
                           bool is_forced_break);

  // Add a layout result and propagate info from it. This involves appending the
  // fragment and its relative offset to the builder, but also keeping track of
  // out-of-flow positioned descendants, propagating fragmentainer breaks, and
  // more. In some cases, such as grid, the relative offset may need to be
  // computed ahead of time. If so, a |relative_offset| will be passed
  // in. Otherwise, the relative offset will be calculated as normal.
  // |inline_container| is passed when adding an OOF that is contained by a
  // non-atomic inline.
  void AddResult(
      const LayoutResult&,
      const LogicalOffset,
      std::optional<const BoxStrut> margins,
      std::optional<LogicalOffset> relative_offset = std::nullopt,
      const OofInlineContainer<LogicalOffset>* inline_container = nullptr);
  // AddResult() with the default margin computation.
  void AddResult(const LayoutResult& child_layout_result,
                 const LogicalOffset offset);

  // Add a child fragment and propagate info from it. Called by AddResult().
  // Other callers should call AddResult() instead of this when possible, since
  // there is information in the layout result that might need to be propagated.
  void AddChild(
      const PhysicalFragment&,
      const LogicalOffset&,
      const MarginStrut* margin_strut = nullptr,
      bool is_self_collapsing = false,
      std::optional<LogicalOffset> relative_offset = std::nullopt,
      const OofInlineContainer<LogicalOffset>* inline_container = nullptr);

  // Manually add a break token to the builder. Note that we're assuming that
  // this break token is for content in the same flow as this parent.
  void AddBreakToken(const BreakToken*, bool is_in_parallel_flow = false);

  // Before layout we'll determine whether we can tell for sure that the node
  // (or what's left of it to lay out, in case we've already broken) will fit in
  // the current fragmentainer. If this is the case, we'll know that any
  // block-end padding and border will come at the end of this fragment, and, if
  // it's the first fragment for the node, this will make us ensure some child
  // content before allowing breaks. See MustStayInCurrentFragmentainer().
  void SetIsKnownToFitInFragmentainer(bool b) {
    is_known_to_fit_in_fragmentainer_ = b;
  }
  bool IsKnownToFitInFragmentainer() const {
    return is_known_to_fit_in_fragmentainer_;
  }

  void SetIsBlockSizeForFragmentationClamped() {
    is_block_size_for_fragmentation_clamped_ = true;
  }

  // If a node fits in one fragmentainer due to restricted block-size, it must
  // stay there, even if the first piece of child content should require more
  // space than that (which would normally push the entire node into the next
  // fragmentainer, since there typically is no valid breakpoint before the
  // first child - only *between* siblings). Furthermore, any first piece of
  // child content also needs to stay in the current fragmentainer, even if this
  // causes fragmentainer overflow. This is not mandated by any spec, but it is
  // compatible with Gecko, and is required in order to print Google Docs.
  //
  // See https://github.com/w3c/csswg-drafts/issues/6056#issuecomment-951767882
  bool MustStayInCurrentFragmentainer() const {
    return is_known_to_fit_in_fragmentainer_ && is_first_for_node_;
  }

  // Specify whether this will be the first fragment generated for the node.
  void SetIsFirstForNode(bool is_first) { is_first_for_node_ = is_first; }

  bool ShouldCloneBoxEndDecorations() const {
    return should_clone_box_end_decorations_;
  }
  void SetShouldCloneBoxEndDecorations(bool b) {
    should_clone_box_end_decorations_ = b;
  }

  void SetShouldPreventBreakBeforeBlockEndDecorations(bool b) {
    should_prevent_break_before_block_end_decorations_ = b;
  }
  bool ShouldPreventBreakBeforeBlockEndDecorations() const {
    return should_prevent_break_before_block_end_decorations_;
  }

  void SetIsMonolithic(bool b) { is_monolithic_ = b; }

  // Set how much of the block-size we've used so far for this box. This will be
  // the sum of the block-size of all previous fragments PLUS the one we're
  // building now.
  void SetConsumedBlockSize(LayoutUnit size) {
    EnsureBreakTokenData()->consumed_block_size = size;
  }

  // Set how much to adjust |consumed_block_size_| for legacy write-back. See
  // BlockBreakToken::ConsumedBlockSizeForLegacy() for more details.
  void SetConsumedBlockSizeLegacyAdjustment(LayoutUnit adjustment) {
    EnsureBreakTokenData()->consumed_block_size_legacy_adjustment = adjustment;
  }

  void ReserveSpaceForMonolithicOverflow(LayoutUnit monolithic_overflow) {
    DCHECK(GetConstraintSpace().IsPaginated());
    auto* data = EnsureBreakTokenData();
    data->monolithic_overflow =
        std::max(data->monolithic_overflow, monolithic_overflow);
  }

  // Set how much of the column block-size we've used so far. This will be used
  // to determine the block-size of any new columns added by descendant
  // out-of-flow positioned elements.
  void SetBlockOffsetForAdditionalColumns(LayoutUnit size) {
    block_offset_for_additional_columns_ = size;
  }
  LayoutUnit BlockOffsetForAdditionalColumns() const {
    return block_offset_for_additional_columns_;
  }

  void SetSequenceNumber(unsigned sequence_number) {
    EnsureBreakTokenData()->sequence_number = sequence_number;
  }

  // During regular layout a break token is created at the end of layout, if
  // required. When re-using a previous fragment and its children, though, we
  // may want to just re-use the break token as well.
  void PresetNextBreakToken(const BreakToken* break_token) {
    // We should either do block fragmentation as part of normal layout, or
    // pre-set a break token.
    DCHECK(!did_break_self_);
    DCHECK(child_break_tokens_.empty());

    break_token_ = break_token;
  }

  // Return true if we broke inside this node on our own initiative (typically
  // not because of a child break, but rather due to the size of this node).
  bool DidBreakSelf() const { return did_break_self_; }
  void SetDidBreakSelf() { did_break_self_ = true; }

  // Return true if a break has been inserted, doesn't matter if it's in the
  // same flow or not. As long as there are only breaks in parallel flows, we
  // may continue layout, but when we're done, we'll need to create a break
  // token for this fragment nevertheless, so that we re-enter, descend and
  // resume at the broken children in the next fragmentainer.
  bool HasInsertedChildBreak() const {
    if (child_break_tokens_.empty())
      return false;
    for (auto& child_token : child_break_tokens_) {
      const auto* block_child_token =
          DynamicTo<BlockBreakToken>(child_token.Get());
      if (!block_child_token || !block_child_token->IsRepeated())
        return true;
    }
    return false;
  }

  // Return true if we need to break inside this node, the way things are
  // currently looking. This should only be called at the end of layout, right
  // before creating a fragment.
  bool ShouldBreakInside() const {
    if (HasInsertedChildBreak())
      return true;
    // If there's an outgoing inline break-token at this point, and we're about
    // to finish layout, it means that inline layout needs to continue in the
    // next fragmentianer.
    if (last_inline_break_token_)
      return true;

    // If overflowed by monolithic overflow, we need more pages.
    if (break_token_data_ && break_token_data_->monolithic_overflow) {
      return true;
    }

    // Grid layout doesn't insert break before tokens, and instead set this bit
    // to indicate there is content after the current break. Out-of-flow layout
    // of fragmentainers also doesn't insert break-before tokens for OOFs that
    // are to start in a later fragmentainer. But we still want the
    // fragmentainer to create a break token, since there's going to be more.
    return has_subsequent_children_;
  }

  // Return true if we need to break before or inside any in-flow child that
  // doesn't establish a parallel flow. When this happens, we want to finish our
  // fragment, create a break token, and resume in the next fragmentainer.
  bool HasInflowChildBreakInside() const {
    return has_inflow_child_break_inside_;
  }

  void SetInitialBreakBefore(EBreakBetween break_before) {
    initial_break_before_ = break_before;
  }

  void SetInitialBreakBeforeIfNeeded(EBreakBetween break_before) {
    if (!initial_break_before_)
      initial_break_before_ = break_before;
  }

  EBreakBetween PreviousBreakAfter() const { return previous_break_after_; }
  void SetPreviousBreakAfter(EBreakBetween break_after) {
    previous_break_after_ = break_after;
  }

  void SetPageNameIfNeeded(AtomicString name) {
    if (page_name_ == g_null_atom) {
      page_name_ = name;
    }
  }
  const AtomicString& PageName() const { return page_name_; }

  // Join/"collapse" the previous (stored) break-after value with the next
  // break-before value, to determine how to deal with breaking between two
  // in-flow siblings.
  EBreakBetween JoinedBreakBetweenValue(EBreakBetween break_before) const;

  // Return the number of line boxes laid out.
  int LineCount() const { return line_count_; }
  void SetLineCount(int line_count) { line_count_ = line_count; }

  // Set when we have iterated over all the children. This means that all
  // children have been fully laid out, or have break tokens. No more children
  // left to discover.
  void SetHasSeenAllChildren() { has_seen_all_children_ = true; }
  bool HasSeenAllChildren() { return has_seen_all_children_; }

  void SetHasSubsequentChildren() { has_subsequent_children_ = true; }

  void SetIsAtBlockEnd() { is_at_block_end_ = true; }
  bool IsAtBlockEnd() const { return is_at_block_end_; }

  void SetIsTruncatedByFragmentationLine() {
    is_truncated_by_fragmentation_line = true;
  }

  // See |PhysicalBoxFragment::InflowBounds|.
  void SetInflowBounds(const LogicalRect& inflow_bounds) {
    DCHECK_NE(box_type_, PhysicalFragment::BoxType::kInlineBox);
    DCHECK(Node().IsScrollContainer());
#if DCHECK_IS_ON()
    is_inflow_bounds_explicitly_set_ = true;
#endif
    inflow_bounds_ = inflow_bounds;
  }

  void SetEarlyBreak(const EarlyBreak* breakpoint) {
    early_break_ = breakpoint;
  }
  bool HasEarlyBreak() const { return early_break_; }
  const EarlyBreak& GetEarlyBreak() const {
    DCHECK(early_break_);
    return *early_break_;
  }

  // Creates the fragment. Can only be called once.
  const LayoutResult* ToBoxFragment() {
    DCHECK_NE(GetBoxType(), PhysicalFragment::kInlineBox);
    return ToBoxFragment(GetWritingMode());
  }
  const LayoutResult* ToInlineBoxFragment() {
    // The logical coordinate for inline box uses line-relative writing-mode,
    // not
    // flow-relative.
    DCHECK_EQ(GetBoxType(), PhysicalFragment::kInlineBox);
    return ToBoxFragment(ToLineWritingMode(GetWritingMode()));
  }

  void SetIsFieldsetContainer() { is_fieldset_container_ = true; }
  void SetIsTablePart() { is_table_part_ = true; }

  void SetIsInlineFormattingContext(bool is_inline_formatting_context) {
    is_inline_formatting_context_ = is_inline_formatting_context;
  }

  void SetIsMathMLFraction() { is_math_fraction_ = true; }
  void SetIsMathMLOperator() { is_math_operator_ = true; }
  void SetMathMLPaintInfo(const MathMLPaintInfo* mathml_paint_info) {
    mathml_paint_info_ = mathml_paint_info;
  }

  void SetSidesToInclude(LineLogicalBoxSides sides_to_include) {
    sides_to_include_ = sides_to_include;
  }
  void SetSidesToInclude(LogicalBoxSides sides_to_include) {
    sides_to_include_ = LineLogicalBoxSides(sides_to_include, Direction());
  }

  void SetCustomLayoutData(
      scoped_refptr<SerializedScriptValue> custom_layout_data) {
    custom_layout_data_ = std::move(custom_layout_data);
  }

  // Sets the first baseline for this fragment.
  void SetFirstBaseline(LayoutUnit baseline) { first_baseline_ = baseline; }
  std::optional<LayoutUnit> FirstBaseline() const { return first_baseline_; }

  // Sets the last baseline for this fragment.
  void SetLastBaseline(LayoutUnit baseline) { last_baseline_ = baseline; }
  std::optional<LayoutUnit> LastBaseline() const { return last_baseline_; }

  // Sets both the first and last baseline to the same value.
  void SetBaselines(LayoutUnit baseline) {
    first_baseline_ = baseline;
    last_baseline_ = baseline;
  }
  void ClearBaselines() {
    first_baseline_ = std::nullopt;
    last_baseline_ = std::nullopt;
  }

  // Lets the parent layout algorithm know if it should use the first or last
  // baseline for the special inline-block baseline algorithm.
  void SetUseLastBaselineForInlineBaseline() {
    use_last_baseline_for_inline_baseline_ = true;
  }

  void SetGapGeometry(const GapGeometry* gap_geometry) {
    gap_geometry_ = gap_geometry;
  }

  const GapGeometry* GetGapGeometryForTest() { return gap_geometry_; }

  void SetTableGridRect(const LogicalRect& table_grid_rect) {
    table_grid_rect_ = table_grid_rect;
  }

  void SetTableColumnGeometries(
      const TableColumnGeometries& table_column_geometries) {
    table_column_geometries_ = table_column_geometries;
  }

  void SetTableCollapsedBorders(const TableBorders& table_collapsed_borders) {
    table_collapsed_borders_ = &table_collapsed_borders;
  }

  void SetTableCollapsedBordersGeometry(
      std::unique_ptr<CollapsedTableBordersGeometry>
          table_collapsed_borders_geometry) {
    table_collapsed_borders_geometry_ =
        std::move(table_collapsed_borders_geometry);
  }

  void SetTableColumnCount(wtf_size_t table_column_count) {
    table_column_count_ = table_column_count;
  }

  void SetTableCellColumnIndex(wtf_size_t table_cell_column_index) {
    table_cell_column_index_ = table_cell_column_index;
  }

  void SetTableSectionCollapsedBordersGeometry(
      wtf_size_t start_row_index,
      Vector<LayoutUnit>&& row_offsets) {
    table_section_start_row_index_ = start_row_index;
    table_section_row_offsets_ = std::move(row_offsets);
  }

  void TransferGridLayoutData(
      std::unique_ptr<GridLayoutData> grid_layout_data) {
    grid_layout_data_ = std::move(grid_layout_data);
  }
  void TransferFlexLayoutData(
      std::unique_ptr<DevtoolsFlexInfo> flex_layout_data) {
    flex_layout_data_ = std::move(flex_layout_data);
  }
  void TransferFrameSetLayoutData(std::unique_ptr<FrameSetLayoutData> data) {
    frame_set_layout_data_ = std::move(data);
  }
  void SetReadingFlowNodes(HeapVector<Member<blink::Node>>&& nodes) {
    reading_flow_nodes_ = std::move(nodes);
  }

  const GridLayoutData& GetGridLayoutData() const {
    DCHECK(grid_layout_data_);
    return *grid_layout_data_.get();
  }

  bool HasBreakTokenData() const { return break_token_data_; }

  BlockBreakTokenData* EnsureBreakTokenData() {
    if (!HasBreakTokenData())
      break_token_data_ = MakeGarbageCollected<BlockBreakTokenData>();
    return break_token_data_;
  }

  BlockBreakTokenData* GetBreakTokenData() { return break_token_data_; }

  void SetBreakTokenData(BlockBreakTokenData* break_token_data) {
    break_token_data_ = break_token_data;
  }

#if DCHECK_IS_ON()
  // If we don't participate in a fragmentation context, this method can check
  // that all block fragmentation related fields have their initial value.
  void CheckNoBlockFragmentation() const;
#endif

  // Moves all the children by |offset| in the block-direction. (Ensure that
  // any baselines, OOFs, etc, are also moved by the appropriate amount).
  void MoveChildrenInBlockDirection(LayoutUnit offset);

  void SetMathItalicCorrection(LayoutUnit italic_correction) {
    math_italic_correction_ = italic_correction;
  }

  void AdjustFragmentainerDescendant(
      LogicalOofNodeForFragmentation& descendant,
      bool only_fixedpos_containing_block = false);
  void AdjustFixedposContainingBlockForFragmentainerDescendants();
  void AdjustFixedposContainingBlockForInnerMulticols();

  void SetHasForcedBreak() {
    has_forced_break_ = true;
    minimal_space_shortage_ = kIndefiniteSize;
  }

  bool HasForcedBreak() const { return has_forced_break_; }

  const BreakToken* LastChildBreakToken() const {
    DCHECK(!child_break_tokens_.empty());
    return child_break_tokens_.back().Get();
  }

  // Propagate the break-before/break-after of the child (if applicable).
  void PropagateChildBreakValues(const LayoutResult& child_layout_result);

  bool ShouldCalculateScrollableOverflow() const {
    // Most nodes should calculate scrollable overflow and store it on the
    // resulting fragment, except for replaced content, and the root fragment
    // when paginating (the size of the root fragment is the same as the initial
    // containing block size, but page sizes may vary. Overflow calculation must
    // be done per fragmentainer).
    return node_ && !node_.IsReplaced() &&
           (!node_.IsPaginatedRoot() || IsFragmentainerBoxType());
  }

  // Handle (lay out / propagate) out-of-flow positioned descendants and other
  // special descendants. This function is to be called when an algorithm is
  // done with regular in-flow descendants and has set up its final size.
  void HandleOofsAndSpecialDescendants();

 private:
  // Propagate fragmentation details. This includes checking whether we have
  // fragmented in this flow, break appeal, column spanner detection, and column
  // balancing hints.
  void PropagateBreakInfo(const LayoutResult&, LogicalOffset);

  const LayoutResult* ToBoxFragment(WritingMode);

  const FragmentGeometry* initial_fragment_geometry_ = nullptr;
  BoxStrut border_padding_;
  BoxStrut border_scrollbar_padding_;
  // We clamp the block-start of |border_scrollbar_padding_| after an item
  // fragments. Store the original block-start, as well, for cases where it is
  // needed.
  LayoutUnit original_border_scrollbar_padding_block_start_;
  LogicalSize child_available_size_;
  LayoutUnit intrinsic_block_size_;
  std::optional<LogicalRect> inflow_bounds_;

  bool is_fieldset_container_ = false;
  bool is_table_part_ = false;
  bool is_initial_block_size_indefinite_ = false;
  bool is_inline_formatting_context_;
  bool is_known_to_fit_in_fragmentainer_ = false;
  bool is_block_size_for_fragmentation_clamped_ = false;
  bool is_monolithic_ = true;
  bool is_first_for_node_ = true;
  bool should_clone_box_end_decorations_ = false;
  bool should_prevent_break_before_block_end_decorations_ = false;
  bool did_break_self_ = false;
  bool has_inflow_child_break_inside_ = false;
  bool has_forced_break_ = false;
  bool has_seen_all_children_ = false;
  bool has_subsequent_children_ = false;
  bool is_math_fraction_ = false;
  bool is_math_operator_ = false;
  bool is_at_block_end_ = false;
  bool is_truncated_by_fragmentation_line = false;
  bool use_last_baseline_for_inline_baseline_ = false;
  bool has_moved_children_in_block_direction_ = false;

  // Whether the `text-box-trim` is effective for block-start/end edges of a
  // node.
  bool should_text_box_trim_node_start_ = false;
  bool should_text_box_trim_node_end_ = false;

  // Whether the `text-box-trim` is effective for block-start/end edges of a
  // fragmentainer.
  bool should_text_box_trim_fragmentainer_start_ = false;
  bool should_text_box_trim_fragmentainer_end_ = false;

  LayoutUnit block_offset_for_additional_columns_;

  LayoutUnit block_size_for_fragmentation_;

  // The break-before value on the initial child we cannot honor. There's no
  // valid class A break point before a first child, only *between* siblings.
  std::optional<EBreakBetween> initial_break_before_;

  // The break-after value of the previous in-flow sibling.
  EBreakBetween previous_break_after_ = EBreakBetween::kAuto;

  AtomicString page_name_ = g_null_atom;

  std::optional<LayoutUnit> first_baseline_;
  std::optional<LayoutUnit> last_baseline_;
  LayoutUnit math_italic_correction_;

  const GapGeometry* gap_geometry_ = nullptr;

  // Table specific types.
  std::optional<LogicalRect> table_grid_rect_;
  TableColumnGeometries table_column_geometries_;
  const TableBorders* table_collapsed_borders_ = nullptr;
  std::unique_ptr<CollapsedTableBordersGeometry>
      table_collapsed_borders_geometry_;
  std::optional<wtf_size_t> table_column_count_;

  // Table cell specific types.
  std::optional<wtf_size_t> table_cell_column_index_;
  wtf_size_t table_section_start_row_index_;
  Vector<LayoutUnit> table_section_row_offsets_;

  BlockBreakTokenData* break_token_data_ = nullptr;

  // Grid specific types.
  std::unique_ptr<GridLayoutData> grid_layout_data_;

  std::unique_ptr<DevtoolsFlexInfo> flex_layout_data_;
  std::unique_ptr<FrameSetLayoutData> frame_set_layout_data_;

  HeapVector<Member<blink::Node>> reading_flow_nodes_;

  LineLogicalBoxSides sides_to_include_;

  scoped_refptr<SerializedScriptValue> custom_layout_data_;

  const MathMLPaintInfo* mathml_paint_info_ = nullptr;

#if DCHECK_IS_ON()
  // Describes what size_.block_size represents; either the size of a single
  // fragment (false), or the size of all fragments for a node (true).
  bool block_size_is_for_all_fragments_ = false;

  // If any fragment has been added with an offset including the relative
  // position, we also need the inflow-bounds set explicitly.
  bool needs_inflow_bounds_explicitly_set_ = false;
  bool needs_may_have_descendant_above_block_start_explicitly_set_ = false;
  bool is_inflow_bounds_explicitly_set_ = false;
#endif

  friend class BlockBreakToken;
  friend class LayoutResult;
  friend class PhysicalBoxFragment;
  friend class PhysicalFragmentRareData;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BOX_FRAGMENT_BUILDER_H_
