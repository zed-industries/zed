// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_NODE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/grid/layout_grid.h"

namespace blink {

class GridItems;
class GridSizingSubtree;

// Grid specific extensions to `BlockNode`.
class CORE_EXPORT GridNode final : public BlockNode {
 public:
  explicit GridNode(LayoutBox* box) : BlockNode(box) {
    DCHECK(box);
    DCHECK(box->IsLayoutGrid());
  }

  const GridPlacementData& CachedPlacementData() const {
    return To<LayoutGrid>(box_.Get())->CachedPlacementData();
  }

  const GridLineResolver& CachedLineResolver() const {
    return CachedPlacementData().line_resolver;
  }

  void InvalidateSubgridMinMaxSizesCache() const {
    box_->SetSubgridMinMaxSizesCacheDirty(true);
  }

  bool ShouldInvalidateSubgridMinMaxSizesCacheFor(
      const GridLayoutData& layout_data) const {
    return To<LayoutGrid>(box_.Get())
        ->ShouldInvalidateSubgridMinMaxSizesCacheFor(layout_data);
  }

  // If `oof_children` is provided, aggregate any out of flow children.
  GridItems ConstructGridItems(const GridLineResolver& line_resolver,
                               bool* must_invalidate_placement_cache,
                               HeapVector<Member<LayoutBox>>* opt_oof_children,
                               bool* opt_has_nested_subgrid = nullptr) const;

  void AppendSubgriddedItems(GridItems* grid_items) const;

  MinMaxSizesResult ComputeSubgridMinMaxSizes(
      const GridSizingSubtree& sizing_subtree,
      const ConstraintSpace& space) const;

  LayoutUnit ComputeSubgridIntrinsicBlockSize(
      const GridSizingSubtree& sizing_subtree,
      const ConstraintSpace& space) const;

 private:
  GridItems ConstructGridItems(
      const GridLineResolver& line_resolver,
      const ComputedStyle& root_grid_style,
      const ComputedStyle& parent_grid_style,
      bool must_consider_grid_items_for_column_sizing,
      bool must_consider_grid_items_for_row_sizing,
      bool* must_invalidate_placement_cache,
      HeapVector<Member<LayoutBox>>* opt_oof_children = nullptr,
      bool* opt_has_nested_subgrid = nullptr) const;
};

template <>
struct DowncastTraits<GridNode> {
  static bool AllowFrom(const LayoutInputNode& node) { return node.IsGrid(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_NODE_H_
