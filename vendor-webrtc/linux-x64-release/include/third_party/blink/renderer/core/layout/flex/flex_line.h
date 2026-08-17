// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_LINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_LINE_H_

#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// This contains a limited subset of the information from `FlexItem` that we
// need during fragmentation. This is populated on the FlexLine (which are
// stored on the break-token), during `FlexLayoutAlgorithm::PlaceFlexItems`.
struct FlexItemData {
  DISALLOW_NEW();

 public:
  FlexItemData(BlockNode block_node,
               wtf_size_t item_index,
               LogicalOffset offset,
               ItemPosition alignment,
               LayoutUnit main_axis_final_size,
               LayoutUnit margin_block_end,
               LayoutUnit total_remaining_block_size,
               bool is_initial_block_size_indefinite,
               bool is_used_flex_basis_indefinite,
               bool has_descendant_that_depends_on_percentage_block_size)
      : block_node(block_node),
        item_index(item_index),
        offset(offset),
        alignment(alignment),
        main_axis_final_size(main_axis_final_size),
        margin_block_end(margin_block_end),
        total_remaining_block_size(total_remaining_block_size),
        is_initial_block_size_indefinite(is_initial_block_size_indefinite),
        is_used_flex_basis_indefinite(is_used_flex_basis_indefinite),
        has_descendant_that_depends_on_percentage_block_size(
            has_descendant_that_depends_on_percentage_block_size) {}

  const ComputedStyle& Style() const { return block_node.Style(); }

  void Trace(Visitor* visitor) const { visitor->Trace(block_node); }

  BlockNode block_node;
  wtf_size_t item_index;
  LogicalOffset offset;
  ItemPosition alignment;
  LayoutUnit main_axis_final_size;
  LayoutUnit margin_block_end;
  // This will originally be set to the total block size of the item before
  // fragmentation. It will then be reduced while performing fragmentation. If
  // it becomes negative, that means that the item expanded as a result of
  // fragmentation. This is only used for column flex containers.
  LayoutUnit total_remaining_block_size;
  bool is_initial_block_size_indefinite = false;
  bool is_used_flex_basis_indefinite = false;
  bool has_descendant_that_depends_on_percentage_block_size = false;
};

// A flex-line post running the line-flexer. This is potentially stored on the
// break-token if fragmented.
struct FlexLine {
  DISALLOW_NEW();

 public:
  FlexLine(Vector<wtf_size_t> item_indices,
           LayoutUnit main_axis_free_space,
           LayoutUnit line_cross_size,
           LayoutUnit major_baseline,
           LayoutUnit minor_baseline,
           unsigned main_axis_auto_margin_count)
      : item_indices(std::move(item_indices)),
        main_axis_free_space(main_axis_free_space),
        line_cross_size(line_cross_size),
        major_baseline(major_baseline),
        minor_baseline(minor_baseline),
        main_axis_auto_margin_count(main_axis_auto_margin_count) {}

  LayoutUnit LineCrossEnd() const {
    return line_cross_size + cross_axis_offset + item_offset_adjustment;
  }

  void Trace(Visitor* visitor) const { visitor->Trace(line_items_data); }

  Vector<wtf_size_t> item_indices;
  LayoutUnit main_axis_free_space;
  LayoutUnit line_cross_size;
  LayoutUnit major_baseline;
  LayoutUnit minor_baseline;
  unsigned main_axis_auto_margin_count;

  LayoutUnit cross_axis_offset;

  // These fields are only used/populated during fragmentation.
  LayoutUnit item_offset_adjustment;
  bool has_seen_all_children = false;
  HeapVector<FlexItemData> line_items_data;
};

// Flex-layout usually has exactly one line.
using FlexLineVector = HeapVector<FlexLine, 1>;

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::FlexItemData)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::FlexLine)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_LINE_H_
