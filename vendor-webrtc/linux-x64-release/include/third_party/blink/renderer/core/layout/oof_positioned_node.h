// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OOF_POSITIONED_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OOF_POSITIONED_NODE_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/geometry/static_position.h"
#include "third_party/blink/renderer/core/layout/layout_inline.h"
#include "third_party/blink/renderer/core/layout/physical_fragment.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

// If an out-of-flow positioned element is inside a fragmentation context, it
// will be laid out once it reaches the fragmentation context root rather than
// once it reaches its containing block. A containing block holds the
// containing block information needed to place these OOF positioned nodes once
// they reach the fragmentation context root. See
// PhysicalOofNodeForFragmentation/LogicalOofNodeForFragmentation for more
// details.
template <typename OffsetType>
class OofContainingBlock {
  DISALLOW_NEW();

 public:
  OofContainingBlock() = default;

  OofContainingBlock(OffsetType offset,
                     OffsetType relative_offset,
                     const PhysicalFragment* fragment,
                     std::optional<LayoutUnit> clipped_container_block_offset,
                     bool is_inside_column_spanner)
      : offset_(offset),
        relative_offset_(relative_offset),
        fragment_(std::move(fragment)),
        clipped_container_block_offset_(
            clipped_container_block_offset.value_or(LayoutUnit::Min())),
        is_inside_column_spanner_(is_inside_column_spanner) {}

  OffsetType Offset() const { return offset_; }
  void IncreaseBlockOffset(LayoutUnit block_offset) {
    offset_.block_offset += block_offset;
  }
  OffsetType RelativeOffset() const { return relative_offset_; }
  const PhysicalFragment* Fragment() const { return fragment_.Get(); }
  std::optional<LayoutUnit> ClippedContainerBlockOffset() const {
    if (clipped_container_block_offset_ == LayoutUnit::Min()) {
      return std::nullopt;
    }
    return clipped_container_block_offset_;
  }
  bool IsInsideColumnSpanner() const { return is_inside_column_spanner_; }

  // True if the containing block of an OOF is inside a clipped container inside
  // a fragmentation context.
  // For example: <multicol><clipped-overflow-container><relpos><abspos>
  bool IsFragmentedInsideClippedContainer() const {
    return clipped_container_block_offset_ != LayoutUnit::Min();
  }

  void Trace(Visitor* visitor) const { visitor->Trace(fragment_); }

 private:
  OffsetType offset_;
  // The relative offset is stored separately to ensure that it is applied after
  // fragmentation: https://www.w3.org/TR/css-break-3/#transforms.
  OffsetType relative_offset_;
  Member<const PhysicalFragment> fragment_;
  // The distance to the innermost container that clips block overflow, if any.
  LayoutUnit clipped_container_block_offset_ = LayoutUnit::Min();
  // True if there is a column spanner between the containing block and the
  // multicol container (or if the containing block is a column spanner).
  bool is_inside_column_spanner_ = false;
};

// This holds the containing block for an out-of-flow positioned element
// if the containing block is a non-atomic inline. It is the continuation
// root (i.e. the first LayoutInline in the continuation chain for the same
// node) if continuations are involved.
template <typename OffsetType>
struct OofInlineContainer {
  DISALLOW_NEW();

 public:
  Member<const LayoutInline> container;
  // Store the relative offset so that it can be applied after fragmentation,
  // if inside a fragmentation context.
  OffsetType relative_offset;

  OofInlineContainer() = default;
  OofInlineContainer(const LayoutInline* container, OffsetType relative_offset)
      : container(container), relative_offset(relative_offset) {}

  void Trace(Visitor* visitor) const { visitor->Trace(container); }
};

// If an out-of-flow positioned element is inside a nested fragmentation
// context, it will be laid out once it reaches the outermost fragmentation
// context root. A multicol with pending OOFs is the inner multicol information
// needed to perform layout on the OOF descendants once they make their way to
// the outermost context.
template <typename OffsetType>
struct MulticolWithPendingOofs
    : public GarbageCollected<MulticolWithPendingOofs<OffsetType>> {
 public:
  // If no fixedpos containing block was found, |multicol_offset| will be
  // relative to the outer fragmentation context root. Otherwise, it will be
  // relative to the fixedpos containing block.
  OffsetType multicol_offset;
  // If an OOF node in a nested fragmentation context has fixedpos descendants,
  // those descendants will not find their containing block if the containing
  // block lives inside an outer fragmentation context. Thus, we also need to
  // store information on the containing block and inline container for any
  // fixedpos descendants, if one exists.
  OofContainingBlock<OffsetType> fixedpos_containing_block;
  OofInlineContainer<OffsetType> fixedpos_inline_container;

  MulticolWithPendingOofs() = default;
  MulticolWithPendingOofs(
      OffsetType multicol_offset,
      OofContainingBlock<OffsetType> fixedpos_containing_block,
      OofInlineContainer<OffsetType> fixedpos_inline_container)
      : multicol_offset(multicol_offset),
        fixedpos_containing_block(fixedpos_containing_block),
        fixedpos_inline_container(fixedpos_inline_container) {}

  void Trace(Visitor* visitor) const {
    visitor->Trace(fixedpos_containing_block);
    visitor->Trace(fixedpos_inline_container);
  }
};

// A physical out-of-flow positioned-node is an element with the style
// "postion: absolute" or "position: fixed" which hasn't been bubbled up to its
// containing block yet, (e.g. an element with "position: relative"). As soon
// as a positioned-node reaches its containing block, it gets placed, and
// doesn't bubble further up the tree.
//
// This needs its static position [1] to be placed correctly in its containing
// block.
//
// This is struct is allowed to be stored/persisted.
//
// [1] https://www.w3.org/TR/CSS2/visudet.html#abs-non-replaced-width
struct CORE_EXPORT PhysicalOofPositionedNode {
  DISALLOW_NEW();

  using HorizontalEdge = PhysicalStaticPosition::HorizontalEdge;
  using VerticalEdge = PhysicalStaticPosition::VerticalEdge;
  using PhysicalAlignmentDirection =
      PhysicalStaticPosition::PhysicalAlignmentDirection;

 public:
  Member<LayoutBox> box;
  // Unpacked PhysicalStaticPosition.
  PhysicalOffset static_position;
  unsigned static_position_horizontal_edge : 2;
  unsigned static_position_vertical_edge : 2;
  unsigned static_position_align_self_direction : 1;
  // Whether or not this is an PhysicalOofNodeForFragmentation.
  unsigned is_for_fragmentation : 1;
  unsigned requires_content_before_breaking : 1;
  unsigned is_hidden_for_paint : 1;
  OofInlineContainer<PhysicalOffset> inline_container;

  PhysicalOofPositionedNode(
      BlockNode node,
      PhysicalStaticPosition static_position,
      bool requires_content_before_breaking,
      bool is_hidden_for_paint,
      OofInlineContainer<PhysicalOffset> inline_container = {})
      : box(node.GetLayoutBox()),
        static_position(static_position.offset),
        static_position_horizontal_edge(static_position.horizontal_edge),
        static_position_vertical_edge(static_position.vertical_edge),
        static_position_align_self_direction(
            static_position.align_self_direction),
        is_for_fragmentation(false),
        requires_content_before_breaking(requires_content_before_breaking),
        is_hidden_for_paint(is_hidden_for_paint),
        inline_container(inline_container) {
    DCHECK(node.IsBlock());
  }

  BlockNode Node() const { return BlockNode(box); }
  HorizontalEdge GetStaticPositionHorizontalEdge() const {
    return static_cast<HorizontalEdge>(static_position_horizontal_edge);
  }
  VerticalEdge GetStaticPositionVerticalEdge() const {
    return static_cast<VerticalEdge>(static_position_vertical_edge);
  }
  PhysicalAlignmentDirection GetStaticPositionAlignSelfDirection() const {
    return static_cast<PhysicalAlignmentDirection>(
        static_position_align_self_direction);
  }
  PhysicalStaticPosition StaticPosition() const {
    return {static_position, GetStaticPositionHorizontalEdge(),
            GetStaticPositionVerticalEdge(),
            GetStaticPositionAlignSelfDirection()};
  }

  void Trace(Visitor* visitor) const;
  void TraceAfterDispatch(Visitor*) const;
};

// The logical version of above. It is used within a an algorithm pass (within
// an |FragmentBuilder|), and its logical coordinate system is wrt.
// the container builder's writing-mode.
//
// It is *only* used within an algorithm pass, (it is temporary, and should not
// be stored/persisted).
struct CORE_EXPORT LogicalOofPositionedNode {
  DISALLOW_NEW();

 public:
  Member<LayoutBox> box;
  LogicalStaticPosition static_position;
  OofInlineContainer<LogicalOffset> inline_container;
  // Whether or not this is an LogicalOofNodeForFragmentation.
  unsigned is_for_fragmentation : 1;

  unsigned requires_content_before_breaking : 1;

  unsigned is_hidden_for_paint : 1;

  LogicalOofPositionedNode(
      BlockNode node,
      LogicalStaticPosition static_position,
      bool requires_content_before_breaking,
      bool is_hidden_for_paint,
      OofInlineContainer<LogicalOffset> inline_container = {})
      : box(node.GetLayoutBox()),
        static_position(static_position),
        inline_container(inline_container),
        is_for_fragmentation(false),
        requires_content_before_breaking(requires_content_before_breaking),
        is_hidden_for_paint(is_hidden_for_paint) {
    DCHECK(node.IsBlock());
  }

  BlockNode Node() const { return BlockNode(box); }

  void Trace(Visitor* visitor) const;
  void TraceAfterDispatch(Visitor*) const;
};

// When fragmentation comes into play, we no longer place a positioned-node as
// soon as it reaches its containing block. Instead, we continue to bubble the
// positioned node up until it reaches the fragmentation context root. There, it
// will get placed and properly fragmented.
//
// In addition to the static position, we also needs the containing block
// fragment to be placed correctly within the fragmentation context root. In
// addition, the containing block offset is needed to compute the start offset
// and the initial fragmentainer of an out-of-flow positioned-node.
//
// If an OOF node in a fragmentation context has fixedpos descendants, those
// descendants will not find their containing block if the containing block
// lives inside the fragmentation context root. Thus, we also need to store
// information on the containing block and inline container for any fixedpos
// descendants, if one exists.
//
// This is struct is allowed to be stored/persisted.
struct CORE_EXPORT PhysicalOofNodeForFragmentation final
    : public PhysicalOofPositionedNode {
  DISALLOW_NEW();

 public:
  OofContainingBlock<PhysicalOffset> containing_block;
  OofContainingBlock<PhysicalOffset> fixedpos_containing_block;
  OofInlineContainer<PhysicalOffset> fixedpos_inline_container;

  PhysicalOofNodeForFragmentation(
      BlockNode node,
      PhysicalStaticPosition static_position,
      bool requires_content_before_breaking,
      bool is_hidden_for_paint,
      OofInlineContainer<PhysicalOffset> inline_container = {},
      OofContainingBlock<PhysicalOffset> containing_block = {},
      OofContainingBlock<PhysicalOffset> fixedpos_containing_block = {},
      OofInlineContainer<PhysicalOffset> fixedpos_inline_container = {})
      : PhysicalOofPositionedNode(node,
                                  static_position,
                                  requires_content_before_breaking,
                                  is_hidden_for_paint,
                                  inline_container),
        containing_block(containing_block),
        fixedpos_containing_block(fixedpos_containing_block),
        fixedpos_inline_container(fixedpos_inline_container) {
    is_for_fragmentation = true;
  }

  void TraceAfterDispatch(Visitor* visitor) const;
};

// The logical version of the above. It is used within a an algorithm pass
// (within an |FragmentBuilder|), and its logical coordinate system
// is wrt. the container builder's writing-mode.
//
// It is *only* used within an algorithm pass, (it is temporary, and should not
// be stored/persisted).
struct CORE_EXPORT LogicalOofNodeForFragmentation final
    : public LogicalOofPositionedNode {
  DISALLOW_NEW();

 public:
  OofContainingBlock<LogicalOffset> containing_block;
  OofContainingBlock<LogicalOffset> fixedpos_containing_block;
  OofInlineContainer<LogicalOffset> fixedpos_inline_container;

  LogicalOofNodeForFragmentation(
      BlockNode node,
      LogicalStaticPosition static_position,
      bool requires_content_before_breaking,
      bool is_hidden_for_paint,
      OofInlineContainer<LogicalOffset> inline_container = {},
      OofContainingBlock<LogicalOffset> containing_block = {},
      OofContainingBlock<LogicalOffset> fixedpos_containing_block = {},
      OofInlineContainer<LogicalOffset> fixedpos_inline_container = {})
      : LogicalOofPositionedNode(node,
                                 static_position,
                                 requires_content_before_breaking,
                                 is_hidden_for_paint,
                                 inline_container),
        containing_block(containing_block),
        fixedpos_containing_block(fixedpos_containing_block),
        fixedpos_inline_container(fixedpos_inline_container) {
    is_for_fragmentation = true;
  }

  explicit LogicalOofNodeForFragmentation(
      const LogicalOofPositionedNode& oof_node)
      : LogicalOofPositionedNode(oof_node.Node(),
                                 oof_node.static_position,
                                 oof_node.requires_content_before_breaking,
                                 oof_node.is_hidden_for_paint,
                                 oof_node.inline_container) {
    is_for_fragmentation = true;
  }

  const LayoutObject* CssContainingBlock() const { return box->Container(); }

  void TraceAfterDispatch(Visitor* visitor) const;
};

template <>
struct DowncastTraits<LogicalOofNodeForFragmentation> {
  static bool AllowFrom(const LogicalOofPositionedNode& oof_node) {
    return oof_node.is_for_fragmentation;
  }
};

// This is a sub class of |PhysicalFragment::OofData| that can store OOF
// propagation data under the NG block fragmentation context.
//
// This class is defined here instead of |PhysicalFragment| because types
// needed for this class requires full definition of |PhysicalFragment|, and
// |PhysicalFragment| requires full definition of this class if this is put
// into |PhysicalFragment|.
struct FragmentedOofData final : PhysicalFragment::OofData {
  using MulticolCollection =
      HeapHashMap<Member<LayoutBox>,
                  Member<MulticolWithPendingOofs<PhysicalOffset>>>;

  static bool HasOutOfFlowPositionedFragmentainerDescendants(
      const PhysicalFragment& fragment) {
    const auto* oof_data = fragment.GetFragmentedOofData();
    return oof_data &&
           !oof_data->oof_positioned_fragmentainer_descendants.empty();
  }

  bool NeedsOOFPositionedInfoPropagation() const {
    return !oof_positioned_fragmentainer_descendants.empty() ||
           !multicols_with_pending_oofs.empty();
  }

  static base::span<PhysicalOofNodeForFragmentation>
  OutOfFlowPositionedFragmentainerDescendants(
      const PhysicalFragment& fragment) {
    const auto* oof_data = fragment.GetFragmentedOofData();
    if (!oof_data || oof_data->oof_positioned_fragmentainer_descendants.empty())
      return base::span<PhysicalOofNodeForFragmentation>();
    HeapVector<PhysicalOofNodeForFragmentation>& descendants =
        const_cast<HeapVector<PhysicalOofNodeForFragmentation>&>(
            oof_data->oof_positioned_fragmentainer_descendants);
    return descendants;
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(oof_positioned_fragmentainer_descendants);
    visitor->Trace(multicols_with_pending_oofs);
    PhysicalFragment::OofData::Trace(visitor);
  }

  HeapVector<PhysicalOofNodeForFragmentation>
      oof_positioned_fragmentainer_descendants;
  MulticolCollection multicols_with_pending_oofs;
};

inline PhysicalOffset RelativeInsetToPhysical(
    LogicalOffset relative_inset,
    WritingDirectionMode writing_direction) {
  return relative_inset.ConvertToPhysical(writing_direction, PhysicalSize(),
                                          PhysicalSize());
}

inline LogicalOffset RelativeInsetToLogical(
    PhysicalOffset relative_inset,
    WritingDirectionMode writing_direction) {
  return WritingModeConverter(writing_direction, PhysicalSize())
      .ToLogical(relative_inset, PhysicalSize());
}

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::PhysicalOofPositionedNode)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::PhysicalOofNodeForFragmentation)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::LogicalOofPositionedNode)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::LogicalOofNodeForFragmentation)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OOF_POSITIONED_NODE_H_
