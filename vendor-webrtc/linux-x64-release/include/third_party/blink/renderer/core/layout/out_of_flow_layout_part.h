// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OUT_OF_FLOW_LAYOUT_PART_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OUT_OF_FLOW_LAYOUT_PART_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/counters_attachment_context.h"
#include "third_party/blink/renderer/core/layout/absolute_utils.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_rect.h"
#include "third_party/blink/renderer/core/layout/geometry/static_position.h"
#include "third_party/blink/renderer/core/layout/inline/inline_containing_block_utils.h"
#include "third_party/blink/renderer/core/layout/non_overflowing_scroll_range.h"
#include "third_party/blink/renderer/core/style/computed_style_base_constants.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class BlockBreakToken;
class LayoutBox;
class LayoutObject;
class LayoutResult;
template <typename OffsetType>
class OofContainingBlock;
class SimplifiedOofLayoutAlgorithm;
struct LogicalOofPositionedNode;
template <typename OffsetType>
struct MulticolWithPendingOofs;

// Helper class for positioning of out-of-flow blocks.
// It should be used together with BoxFragmentBuilder.
// See BoxFragmentBuilder::AddOutOfFlowChildCandidate documentation
// for example of using these classes together.
class CORE_EXPORT OutOfFlowLayoutPart {
  STACK_ALLOCATED();

 public:
  explicit OutOfFlowLayoutPart(BoxFragmentBuilder* container_builder);
  void Run();

  struct ColumnBalancingInfo {
    DISALLOW_NEW();

   public:
    ColumnBalancingInfo() = default;

    bool HasOutOfFlowFragmentainerDescendants() const {
      return !out_of_flow_fragmentainer_descendants.empty();
    }
    void SwapOutOfFlowFragmentainerDescendants(
        HeapVector<LogicalOofNodeForFragmentation>* descendants) {
      DCHECK(descendants->empty());
      std::swap(out_of_flow_fragmentainer_descendants, *descendants);
    }

    void PropagateSpaceShortage(LayoutUnit space_shortage);

    // The list of OOF fragmentainer descendants of |columns|.
    HeapVector<LogicalOofNodeForFragmentation>
        out_of_flow_fragmentainer_descendants;
    // The smallest space shortage found while laying out the members of
    // |out_of_flow_fragmentainer_descendants| within the set of existing
    // |columns|.
    LayoutUnit minimal_space_shortage = kIndefiniteSize;
    // The number of new columns needed to hold the
    // |out_of_flow_fragmentainer_descendants| within the existing set of
    // |columns|.
    wtf_size_t num_new_columns = 0;
    // True if there is any violating breaks found when performing layout on the
    // |out_of_flow_fragmentainer_descendants|. Since break avoidance rules
    // don't apply to OOFs, this can only happen when a monolithic OOF has to
    // overflow.
    bool has_violating_break = false;

    void Trace(Visitor* visitor) const {
      visitor->Trace(out_of_flow_fragmentainer_descendants);
    }
  };

  // Specify a vector where child fragments are stored, rather than using the
  // fragment builder (the default). In some cases, what OutOfFlowLayoutPart
  // produces, isn't the final results, which means that they cannot be written
  // directly to the fragment builder.
  void SetChildFragmentStorage(
      FragmentBuilder::ChildrenVector* child_fragment_storage) {
    child_fragment_storage_ = child_fragment_storage;
  }

  // When handling fragmentation, perform layout on the column and OOF members
  // of |column_balancing_info| and |child_fragment_storage| rather than
  // directly on the builder, and keep track of any info needed for the OOF
  // children to affect column balancing.
  void SetColumnBalancingInfo(
      ColumnBalancingInfo* column_balancing_info,
      FragmentBuilder::ChildrenVector* child_fragment_storage) {
    DCHECK(column_balancing_info);
    DCHECK(child_fragment_storage);
    column_balancing_info_ = column_balancing_info;
    SetChildFragmentStorage(child_fragment_storage);
  }

  // Go through each page area fragment and propagate OOF descendants.
  void PropagateOOFsFromPageAreas();

  // Handle the layout of any OOF elements in a fragmentation context.
  void HandleFragmentation();

  // Return true if we were invoked at a pagination root, and any additional
  // pages that were laid out need to know the total page count (to support
  // counter(pages) in page margin boxes).
  bool NeedsTotalPageCount() { return needs_total_page_count_; }

  bool AdditionalPagesWereAdded() const { return additional_pages_were_added_; }

  // Information needed to position descendant within a containing block.
  // Geometry expressed here is complicated:
  // There are two types of containing blocks:
  // 1) Default containing block (DCB)
  //    Containing block passed in OutOfFlowLayoutPart constructor.
  //    It is the block element inside which this algorithm runs.
  //    All OOF descendants not in inline containing block are placed in DCB.
  // 2) Inline containing block
  //    OOF descendants might be positioned wrt inline containing block.
  //    Inline containing block is positioned wrt default containing block.
  struct ContainingBlockInfo {
    DISALLOW_NEW();

   public:
    // The writing direction of the container.
    WritingDirectionMode writing_direction = {WritingMode::kHorizontalTb,
                                              TextDirection::kLtr};
    // If the container is scrollable.
    bool is_scroll_container;
    // Size and offset of the container.
    LogicalRect rect;
    // The relative positioned offset to be applied after fragmentation is
    // completed.
    LogicalOffset relative_offset;
    // The offset of the container to its border box, including the block
    // contribution from previous fragmentainers.
    LogicalOffset offset_to_border_box;
  };

  // This stores the information needed to update a multicol child inside an
  // existing multicol fragment. This is used during nested fragmentation of an
  // OOF positioned element.
  //
  // TODO(layout-dev): Remove? This class now only holds a break token pointer,
  // so things can most likely be simplified.
  struct MulticolChildInfo {
    DISALLOW_NEW();

   public:
    Member<const BlockBreakToken> parent_break_token;

    void Trace(Visitor* visitor) const;
  };

  // Info needed to perform Layout() on an OOF positioned node.
  struct NodeInfo {
    DISALLOW_NEW();

   public:
    BlockNode node;
    const LogicalStaticPosition static_position;
    const ContainingBlockInfo base_container_info;
    const WritingDirectionMode default_writing_direction;
    const OofContainingBlock<LogicalOffset> containing_block;
    const OofContainingBlock<LogicalOffset> fixedpos_containing_block;
    const OofInlineContainer<LogicalOffset> fixedpos_inline_container;
    bool requires_content_before_breaking = false;
    bool is_hidden_for_paint = false;

    NodeInfo(BlockNode node,
             const LogicalStaticPosition static_position,
             const ContainingBlockInfo base_container_info,
             const WritingDirectionMode default_writing_direction,
             bool is_fragmentainer_descendant,
             const OofContainingBlock<LogicalOffset>& containing_block,
             const OofContainingBlock<LogicalOffset>& fixedpos_containing_block,
             const OofInlineContainer<LogicalOffset>& fixedpos_inline_container,
             bool requires_content_before_breaking,
             bool is_hidden_for_paint)
        : node(node),
          static_position(static_position),
          base_container_info(base_container_info),
          default_writing_direction(default_writing_direction),
          containing_block(containing_block),
          fixedpos_containing_block(fixedpos_containing_block),
          fixedpos_inline_container(fixedpos_inline_container),
          requires_content_before_breaking(requires_content_before_breaking),
          is_hidden_for_paint(is_hidden_for_paint) {}

    void Trace(Visitor* visitor) const;
  };

  // Stores the calculated offset for an OOF positioned node, along with the
  // information that was used in calculating the offset that will be used, in
  // addition to the information in NodeInfo, to perform a final layout
  // pass.
  struct OffsetInfo {
    DISALLOW_NEW();

   public:
    // Absolutized inset property values. Not necessarily the insets of the box.
    BoxStrut insets_for_get_computed_style;
    // Offset to container's border box.
    LogicalOffset offset;
    // Holds the initial layout result if we needed to know the size in order
    // to calculate the offset. If an initial result is set, it will either be
    // re-used or replaced in the final layout pass.
    Member<const LayoutResult> initial_layout_result;

    // The `block_estimate` and `container_content_size` is wrt. the
    // candidate's writing mode.
    std::optional<LayoutUnit> block_estimate;
    LogicalSize container_content_size;

    LogicalOofDimensions node_dimensions;

    // The offset from the OOF to the top of the fragmentation context root.
    // This should only be used when laying out a fragmentainer descendant.
    LogicalOffset original_offset;

    // This field is set only if this |OffsetInfo| is calculated from a
    // position-try-fallbacks style, either from a @position-try rule or a
    // tactic, or the anchored element has position-visibility: no-overflow.
    HeapVector<NonOverflowingScrollRange> non_overflowing_scroll_ranges;

    // This field is set when we're calculating |OffsetInfo| with
    // try_fit_available_space=true, e.g. when we have a non-empty
    // position-try-fallbacks. We have to retain the IMCB to implement
    // position-try-order, which decides which of the various candidates styles
    // we should select based on the biggest IMCB size (in some axis).
    std::optional<InsetModifiedContainingBlock> imcb_for_position_order;

    bool inline_size_depends_on_min_max_sizes = false;

    // True if this element is anchor-positioned, and any anchor reference in
    // the axis is in the same scroll container as the default anchor, in which
    // case we need scroll adjustment in the axis after layout.
    bool needs_scroll_adjustment_in_x = false;
    bool needs_scroll_adjustment_in_y = false;

    // True if the element overflows the inset-modified containing block.
    bool overflows_containing_block = false;

    Member<Element> accessibility_anchor;
    Member<GCedHeapHashSet<Member<Element>>> display_locks_affected_by_anchors;

    void Trace(Visitor* visitor) const;
  };

  struct NodeToLayout {
    DISALLOW_NEW();

   public:
    NodeInfo node_info;
    OffsetInfo offset_info;
    Member<const BlockBreakToken> break_token;

    // The physical fragment of the containing block used when laying out a
    // fragmentainer descendant. This is the containing block as defined by the
    // spec: https://www.w3.org/TR/css-position-3/#absolute-cb.
    Member<const PhysicalFragment> containing_block_fragment;

    void Trace(Visitor* visitor) const;
  };

  static std::optional<LogicalSize> InitialContainingBlockFixedSize(
      BlockNode container);

 private:
  const ContainingBlockInfo GetContainingBlockInfo(
      const LogicalOofPositionedNode&);

  FragmentationType GetFragmentainerType() const {
    if (container_builder_->Node().IsPaginatedRoot())
      return kFragmentPage;
    return kFragmentColumn;
  }
  const ConstraintSpace& GetConstraintSpace() const {
    return container_builder_->GetConstraintSpace();
  }

  void ComputeInlineContainingBlocks(
      const HeapVector<LogicalOofPositionedNode>&);
  void ComputeInlineContainingBlocksForFragmentainer(
      const HeapVector<LogicalOofNodeForFragmentation>&);
  // |containing_block_relative_offset| is the accumulated relative offset from
  // the inline's containing block to the fragmentation context root.
  // |containing_block_offset| is the offset of the inline's containing block
  // relative to the fragmentation context root (not including any offset from
  // relative positioning).
  void AddInlineContainingBlockInfo(
      const InlineContainingBlockUtils::InlineContainingBlockMap&,
      const WritingDirectionMode container_writing_direction,
      PhysicalSize container_builder_size,
      LogicalOffset containing_block_relative_offset = LogicalOffset(),
      LogicalOffset containing_block_offset = LogicalOffset(),
      bool adjust_for_fragmentation = false);

  void LayoutCandidates(HeapVector<LogicalOofPositionedNode>*);

  void HandleMulticolsWithPendingOOFs(BoxFragmentBuilder* container_builder);
  void LayoutOOFsInMulticol(
      const BlockNode& multicol,
      const MulticolWithPendingOofs<LogicalOffset>* multicol_info);

  // Layout the OOF nodes that are descendants of a fragmentation context root.
  // |multicol_children| holds the children of an inner multicol if
  // we are laying out OOF elements inside a nested fragmentation context.
  void LayoutFragmentainerDescendants(
      HeapVector<LogicalOofNodeForFragmentation>* descendants,
      LogicalOffset fragmentainer_progression,
      bool outer_context_has_fixedpos_container = false,
      HeapVector<MulticolChildInfo>* multicol_children = nullptr);

  AnchorEvaluatorImpl CreateAnchorEvaluator(
      const ContainingBlockInfo& container_info,
      const BlockNode& candidate,
      const StitchedAnchorQueries* anchor_queries) const;

  ContainingBlockInfo ApplyPositionAreaOffsets(
      const PositionAreaOffsets& offsets,
      PhysicalOffset default_anchor_scroll_shift,
      const ContainingBlockInfo& container_info) const;

  NodeInfo SetupNodeInfo(const LogicalOofPositionedNode& oof_node);

  const LayoutResult* LayoutOOFNode(
      NodeToLayout& oof_node_to_layout,
      const ConstraintSpace* fragmentainer_constraint_space = nullptr,
      bool is_last_fragmentainer_so_far = false);

  // TODO(almaher): We are calculating more than just the offset. Consider
  // changing this to a more accurate name.
  OffsetInfo CalculateOffset(
      const NodeInfo& node_info,
      const StitchedAnchorQueries* anchor_queries = nullptr);
  // Calculates offsets with the given ComputedStyle. Returns nullopt if
  // |try_fit_available_space| is true and the layout result does not fit the
  // available space.
  std::optional<OffsetInfo> TryCalculateOffset(
      const NodeInfo& node_info,
      const ComputedStyle& style,
      AnchorEvaluatorImpl&,
      std::optional<wtf_size_t> option_index,
      bool try_fit_available_space,
      PhysicalOffset default_anchor_scroll_shift,
      NonOverflowingScrollRange* out_scroll_range);

  const LayoutResult* Layout(
      const NodeToLayout& oof_node_to_layout,
      const ConstraintSpace* fragmentainer_constraint_space,
      bool is_last_fragmentainer_so_far);

  bool IsContainingBlockForCandidate(const LogicalOofPositionedNode&);

  const LayoutResult* GenerateFragment(
      const NodeToLayout& oof_node_to_layout,
      const ConstraintSpace* fragmentainer_constraint_space,
      bool is_last_fragmentainer_so_far);

  // Performs layout on the OOFs stored in |pending_descendants| and
  // |fragmented_descendants|, adding them as children in the fragmentainer
  // found at the provided |index|. If a fragmentainer does not already exist at
  // the given |index|, one will be created. The OOFs stored in
  // |fragmented_descendants| are those that are continuing layout from a
  // previous fragmentainer.  |fragmented_descendants| is also an output
  // variable in that any OOF that has not finished layout in the current pass
  // will be added back to |fragmented_descendants| to continue layout in the
  // next fragmentainer.  |has_actual_break_inside| will be set to true if any
  // of the OOFs laid out broke (this does not include repeated fixed-positioned
  // elements).
  void LayoutOOFsInFragmentainer(
      HeapVector<NodeToLayout>& pending_descendants,
      wtf_size_t index,
      LogicalOffset fragmentainer_progression,
      bool has_oofs_in_later_fragmentainer,
      LayoutUnit* monolithic_overflow,
      bool* has_actual_break_inside,
      HeapVector<NodeToLayout>* fragmented_descendants);
  void AddOOFToFragmentainer(NodeToLayout& descendant,
                             const ConstraintSpace* fragmentainer_space,
                             LogicalOffset fragmentainer_offset,
                             wtf_size_t index,
                             bool is_last_fragmentainer_so_far,
                             bool* has_actual_break_inside,
                             SimplifiedOofLayoutAlgorithm* algorithm,
                             HeapVector<NodeToLayout>* fragmented_descendants);
  ConstraintSpace GetFragmentainerConstraintSpace(wtf_size_t index);
  void ComputeStartFragmentIndexAndRelativeOffset(
      WritingMode default_writing_mode,
      LayoutUnit block_estimate,
      std::optional<LayoutUnit> clipped_container_block_offset,
      wtf_size_t* start_index,
      LogicalOffset* offset) const;

  // This saves the static-position for an OOF-positioned object into its
  // paint-layer.
  void SaveStaticPositionOnPaintLayer(
      LayoutBox* layout_box,
      const LogicalStaticPosition& position) const;
  LogicalStaticPosition ToStaticPositionForLegacy(
      LogicalStaticPosition position) const;

  const FragmentBuilder::ChildrenVector& FragmentationContextChildren() const {
    DCHECK(container_builder_->IsBlockFragmentationContextRoot());
    return child_fragment_storage_ ? *child_fragment_storage_
                                   : container_builder_->Children();
  }

  // Get the child / descendant fragment at the specified index. These are
  // normally fragmentainers, but for multicol, column spanners are also
  // included. For paginated layout, a fragmentainer (page area fragment) is
  // always returned, but note that these are not direct child fragments of the
  // fragmentation context root (a page area is a child of a page border box,
  // which is a child of a page container).
  const PhysicalBoxFragment& GetChildFragment(wtf_size_t index) const;

  wtf_size_t ChildCount() const {
    return FragmentationContextChildren().size();
  }

  void AddFragmentainer(const PhysicalBoxFragment& fragmentainer,
                        LogicalOffset fragmentainer_offset) {
    if (child_fragment_storage_) {
      child_fragment_storage_->push_back(
          LogicalFragmentLink{fragmentainer, fragmentainer_offset});
    } else {
      container_builder_->AddChild(fragmentainer, fragmentainer_offset);
    }
  }

  // Return the break token of the previous fragmentainer to the child at
  // `index`.
  const BlockBreakToken* PreviousFragmentainerBreakToken(wtf_size_t) const;

  BoxFragmentBuilder* container_builder_;
  // The OutOfFlowLayoutPart for the outer block fragmentation context when this
  // is an inner layout of nested block fragmentation.
  OutOfFlowLayoutPart* outer_oof_layout_part_ = nullptr;
  ContainingBlockInfo default_containing_block_info_for_absolute_;
  ContainingBlockInfo default_containing_block_info_for_fixed_;
  HeapHashMap<Member<const LayoutObject>, ContainingBlockInfo>
      containing_blocks_map_;

  // Out-of-flow positioned nodes that we should lay out at a later time. For
  // example, if the containing block has not finished layout.
  HeapVector<LogicalOofNodeForFragmentation> delayed_descendants_;

  // Holds the children of an inner multicol if we are laying out OOF elements
  // inside a nested fragmentation context.
  HeapVector<MulticolChildInfo>* multicol_children_;
  // If set, we are currently attempting to balance the columns of a multicol.
  // In which case, we need to know how much any OOF fragmentainer descendants
  // will affect column balancing, if any, without actually adding the OOFs to
  // the associated columns.
  ColumnBalancingInfo* column_balancing_info_ = nullptr;

  // When set, child fragments will be stored in this vector, rather than in the
  // fragment builder.
  FragmentBuilder::ChildrenVector* child_fragment_storage_ = nullptr;

  // The consumed block size of previous fragmentainers. This is accumulated and
  // used as we add OOF elements to fragmentainers.
  LayoutUnit fragmentainer_consumed_block_size_;
  bool is_absolute_container_ = false;
  bool is_fixed_container_ = false;
  bool has_block_fragmentation_ = false;
  // A fixedpos containing block was found in an outer fragmentation context.
  bool outer_context_has_fixedpos_container_ = false;

  // Set if any additional pages that were laid out need to know the total page
  // count.
  bool needs_total_page_count_ = false;

  bool additional_pages_were_added_ = false;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::OutOfFlowLayoutPart::MulticolChildInfo)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::OutOfFlowLayoutPart::NodeToLayout)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OUT_OF_FLOW_LAYOUT_PART_H_
