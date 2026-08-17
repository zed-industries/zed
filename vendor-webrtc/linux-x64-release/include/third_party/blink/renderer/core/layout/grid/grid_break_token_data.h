// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_BREAK_TOKEN_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_BREAK_TOKEN_DATA_H_

#include "third_party/blink/renderer/core/layout/block_break_token_data.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

struct GridItemPlacementData {
  GridItemPlacementData(
      LogicalOffset offset,
      LogicalOffset relative_offset,
      bool has_descendant_that_depends_on_percentage_block_size)
      : offset(offset),
        relative_offset(relative_offset),
        has_descendant_that_depends_on_percentage_block_size(
            has_descendant_that_depends_on_percentage_block_size) {}

  LogicalOffset offset;
  LogicalOffset relative_offset;
  bool has_descendant_that_depends_on_percentage_block_size;
};

struct GridBreakTokenData final : BlockBreakTokenData {
  GridBreakTokenData(
      const BlockBreakTokenData* break_token_data,
      GridItems&& grid_items,
      GridLayoutSubtree grid_layout_subtree,
      LayoutUnit intrinsic_block_size,
      LayoutUnit offset_in_stitched_container,
      const Vector<GridItemPlacementData>& grid_items_placement_data,
      const Vector<LayoutUnit>& row_offset_adjustments,
      const Vector<EBreakBetween>& row_break_between,
      const HeapVector<Member<LayoutBox>>& oof_children)
      : BlockBreakTokenData(kGridBreakTokenData, break_token_data),
        grid_items(std::move(grid_items)),
        grid_layout_subtree(std::move(grid_layout_subtree)),
        intrinsic_block_size(intrinsic_block_size),
        offset_in_stitched_container(offset_in_stitched_container),
        grid_items_placement_data(grid_items_placement_data),
        row_offset_adjustments(row_offset_adjustments),
        row_break_between(row_break_between),
        oof_children(oof_children) {}

  void Trace(Visitor* visitor) const override {
    visitor->Trace(grid_items);
    visitor->Trace(oof_children);
    BlockBreakTokenData::Trace(visitor);
  }

  GridItems grid_items;
  GridLayoutSubtree grid_layout_subtree;
  LayoutUnit intrinsic_block_size;

  // This is similar to |BlockBreakTokenData::consumed_block_size|, however
  // it isn't used for determining the final block-size of the fragment and
  // won't include any block-end padding (this prevents saturation bugs).
  // It also won't include any cloned box decorations.
  LayoutUnit offset_in_stitched_container;

  Vector<GridItemPlacementData> grid_items_placement_data;
  Vector<LayoutUnit> row_offset_adjustments;
  Vector<EBreakBetween> row_break_between;
  HeapVector<Member<LayoutBox>> oof_children;
};

template <>
struct DowncastTraits<GridBreakTokenData> {
  static bool AllowFrom(const BlockBreakTokenData& token_data) {
    return token_data.IsGridType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_BREAK_TOKEN_DATA_H_
