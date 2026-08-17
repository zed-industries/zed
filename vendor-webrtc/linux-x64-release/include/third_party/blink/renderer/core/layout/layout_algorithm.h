// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/constraint_space.h"
#include "third_party/blink/renderer/core/layout/constraint_space_builder.h"
#include "third_party/blink/renderer/core/layout/fragmentation_utils.h"
#include "third_party/blink/renderer/core/layout/min_max_sizes.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ColumnSpannerPath;
class ComputedStyle;
class EarlyBreak;
class LayoutResult;

// Parameters to pass when creating a layout algorithm for a block node.
struct LayoutAlgorithmParams {
  STACK_ALLOCATED();

 public:
  LayoutAlgorithmParams(
      BlockNode node,
      const FragmentGeometry& fragment_geometry,
      const ConstraintSpace& space,
      const BlockBreakToken* break_token = nullptr,
      const EarlyBreak* early_break = nullptr,
      const HeapVector<Member<EarlyBreak>>* additional_early_breaks = nullptr)
      : node(node),
        fragment_geometry(fragment_geometry),
        space(space),
        break_token(break_token),
        early_break(early_break),
        additional_early_breaks(additional_early_breaks) {}

  BlockNode node;
  const FragmentGeometry& fragment_geometry;
  const ConstraintSpace& space;
  const BlockBreakToken* break_token;
  const EarlyBreak* early_break;
  const ColumnSpannerPath* column_spanner_path = nullptr;
  const LayoutResult* previous_result = nullptr;
  const HeapVector<Member<EarlyBreak>>* additional_early_breaks;
};

// Base class template for all layout algorithms.
//
// Subclassed template specializations (actual layout algorithms) are required
// to define the following two functions:
//
//   MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&);
//   const LayoutResult* Layout();
//
// ComputeMinMaxSizes() should compute the min-content and max-content intrinsic
// sizes for the given box. The result should not take any min-width, max-width
// or width properties into account.
//
// Layout() is the actual layout function. Lays out the children and descendants
// within the constraints given by the ConstraintSpace. Returns a layout result
// with the resulting layout information.
template <typename InputNodeType,
          typename BoxFragmentBuilderType,
          typename BreakTokenType>
class CORE_EXPORT LayoutAlgorithm {
  STACK_ALLOCATED();
 public:
  LayoutAlgorithm(InputNodeType node,
                  const ComputedStyle* style,
                  const ConstraintSpace& space,
                  TextDirection direction,
                  const BreakTokenType* break_token)
      : node_(node),
        container_builder_(node,
                           style,
                           space,
                           {space.GetWritingMode(), direction},
                           break_token) {}

  // Constructor for algorithms that use BoxFragmentBuilder and
  // BlockBreakToken.
  explicit LayoutAlgorithm(const LayoutAlgorithmParams& params)
      : node_(To<InputNodeType>(params.node)),
        early_break_(params.early_break),
        container_builder_(
            params.node,
            &params.node.Style(),
            params.space,
            {params.space.GetWritingMode(), params.space.Direction()},
            params.break_token),
        additional_early_breaks_(params.additional_early_breaks) {
    container_builder_.SetIsNewFormattingContext(
        params.space.IsNewFormattingContext());
    container_builder_.SetInitialFragmentGeometry(params.fragment_geometry);
    if (params.space.HasBlockFragmentation() ||
        IsBreakInside(params.break_token)) [[unlikely]] {
      SetupFragmentBuilderForFragmentation(
          params.space, params.node, params.break_token, &container_builder_);
    }
  }

 protected:
  // Protected (non-virtual) destructor, to make sure that the destructor is
  // invoked directly on subclasses.
  ~LayoutAlgorithm() = default;

  enum RelayoutType {
    kNoRelayout = 0,
    kRelayoutForEarlyBreak = 1,
    kRelayoutIgnoringLineClamp = 2,
    kRelayoutWithLineClampBlockSize = 4,
    kRelayoutForTextBoxTrim = 8,
    kRelayoutWithoutFragmentation = 16,
    kRelayoutIgnoringChildScrollbarChanges = 32,
    kRelayoutAsLastTableBox = 64,
  };
  // Bitmask of active relayout types (`RelayoutType`).
  typedef int RelayoutMode;

  const ConstraintSpace& GetConstraintSpace() const {
    return container_builder_.GetConstraintSpace();
  }

  const ComputedStyle& Style() const { return node_.Style(); }

  BfcOffset ContainerBfcOffset() const {
    DCHECK(container_builder_.BfcBlockOffset());
    return {container_builder_.BfcLineOffset(),
            *container_builder_.BfcBlockOffset()};
  }

  const InputNodeType& Node() const { return node_; }

  const BreakTokenType* GetBreakToken() const {
    return container_builder_.PreviousBreakToken();
  }

  const BoxStrut& Borders() const { return container_builder_.Borders(); }
  const BoxStrut& Scrollbar() const { return container_builder_.Scrollbar(); }
  const BoxStrut& Padding() const { return container_builder_.Padding(); }
  const BoxStrut& BorderPadding() const {
    return container_builder_.BorderPadding();
  }
  const BoxStrut& BorderScrollbarPadding() const {
    return container_builder_.BorderScrollbarPadding();
  }
  LayoutUnit OriginalBorderScrollbarPaddingBlockStart() const {
    return container_builder_.OriginalBorderScrollbarPaddingBlockStart();
  }
  const LogicalSize& ChildAvailableSize() const {
    return container_builder_.ChildAvailableSize();
  }

  ExclusionSpace& GetExclusionSpace() {
    return container_builder_.GetExclusionSpace();
  }

  LayoutUnit FragmentainerCapacityForChildren() const {
    return FragmentainerCapacity(container_builder_, /*is_for_children=*/true);
  }

  LayoutUnit FragmentainerOffsetForChildren() const {
    return FragmentainerOffset(container_builder_, /*is_for_children=*/true);
  }

  LayoutUnit FragmentainerSpaceLeftForChildren() const {
    return FragmentainerSpaceLeft(container_builder_, /*is_for_children=*/true);
  }

  BreakStatus BreakBeforeChildIfNeeded(LayoutInputNode child,
                                       const LayoutResult& layout_result,
                                       LayoutUnit fragmentainer_block_offset,
                                       bool has_container_separation) {
    return ::blink::BreakBeforeChildIfNeeded(
        GetConstraintSpace(), child, layout_result, fragmentainer_block_offset,
        FragmentainerCapacityForChildren(), has_container_separation,
        &container_builder_);
  }

  bool MovePastBreakpoint(LayoutInputNode child,
                          const LayoutResult& layout_result,
                          LayoutUnit fragmentainer_block_offset,
                          BreakAppeal appeal_before) {
    return ::blink::MovePastBreakpoint(
        GetConstraintSpace(), child, layout_result, fragmentainer_block_offset,
        FragmentainerCapacityForChildren(), appeal_before, &container_builder_);
  }

  bool MovePastBreakpoint(const LayoutResult& layout_result,
                          LayoutUnit fragmentainer_block_offset,
                          BreakAppeal appeal_before) {
    return ::blink::MovePastBreakpoint(
        GetConstraintSpace(), layout_result, fragmentainer_block_offset,
        FragmentainerCapacityForChildren(), appeal_before, &container_builder_);
  }

  // Carry over any previous relayout data from the previous layout algorithm,
  // and set up any new relayout data for the additional relayout type.
  // Subclasses that have data to set up on their own should override this
  // function and do their stuff in addition to calling this function.
  void SetupRelayoutData(const LayoutAlgorithm& previous_algorithm,
                         RelayoutType relayout_type) {
    if (relayout_mode_ & kRelayoutForEarlyBreak) {
      // We're not going to run out of space in the next layout pass, since
      // we're breaking earlier, so no space shortage will be detected. Repeat
      // what we found in this pass.
      container_builder_.PropagateSpaceShortage(
          previous_algorithm.container_builder_.MinimalSpaceShortage());
    }

    if (relayout_type != kRelayoutForEarlyBreak) {
      early_break_ = previous_algorithm.early_break_;
      additional_early_breaks_ = previous_algorithm.additional_early_breaks_;
    }
    container_builder_.SetBoxType(
        previous_algorithm.container_builder_.GetBoxType());
  }

  // Re-run this layout algorithm for the specified type of relayout.
  template <typename Algorithm>
  const LayoutResult* Relayout(RelayoutType relayout_type,
                               const EarlyBreak* breakpoint = nullptr,
                               const HeapVector<Member<EarlyBreak>>*
                                   additional_early_breaks = nullptr) const {
    // Don't retry with the same type.
    DCHECK(!(relayout_mode_ & relayout_type));

    // Early-breaks are only expected if we're actually re-laying out for an
    // early-break.
    DCHECK(!breakpoint || relayout_type == kRelayoutForEarlyBreak);
    DCHECK(!additional_early_breaks || relayout_type == kRelayoutForEarlyBreak);

    ConstraintSpace new_space = GetConstraintSpace();
    RelayoutMode new_relayout_mode = relayout_mode_ | relayout_type;
    if (new_relayout_mode & kRelayoutWithoutFragmentation) {
      // We'll relayout with a special cloned constraint space that disables
      // further fragmentation (but rather lets clipped child content "overflow"
      // past the fragmentation line). This means that the cached constraint
      // space will still be set up to do block fragmentation, but that should
      // be the right thing, since, as far as input is concerned, this node is
      // meant to perform block fragmentation (and it may already have produced
      // multiple fragments, but this one will be the last).
      new_space = new_space.CloneWithoutFragmentation();
    }

    LayoutAlgorithmParams params(
        Node(), container_builder_.InitialFragmentGeometry(), new_space,
        GetBreakToken(), breakpoint, additional_early_breaks);

    Algorithm relayout_algorithm(params);
    relayout_algorithm.relayout_mode_ = new_relayout_mode;
    relayout_algorithm.SetupRelayoutData(*static_cast<const Algorithm*>(this),
                                         relayout_type);
    return relayout_algorithm.Layout();
  }

  // Lay out again, this time with a predefined good breakpoint that we
  // discovered in the first pass. This happens when we run out of space in a
  // fragmentainer at an less-than-ideal location, due to breaking restrictions,
  // such as orphans, widows, break-before:avoid or break-after:avoid.
  template <typename Algorithm>
  const LayoutResult* RelayoutAndBreakEarlier(
      const EarlyBreak& breakpoint,
      const HeapVector<Member<EarlyBreak>>* additional_early_breaks = nullptr) {
    // Not allowed to recurse!
    DCHECK(!early_break_);
    DCHECK(!additional_early_breaks_ || additional_early_breaks_->empty());

    return Relayout<Algorithm>(kRelayoutForEarlyBreak, &breakpoint,
                               additional_early_breaks);
  }

  // Lay out again, this time without block fragmentation. This happens when a
  // block-axis clipped node reaches the end, but still has content inside that
  // wants to break. We don't want any zero-sized clipped fragments that
  // contribute to superfluous fragmentainers.
  template <typename Algorithm>
  const LayoutResult* RelayoutWithoutFragmentation() {
    DCHECK(GetConstraintSpace().HasBlockFragmentation());
    return Relayout<Algorithm>(kRelayoutWithoutFragmentation);
  }

  InputNodeType node_;

  // When set, this will specify where to break before or inside. If not set,
  // the algorithm will need to figure out where to break on its own.
  const EarlyBreak* early_break_ = nullptr;

  BoxFragmentBuilderType container_builder_;

  // There are cases where we may need more than one early break per fragment.
  // For example, there may be an early break within multiple flex columns. This
  // can be used to pass additional early breaks to the next layout pass.
  const HeapVector<Member<EarlyBreak>>* additional_early_breaks_ = nullptr;

  // The relayout types that are currently active. Every time Relayout() is
  // called, another RelayoutType is added to this field.
  RelayoutMode relayout_mode_ = kNoRelayout;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_ALGORITHM_H_
