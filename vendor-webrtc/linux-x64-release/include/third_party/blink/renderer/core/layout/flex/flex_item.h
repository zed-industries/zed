// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_ITEM_H_

#include "third_party/blink/renderer/core/layout/baseline_utils.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/layout_result.h"
#include "third_party/blink/renderer/core/layout/min_max_sizes.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

struct FlexItem {
  DISALLOW_NEW();

 public:
  FlexItem(BlockNode block_node,
           wtf_size_t item_index,
           float flex_grow,
           float flex_shrink,
           LayoutUnit base_content_size,
           MinMaxSizes main_axis_min_max_sizes,
           LayoutUnit main_axis_border_padding,
           std::optional<LayoutUnit> max_content_contribution,
           PhysicalBoxStrut initial_margins,
           BoxStrut initial_scrollbars,
           uint8_t main_axis_auto_margin_count,
           ItemPosition alignment,
           WritingMode baseline_writing_mode,
           BaselineGroup baseline_group,
           bool is_initial_block_size_indefinite,
           bool is_used_flex_basis_indefinite,
           bool depends_on_min_max_sizes,
           bool is_horizontal_flow)
      : block_node(block_node),
        item_index(item_index),
        flex_grow(flex_grow),
        flex_shrink(flex_shrink),
        base_content_size(base_content_size),
        hypothetical_content_size(
            main_axis_min_max_sizes.ClampSizeToMinAndMax(base_content_size)),
        main_axis_min_max_sizes(main_axis_min_max_sizes),
        main_axis_border_padding(main_axis_border_padding),
        max_content_contribution(max_content_contribution),
        initial_margins(initial_margins),
        initial_scrollbars(initial_scrollbars),
        main_axis_auto_margin_count(main_axis_auto_margin_count),
        alignment(alignment),
        baseline_writing_direction(
            {baseline_writing_mode, TextDirection::kLtr}),
        baseline_group(baseline_group),
        is_initial_block_size_indefinite(is_initial_block_size_indefinite),
        is_used_flex_basis_indefinite(is_used_flex_basis_indefinite),
        depends_on_min_max_sizes(depends_on_min_max_sizes),
        is_horizontal_flow(is_horizontal_flow) {}

  LayoutUnit HypotheticalMainAxisMarginBoxSize() const {
    return hypothetical_content_size + main_axis_border_padding +
           MainAxisMarginExtent();
  }

  LayoutUnit FlexBaseMarginBoxSize() const {
    return base_content_size + main_axis_border_padding +
           MainAxisMarginExtent();
  }

  LayoutUnit FlexedBorderBoxSize() const {
    return flexed_content_size + main_axis_border_padding;
  }

  LayoutUnit FlexedMarginBoxSize() const {
    return flexed_content_size + main_axis_border_padding +
           MainAxisMarginExtent();
  }

  LayoutUnit MainAxisMarginExtent() const {
    return is_horizontal_flow ? initial_margins.HorizontalSum()
                              : initial_margins.VerticalSum();
  }
  LayoutUnit CrossAxisMarginExtent() const {
    return is_horizontal_flow ? initial_margins.VerticalSum()
                              : initial_margins.HorizontalSum();
  }

  void Trace(Visitor* visitor) const {
    visitor->Trace(block_node);
    visitor->Trace(layout_result);
  }

  const BlockNode block_node;
  const wtf_size_t item_index;

  const float flex_grow;
  const float flex_shrink;

  // `base_content_size` include the scrollbar, but not border/padding.
  const LayoutUnit base_content_size;
  const LayoutUnit hypothetical_content_size;

  // `main_axis_min_max_sizes` is the resolved min/max size properties, and also
  // does not include border/padding.
  const MinMaxSizes main_axis_min_max_sizes;
  const LayoutUnit main_axis_border_padding;

  const std::optional<LayoutUnit> max_content_contribution;

  // `initial_margins` are the margins with auto-margins applied.
  // `initial_scrollbars` is the scrollbar state at the beginning of running
  // flex layout, so it can be compared to the final state.
  const PhysicalBoxStrut initial_margins;
  const BoxStrut initial_scrollbars;

  const uint8_t main_axis_auto_margin_count;

  const ItemPosition alignment;
  const WritingDirectionMode baseline_writing_direction;
  const BaselineGroup baseline_group;

  const bool is_initial_block_size_indefinite;
  const bool is_used_flex_basis_indefinite;
  const bool depends_on_min_max_sizes;
  const bool is_horizontal_flow;

  // Fields mutated within the line-flexer.
  bool frozen = false;
  LayoutUnit flexed_content_size;

  // The above fields are used by the flex algorithm. The following fields, by
  // contrast, are just convenient storage.
  Member<const LayoutResult> layout_result;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::FlexItem)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_ITEM_H_
