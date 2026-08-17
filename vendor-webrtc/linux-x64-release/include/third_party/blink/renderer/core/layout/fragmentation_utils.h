// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAGMENTATION_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAGMENTATION_UTILS_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/block_break_token.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/column_spanner_path.h"
#include "third_party/blink/renderer/core/layout/constraint_space.h"
#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/core/layout/inline/inline_break_token.h"
#include "third_party/blink/renderer/core/layout/inline/inline_item.h"
#include "third_party/blink/renderer/core/layout/physical_fragment.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

class EarlyBreak;
class LayoutResult;

// Each column in a flex container is fragmented independently, so we need to
// track early break and break-after info for each column separately.
struct FlexColumnBreakInfo {
  DISALLOW_NEW();

  FlexColumnBreakInfo() = default;

  void Trace(Visitor* visitor) const { visitor->Trace(early_break); }

  LayoutUnit column_intrinsic_block_size;
  Member<EarlyBreak> early_break = nullptr;
  EBreakBetween break_after = EBreakBetween::kAuto;
};

// Join two adjacent break values specified on break-before and/or break-
// after. avoid* values win over auto values, and forced break values win over
// avoid* values. |first_value| is specified on an element earlier in the flow
// than |second_value|. This method is used at class A break points [1], to join
// the values of the previous break-after and the next break-before, to figure
// out whether we may, must, or should not break at that point. It is also used
// when propagating break-before values from first children and break-after
// values on last children to their container.
//
// [1] https://drafts.csswg.org/css-break/#possible-breaks
EBreakBetween JoinFragmentainerBreakValues(EBreakBetween first_value,
                                           EBreakBetween second_value);

// Return true if the specified break value has a forced break effect in the
// current fragmentation context.
bool IsForcedBreakValue(const ConstraintSpace&, EBreakBetween);

// Return true if the specified break value means that we should avoid breaking,
// given the current fragmentation context.
template <typename Property>
bool IsAvoidBreakValue(const ConstraintSpace&, Property);

// Return true if this is a break inside a node (i.e. it's not a break *before*
// something, and also not for repeated content).
inline bool IsBreakInside(const BlockBreakToken* token) {
  return token && !token->IsBreakBefore() && !token->IsRepeated();
}

// Return true if the node may break into multiple fragments (or has already
// broken). In some situations we'll disable block fragmentation while in the
// middle of layout of a node (to prevent superfluous empty fragments, if
// overflow is clipped). In some cases it's not enough to just check if we're
// currently performing block fragmentation; we also need to know if it has
// already been fragmented (to resume layout correctly, but not break again).
inline bool InvolvedInBlockFragmentation(const BoxFragmentBuilder& builder) {
  return builder.GetConstraintSpace().HasBlockFragmentation() ||
         IsBreakInside(builder.PreviousBreakToken());
}

// Return the fragment index (into the layout results vector in LayoutBox),
// based on incoming break token.
inline wtf_size_t FragmentIndex(const BlockBreakToken* incoming_break_token) {
  if (incoming_break_token && !incoming_break_token->IsBreakBefore())
    return incoming_break_token->SequenceNumber() + 1;
  return 0;
}

// Calculate the final "break-between" value at a class A or C breakpoint. This
// is the combination of all break-before and break-after values that met at the
// breakpoint.
EBreakBetween CalculateBreakBetweenValue(LayoutInputNode child,
                                         const LayoutResult&,
                                         const BoxFragmentBuilder&);

// Return true if the container is being resumed after a fragmentainer break,
// and the child is at the first fragment of a node, and we are allowed to break
// before it. Normally, this isn't allowed, as that would take us nowhere,
// progress-wise, but for multicol in nested fragmentation, we'll allow it in
// some cases. If we set the appeal of breaking before the first child high
// enough, we'll automatically discard any subsequent less perfect
// breakpoints. This will make us push everything that would break with an
// appeal lower than the minimum appeal (stored in the constraint space) ahead
// of us, until we reach the next column row (in the next outer fragmentainer).
// That row may be taller, which might help us avoid breaking violations.
bool IsBreakableAtStartOfResumedContainer(
    const ConstraintSpace& space,
    const LayoutResult& child_layout_result,
    const BoxFragmentBuilder& builder);

bool IsBreakableAtStartOfResumedContainer(const ConstraintSpace& space,
                                          const BoxFragmentBuilder& builder,
                                          bool is_first_for_node);

// Calculate the appeal of breaking before this child.
BreakAppeal CalculateBreakAppealBefore(const ConstraintSpace&,
                                       LayoutInputNode child,
                                       const LayoutResult&,
                                       const BoxFragmentBuilder&,
                                       bool has_container_separation);
BreakAppeal CalculateBreakAppealBefore(
    const ConstraintSpace&,
    LayoutResult::EStatus layout_result_status,
    EBreakBetween break_between,
    bool has_container_separation,
    bool breakable_at_start_of_container);

// Calculate the appeal of breaking inside this child. The appeal is based on
// the one stored in the layout result, unless hypothetical_appeal is specified.
// hypothetical_appeal is used to assess the appeal at breakpoints where we
// didn't break, but still need to consider (see EarlyBreak).
BreakAppeal CalculateBreakAppealInside(
    const ConstraintSpace& space,
    const LayoutResult&,
    std::optional<BreakAppeal> hypothetical_appeal = std::nullopt);

// To ensure content progression, we need fragmentainers to hold something
// larger than 0. The spec says that fragmentainers have to accept at least 1px
// of content. See https://www.w3.org/TR/css-break-3/#breaking-rules
inline LayoutUnit ClampedToValidFragmentainerCapacity(LayoutUnit length) {
  return std::max(length, LayoutUnit(1));
}
// This function is most commonly used to figure out space available to children
// of a builder, but if it's used to figure out the minimum valid fragmentainer
// size for the fragment itself, `is_for_children` may be cleared, so that any
// cloned box decorations are included. Such box decorations will otherwise be
// subtracted, since children should steer clear of them.
inline LayoutUnit ClampedToValidFragmentainerCapacity(
    const BoxFragmentBuilder& builder,
    LayoutUnit length,
    bool is_for_children) {
  LayoutUnit minimum(1);
  if (builder.ShouldCloneBoxEndDecorations() && !is_for_children) {
    minimum += builder.BorderScrollbarPadding().BlockSum();
  }
  return std::max(length, minimum);
}

// Return the logical size of the specified fragmentainer, with
// clamping block_size.
LogicalSize FragmentainerLogicalCapacity(
    const PhysicalBoxFragment& fragmentainer);

// Return the fragmentainer block-size to use during layout. This is normally
// the same as the block-size we'll give to the fragment itself, but in order to
// ensure content progression, we need fragmentainers to hold something larger
// than 0 (even if the final fragentainer size may very well be 0). The spec
// says that fragmentainers have to accept at least 1px of content. See
// https://www.w3.org/TR/css-break-3/#breaking-rules
//
// This function is most commonly used to figure out space available to children
// of a builder, but if it's used to figure out the space available the fragment
// itself, `is_for_children` may be cleared, so that any cloned box decorations
// are included. Such box decorations will otherwise be subtracted, since
// children should steer clear of them.
inline LayoutUnit FragmentainerCapacity(const BoxFragmentBuilder& builder,
                                        bool is_for_children) {
  const ConstraintSpace& space = builder.GetConstraintSpace();
  if (!space.HasKnownFragmentainerBlockSize())
    return kIndefiniteSize;
  LayoutUnit size = space.FragmentainerBlockSize();
  if (builder.Style().BoxDecorationBreak() == EBoxDecorationBreak::kClone &&
      is_for_children) {
    // Cloned box decorations effectively shrinks the fragmentainer space
    // available to children.
    size -= builder.BorderScrollbarPadding().block_start;
    if (builder.ShouldCloneBoxEndDecorations()) {
      size -= builder.BorderScrollbarPadding().block_end;
    }
  }
  return ClampedToValidFragmentainerCapacity(builder, size, is_for_children);
}

// Get the start block-offset relatively to the fragmentainer start.
//
// This function is most commonly called for children of a builder, but if it's
// used to figure out the offset for the fragment itself, `is_for_children` may
// be cleared, so that any cloned box decorations are included. Such box
// decorations will otherwise be subtracted, since children should steer clear
// of them.
inline LayoutUnit FragmentainerOffset(const BoxFragmentBuilder& builder,
                                      bool is_for_children = true) {
  const ConstraintSpace& space = builder.GetConstraintSpace();
  if (!space.HasBlockFragmentation()) {
    return LayoutUnit();
  }
  LayoutUnit offset = space.FragmentainerOffset();
  if (builder.Style().BoxDecorationBreak() == EBoxDecorationBreak::kClone &&
      is_for_children) {
    // Adjust the fragmentainer offset, so that any child inside that's at the
    // block-start content edge is seen as offset 0, which helps the
    // fragmentainer machinery to not insert a break right after another break,
    // before there has been some content progress. This fragmentainer offset
    // isn't used to position things in layout, but only to determine whether
    // we're at the beginning of a fragmentainer, and by the layout cache (so
    // that we miss if a fragmented box got its offset changed), so adjusting it
    // like this should be fine.
    const BlockBreakToken* break_token = builder.PreviousBreakToken();
    if (!break_token || !break_token->IsAtBlockEnd()) {
      offset -= builder.BorderScrollbarPadding().block_start;
    }
  }
  return offset;
}

// Return the block space that was available in the current fragmentainer at the
// start of the current block. Note that if the start of the current block is in
// a previous fragmentainer, the size of the current fragmentainer is returned
// instead. If available space is negative, zero is returned. In the case of
// initial column balancing, the size is unknown, in which case kIndefiniteSize
// is returned.
//
// This function is most commonly used to figure out space available to children
// of a builder, but if it's used to figure out the space available the fragment
// itself, `is_for_children` may be cleared, so that any cloned box decorations
// are included. Such box decorations will otherwise be subtracted, since
// children should steer clear of them.
inline LayoutUnit FragmentainerSpaceLeft(const BoxFragmentBuilder& builder,
                                         bool is_for_children) {
  const ConstraintSpace& space = builder.GetConstraintSpace();
  if (!space.HasKnownFragmentainerBlockSize())
    return kIndefiniteSize;
  LayoutUnit capacity = FragmentainerCapacity(builder, is_for_children);
  LayoutUnit offset = FragmentainerOffset(builder, is_for_children);
  return (capacity - offset).ClampNegativeToZero();
}

// Return the border edge block-offset from the block-start of the fragmentainer
// relative to the block-start of the current block formatting context in the
// current fragmentainer. Note that if the current block formatting context
// starts in a previous fragmentainer, we'll return the block-offset relative to
// the current fragmentainer.
inline LayoutUnit FragmentainerOffsetAtBfc(const ConstraintSpace& space) {
  return space.FragmentainerOffset() - space.ExpectedBfcBlockOffset();
}
inline LayoutUnit FragmentainerOffsetAtBfc(const BoxFragmentBuilder& builder) {
  return FragmentainerOffset(builder) -
         builder.GetConstraintSpace().ExpectedBfcBlockOffset();
}

// Adjust margins to take fragmentation into account. Leading/trailing block
// margins must be applied to at most one fragment each. Leading block margins
// come before the first fragment (if at all; see below), and trailing block
// margins come right after the fragment that has any trailing padding+border
// (note that this may not be the final fragment, if children overflow; see
// below). For all other fragments, leading/trailing block margins must be
// ignored.
inline void AdjustMarginsForFragmentation(const BlockBreakToken* break_token,
                                          BoxStrut* box_strut) {
  if (!break_token)
    return;

  // Leading block margins are truncated if they come right after an unforced
  // break (except for floats; floats never truncate margins). And they should
  // only occur in front of the first fragment.
  if (!break_token->IsBreakBefore() ||
      (!break_token->IsForcedBreak() && !break_token->InputNode().IsFloating()))
    box_strut->block_start = LayoutUnit();

  // If we're past the block end, we are in a parallel flow (caused by content
  // overflow), and any trailing block margin has already been applied in the
  // fragmentainer where the block actually ended.
  if (break_token->IsAtBlockEnd())
    box_strut->block_end = LayoutUnit();
}

// Get the offset from one fragmentainer to the next.
LogicalOffset GetFragmentainerProgression(const BoxFragmentBuilder&,
                                          FragmentationType);

// Set up a child's constraint space builder for block fragmentation. The child
// participates in the same fragmentation context as parent_space.
// |requires_content_before_breaking| is set when inside node that we know will
// fit (and stay) in the current fragmentainer. See
// MustStayInCurrentFragmentainer() in BoxFragmentBuilder.
void SetupSpaceBuilderForFragmentation(const ConstraintSpace& parent_space,
                                       const LayoutInputNode& child,
                                       LayoutUnit fragmentainer_offset,
                                       LayoutUnit fragmentainer_block_size,
                                       bool requires_content_before_breaking,
                                       ConstraintSpaceBuilder*);
// If the child establishes a new formatting context,
// |fragmentainer_offset_delta| must be set to the offset from the parent block
// formatting context, or, if the parent formatting context starts in a previous
// fragmentainer; the offset from the current fragmentainer block-start.
void SetupSpaceBuilderForFragmentation(
    const BoxFragmentBuilder& parent_fragment_builder,
    const LayoutInputNode& child,
    LayoutUnit fragmentainer_offset_delta,
    ConstraintSpaceBuilder*);

// Set up a node's fragment builder for block fragmentation. To be done at the
// beginning of layout.
void SetupFragmentBuilderForFragmentation(
    const ConstraintSpace&,
    const LayoutInputNode&,
    const BlockBreakToken* previous_break_token,
    BoxFragmentBuilder*);

// Return whether any block-start border+padding should be included in the
// fragment being generated. Only one of the fragments should include this,
// unless box decorations are to be cloned.
bool ShouldIncludeBlockStartBorderPadding(const BoxFragmentBuilder&);

// Return whether any block-end border+padding should be included in the
// fragment being generated. Only one of the fragments should include this,
// unless box decorations are to be cloned.
bool ShouldIncludeBlockEndBorderPadding(const BoxFragmentBuilder&);

// Return the size of the block-start box decorations, if they are cloned. In
// the cloning box decoration model, block-start box decoration are considered
// cloned in all fragments but the first.
inline LayoutUnit ClonedBlockStartDecoration(
    const BoxFragmentBuilder& builder) {
  const BlockBreakToken* break_token = builder.PreviousBreakToken();
  if (builder.Style().BoxDecorationBreak() == EBoxDecorationBreak::kClone &&
      IsBreakInside(break_token) && !break_token->IsAtBlockEnd()) {
    return builder.BorderScrollbarPadding().block_start;
  }
  return LayoutUnit();
}

// Outcome of considering (and possibly attempting) breaking before or inside a
// child.
enum class BreakStatus {
  // Continue layout. No break was inserted in this operation.
  kContinue,

  // A break was inserted before the child. Discard the child fragment and
  // finish layout of the container. If there was a break inside the child, it
  // will be discarded along with the child fragment.
  kBrokeBefore,

  // The fragment couldn't fit here, but no break was inserted before/inside the
  // child, as it was an unappealing place to break, and we have a better
  // earlier breakpoint. We now need to abort the current layout, and go back
  // and re-layout to said earlier breakpoint.
  kNeedsEarlierBreak,

  // The node broke inside when it's not allowed to generate more fragments
  // (than the one we're working on right now). This happens when a child inside
  // an overflow:clip box breaks, and we're past the block-end edge of the
  // overflow:clip box. The fragmentation engine has one job: to insert breaks
  // in order to prevent content from overflowing the fragmentainers, but if
  // we're past the block-end edge of a clipped box, there'll be no
  // fragmentainer overflow, and therefore no need for breaks.
  kDisableFragmentation,
};

// Update and write fragmentation information to the fragment builder after
// layout. This will update the block-size stored in the builder. It may also
// update the stored intrinsic block-size.
//
// When calculating the block-size, a layout algorithm will include the
// accumulated block-size of all fragments generated for this node - as if they
// were all stitched together as one tall fragment. This is the most convenient
// thing to do, since any block-size specified in CSS applies to the entire box,
// regardless of fragmentation. This function will update the block-size to the
// actual fragment size, by examining possible breakpoints, if necessary.
//
// Return kContinue if we're allowed to generate a fragment. Otherwise, it means
// that we need to abort and relayout, either because we ran out of space at a
// less-than-ideal location (kNeedsEarlierBreak) - in this case between the last
// child and the block-end padding / border, or, because we need to disable
// fragmentation (kDisableFragmentation). kBrokeBefore is never returned here
// (if we need a break before the node, that's something that will be determined
// by the parent algorithm).
BreakStatus FinishFragmentation(BoxFragmentBuilder*);

// Special rules apply for finishing fragmentation when building fragmentainers.
BreakStatus FinishFragmentationForFragmentainer(BoxFragmentBuilder*);

// Return true if there's a valid class A/B breakpoint between the child
// fragment that was just added to the builder, and the next sibling, if one is
// added.
bool HasBreakOpportunityBeforeNextChild(
    const PhysicalFragment& child_fragment,
    const BreakToken* incoming_child_break_token);

// Insert a fragmentainer break before the child if necessary. In that case, the
// previous in-flow position will be updated, we'll return |kBrokeBefore|. If we
// don't break inside, we'll consider the appeal of doing so anyway (and store
// it as the most appealing break point so far if that's the case), since we
// might have to go back and break here. Return |kContinue| if we're to continue
// laying out. If |kNeedsEarlierBreak| is returned, it means that we ran out of
// space, but shouldn't break before the child, but rather abort layout, and
// re-layout to a previously found good breakpoint. |kDisableFragmentation| will
// never be returned from this function (we need to finish layout of the
// container before we can tell whether we reached the end). If
// |has_container_separation| is true, it means that we're at a valid
// breakpoint. We obviously prefer valid breakpoints, but sometimes we need to
// break at undesirable locations. Class A breakpoints occur between block
// siblings. Class B breakpoints between line boxes. Both these breakpoint
// classes imply that we're already past the first in-flow child in the
// container, but there's also another way of achieving container separation:
// class C breakpoints. Those occur if there's a positive gap between the
// block-start content edge of the container and the block-start margin edge of
// the first in-flow child. https://www.w3.org/TR/css-break-3/#possible-breaks
// If |flex_column_break_info| is supplied, we are running layout for a column
// flex container, in which case, we may be tracking certain break behavior at
// the column level.
BreakStatus BreakBeforeChildIfNeeded(
    const ConstraintSpace&,
    LayoutInputNode child,
    const LayoutResult&,
    LayoutUnit fragmentainer_block_offset,
    LayoutUnit fragmentainer_block_size,
    bool has_container_separation,
    BoxFragmentBuilder*,
    bool is_row_item = false,
    FlexColumnBreakInfo* flex_column_break_info = nullptr);

// Insert a break before the child, and propagate space shortage if needed.
// |block_size_override| should only be supplied when you wish to propagate a
// different block-size than that of the provided layout result.
void BreakBeforeChild(
    const ConstraintSpace&,
    LayoutInputNode child,
    const LayoutResult*,
    LayoutUnit fragmentainer_block_offset,
    LayoutUnit fragmentainer_block_size,
    std::optional<BreakAppeal> appeal,
    bool is_forced_break,
    BoxFragmentBuilder*,
    std::optional<LayoutUnit> block_size_override = std::nullopt);

// Propagate the block-size of unbreakable content. This is used to inflate the
// initial minimal column block-size when balancing columns, before we calculate
// a tentative (or final) column block-size. Unbreakable content will actually
// fragment if the columns aren't large enough, and we want to prevent that, if
// possible.
inline void PropagateUnbreakableBlockSize(LayoutUnit block_size,
                                          LayoutUnit fragmentainer_block_offset,
                                          BoxFragmentBuilder* builder) {
  // Whatever is before the block-start of the fragmentainer isn't considered to
  // intersect with the fragmentainer, so subtract it (by adding the negative
  // offset).
  if (fragmentainer_block_offset < LayoutUnit())
    block_size += fragmentainer_block_offset;
  builder->PropagateTallestUnbreakableBlockSize(block_size);
}

// Propagate space shortage to the builder and beyond, if appropriate. This is
// something we do during column balancing, when we already have a tentative
// column block-size, as a means to calculate by how much we need to stretch the
// columns to make everything fit. |block_size_override| should only be supplied
// when you wish to propagate a different block-size than that of the provided
// layout result.
void PropagateSpaceShortage(
    const ConstraintSpace&,
    const LayoutResult*,
    LayoutUnit fragmentainer_block_offset,
    LayoutUnit fragmentainer_block_size,
    FragmentBuilder*,
    std::optional<LayoutUnit> block_size_override = std::nullopt);

// Calculate how much we would need to stretch the column block-size to fit the
// current result (if applicable). |block_size_override| should only be supplied
// when you wish to propagate a different block-size than that of the provided
// layout result.
LayoutUnit CalculateSpaceShortage(
    const ConstraintSpace&,
    const LayoutResult*,
    LayoutUnit fragmentainer_block_offset,
    LayoutUnit fragmentainer_block_size,
    std::optional<LayoutUnit> block_size_override = std::nullopt);
// Update |minimal_space_shortage| based on the current |space_shortage|.
void UpdateMinimalSpaceShortage(std::optional<LayoutUnit> space_shortage,
                                LayoutUnit* minimal_space_shortage);

// Move past the breakpoint before the child, if possible, and return true. Also
// update the appeal of breaking before or inside the child (if we're not going
// to break before it). If false is returned, it means that we need to break
// before the child (or even earlier). See BreakBeforeChildIfNeeded() for
// details on |flex_column_break_info|.
bool MovePastBreakpoint(const ConstraintSpace& space,
                        LayoutInputNode child,
                        const LayoutResult& layout_result,
                        LayoutUnit fragmentainer_block_offset,
                        LayoutUnit fragmentainer_block_size,
                        BreakAppeal appeal_before,
                        BoxFragmentBuilder* builder,
                        bool is_row_item = false,
                        FlexColumnBreakInfo* flex_column_break_info = nullptr);

// Same as above, but without the parts that require an LayoutInputNode.
bool MovePastBreakpoint(const ConstraintSpace& space,
                        const LayoutResult& layout_result,
                        LayoutUnit fragmentainer_block_offset,
                        LayoutUnit fragmentainer_block_size,
                        BreakAppeal appeal_before,
                        BoxFragmentBuilder* builder,
                        bool is_row_item = false,
                        FlexColumnBreakInfo* flex_column_break_info = nullptr);

// If the appeal of breaking before or inside the child is the same or higher
// than any previous breakpoint we've found, set a new breakpoint in the
// builder, and update appeal accordingly. See BreakBeforeChildIfNeeded() for
// details on |flex_column_break_info|.
void UpdateEarlyBreakAtBlockChild(
    const ConstraintSpace&,
    BlockNode child,
    const LayoutResult&,
    BreakAppeal appeal_before,
    BoxFragmentBuilder*,
    FlexColumnBreakInfo* flex_column_break_info = nullptr);

// Attempt to insert a soft break before the child, and return true if we did.
// If false is returned, it means that the desired breakpoint is earlier in the
// container, and that we need to abort and re-layout to that breakpoint.
// |block_size_override| should only be supplied when you wish to propagate a
// different block-size than that of the provided layout result. See
// BreakBeforeChildIfNeeded() for details on |flex_column_break_info|.
bool AttemptSoftBreak(
    const ConstraintSpace&,
    LayoutInputNode child,
    const LayoutResult*,
    LayoutUnit fragmentainer_block_offset,
    LayoutUnit fragmentainer_block_size,
    BreakAppeal appeal_before,
    BoxFragmentBuilder*,
    std::optional<LayoutUnit> block_size_override = std::nullopt,
    FlexColumnBreakInfo* flex_column_break_info = nullptr);

// If we have an previously found break point, and we're entering an ancestor of
// the node we're going to break before, return the early break inside. This can
// then be passed to child layout, so that child layout eventually can tell
// where to insert the break.
const EarlyBreak* EnterEarlyBreakInChild(const BlockNode& child,
                                         const EarlyBreak&);

// Return true if this is the child that we had previously determined to break
// before.
bool IsEarlyBreakTarget(const EarlyBreak&,
                        const BoxFragmentBuilder&,
                        const LayoutInputNode& child);

// Find out if |child| is the next step on the column spanner path (if any), and
// return the remaining path if that's the case, nullptr otherwise.
inline const ColumnSpannerPath* FollowColumnSpannerPath(
    const ColumnSpannerPath* path,
    const BlockNode& child) {
  if (!path)
    return nullptr;
  const ColumnSpannerPath* next_step = path->Child();
  if (next_step && next_step->GetBlockNode() == child) {
    return next_step;
  }
  return nullptr;
}

// Set up a constraint space for columns in multi-column layout, or for pages
// when printing; as specified by fragmentation_type.
ConstraintSpace CreateConstraintSpaceForFragmentainer(
    const ConstraintSpace& parent_space,
    FragmentationType fragmentation_type,
    LogicalSize fragmentainer_size,
    LogicalSize percentage_resolution_size,
    bool balance_columns,
    BreakAppeal min_break_appeal,
    const BoxFragmentBuilder* = nullptr);

// Calculate the container builder and constraint space for a multicol.
BoxFragmentBuilder CreateContainerBuilderForMulticol(
    const BlockNode& multicol,
    const ConstraintSpace& space,
    const FragmentGeometry& fragment_geometry);
ConstraintSpace CreateConstraintSpaceForMulticol(const BlockNode& multicol);

// Return the adjusted child margin to be applied at the end of a fragment.
// Margins should collapse with the fragmentainer boundary. |block_offset| is
// the block-offset where the margin should be applied (i.e. after the block-end
// border edge of the last child fragment).
inline LayoutUnit AdjustedMarginAfterFinalChildFragment(
    const BoxFragmentBuilder& builder,
    LayoutUnit block_offset,
    LayoutUnit block_end_margin) {
  LayoutUnit space_left =
      FragmentainerSpaceLeft(builder, /*is_for_children=*/true) - block_offset;
  return std::min(block_end_margin, space_left.ClampNegativeToZero());
}

// Return the break token that led to the creation of the fragment specified, or
// nullptr if this is the first fragment. Note that this operation is O(n)
// (number of fragments generated from the node), and should be avoided when
// possible. This function should no longer be necessary once everything has
// been properly converted to LayoutNG.
const BlockBreakToken* FindPreviousBreakToken(const PhysicalBoxFragment&);

// Return the LayoutBox::PhysicalFragments() index for this fragment.
wtf_size_t BoxFragmentIndex(const PhysicalBoxFragment&);

// Return the index of the fragmentainer preceding the first fragmentainer
// inside this fragment. Used by nested block fragmentation.
wtf_size_t PreviousInnerFragmentainerIndex(const PhysicalBoxFragment&);

// Return the fragment's offset relatively to the top/left corner of an
// imaginary box where all fragments generated by the node have been stitched
// together. If |out_stitched_fragments_size| is specified, it will be set to
// the size of this imaginary box.
PhysicalOffset OffsetInStitchedFragments(
    const PhysicalBoxFragment&,
    PhysicalSize* out_stitched_fragments_size = nullptr);

// Return the block-size that this fragment will take up inside a fragmentation
// context. This will include overflow from descendants (if it is visible and
// supposed to affect block fragmentation), and also out-of-flow positioned
// descendants (in the initial balancing pass), but not relative offsets.
LayoutUnit BlockSizeForFragmentation(
    const LayoutResult&,
    WritingDirectionMode container_writing_direction);

// Return true if we support painting of multiple fragments for the given
// content. Will return true for anything that is fragmentable / non-monolithic.
// Will also return true for certain types of monolithic content, because, even
// if it's unbreakable, it may generate multiple fragments, if it's part of
// repeated content (such as table headers and footers). This is the case for
// e.g. images, which may for instance be repeated in table headers /
// footers. Return false for monolithic content that we don't want to repeat
// (e.g. iframes).
bool CanPaintMultipleFragments(const PhysicalBoxFragment&);
bool CanPaintMultipleFragments(const LayoutObject&);

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::FlexColumnBreakInfo)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAGMENTATION_UTILS_H_
