// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SCROLLABLE_OVERFLOW_CALCULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SCROLLABLE_OVERFLOW_CALCULATOR_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_rect.h"
#include "third_party/blink/renderer/core/layout/inline/fragment_items_builder.h"
#include "third_party/blink/renderer/core/layout/physical_box_fragment.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This class contains the logic for correctly determining the
// scrollable-overflow (also known as layout-overflow) for a fragment.
// https://drafts.csswg.org/css-overflow-3/#scrollable
class CORE_EXPORT ScrollableOverflowCalculator {
  STACK_ALLOCATED();

 public:
  static PhysicalRect RecalculateScrollableOverflowForFragment(
      const PhysicalBoxFragment&,
      bool has_block_fragmentation);

  ScrollableOverflowCalculator(const BlockNode&,
                               bool is_css_box,
                               bool has_block_fragmentation,
                               const PhysicalBoxStrut& borders,
                               const PhysicalBoxStrut& scrollbar,
                               const PhysicalBoxStrut& padding,
                               PhysicalSize size,
                               WritingDirectionMode);

  // Applies the final adjustments given the bounds of any inflow children
  // (|inflow_bounds|), and returns the final scrollable-overflow.
  const PhysicalRect Result(const std::optional<PhysicalRect> inflow_bounds);

  // Adds scrollable-overflow from |child_fragment|, at |offset|.
  void AddChild(const PhysicalBoxFragment& child_fragment,
                PhysicalOffset offset) {
    if (is_view_ && child_fragment.IsFixedPositioned())
      return;
    PhysicalRect child_overflow =
        ScrollableOverflowForPropagation(child_fragment);
    child_overflow.offset += offset;
    AddOverflow(child_overflow, child_fragment.IsFragmentainerBox());
  }

  // Adds scrollable-overflow from fragment-items.
  void AddItems(const PhysicalBoxFragment&, const FragmentItems&);
  void AddItems(const LayoutObject*,
                const FragmentItemsBuilder::ItemWithOffsetList&);

  void AddTableSelfRect();

 private:
  template <typename Items>
  void AddItemsInternal(const LayoutObject* layout_object, const Items& items);

  PhysicalRect AdjustOverflowForHanging(const PhysicalRect& line_box_rect,
                                        PhysicalRect overflow);
  PhysicalRect AdjustOverflowForScrollOrigin(const PhysicalRect& overflow);

  PhysicalRect ScrollableOverflowForPropagation(
      const PhysicalBoxFragment& child_fragment);

  void AddOverflow(PhysicalRect child_overflow,
                   bool child_is_fragmentainer = false) {
    if (is_scroll_container_)
      child_overflow = AdjustOverflowForScrollOrigin(child_overflow);

    // A fragmentainer may result in an overflow, even if it is empty. For
    // example, an overflow as a result of a non-zero column gap.
    if (!child_overflow.IsEmpty() || child_is_fragmentainer)
      scrollable_overflow_.UniteEvenIfEmpty(child_overflow);
  }

  const BlockNode node_;
  const WritingDirectionMode writing_direction_;
  const bool is_scroll_container_;
  const bool is_view_;
  const bool has_left_overflow_;
  const bool has_top_overflow_;
  const bool has_non_visible_overflow_;
  const bool has_block_fragmentation_;

  const PhysicalBoxStrut padding_;
  const PhysicalSize size_;

  PhysicalRect padding_rect_;
  PhysicalRect scrollable_overflow_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SCROLLABLE_OVERFLOW_CALCULATOR_H_
