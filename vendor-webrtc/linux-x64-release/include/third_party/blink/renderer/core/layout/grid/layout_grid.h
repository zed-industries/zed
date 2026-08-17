// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_LAYOUT_GRID_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_LAYOUT_GRID_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/grid/grid_data.h"
#include "third_party/blink/renderer/core/layout/grid/subgrid_min_max_sizes_cache.h"
#include "third_party/blink/renderer/core/layout/layout_block.h"

namespace blink {

class CORE_EXPORT LayoutGrid : public LayoutBlock {
 public:
  explicit LayoutGrid(Element* element);

  const char* GetName() const override {
    NOT_DESTROYED();
    // This string can affect a production behavior.
    // See tool_highlight.ts in devtools-frontend.
    return "LayoutGrid";
  }

  // Helper functions to help with getting expanded positions when needed.
  // These helpers are currently used for DevTools, ComputedStyles and Gap
  // Decorations.
  static Vector<LayoutUnit> ComputeTrackSizeRepeaterForRange(
      const GridLayoutTrackCollection& track_collection,
      wtf_size_t range_index);
  static Vector<LayoutUnit> ComputeExpandedPositions(
      const GridLayoutData* grid_layout_data,
      GridTrackSizingDirection track_direction);

  bool HasCachedPlacementData() const;
  const GridPlacementData& CachedPlacementData() const;
  void SetCachedPlacementData(GridPlacementData&& placement_data);

  bool HasCachedSubgridMinMaxSizes() const;
  const MinMaxSizes& CachedSubgridMinMaxSizes() const;
  void SetSubgridMinMaxSizesCache(MinMaxSizes&& min_max_sizes,
                                  const GridLayoutData& layout_data);
  bool ShouldInvalidateSubgridMinMaxSizesCacheFor(
      const GridLayoutData& layout_data) const;

  wtf_size_t AutoRepeatCountForDirection(
      GridTrackSizingDirection track_direction) const;
  wtf_size_t ExplicitGridStartForDirection(
      GridTrackSizingDirection track_direction) const;
  wtf_size_t ExplicitGridEndForDirection(
      GridTrackSizingDirection track_direction) const;
  LayoutUnit GridGap(GridTrackSizingDirection track_direction) const;
  LayoutUnit GridItemOffset(GridTrackSizingDirection track_direction) const;
  Vector<LayoutUnit, 1> TrackSizesForComputedStyle(
      GridTrackSizingDirection track_direction) const;

  Vector<LayoutUnit> RowPositions() const;
  Vector<LayoutUnit> ColumnPositions() const;

  const GridLayoutData* LayoutData() const;

 private:
  bool IsLayoutGrid() const final {
    NOT_DESTROYED();
    return true;
  }

  void AddChild(LayoutObject* new_child, LayoutObject* before_child) override;
  void RemoveChild(LayoutObject* child) override;
  void StyleDidChange(StyleDifference diff,
                      const ComputedStyle* old_style) override;

  std::optional<GridPlacementData> cached_placement_data_;
  std::optional<const SubgridMinMaxSizesCache> cached_subgrid_min_max_sizes_;
};

// wtf/casting.h helper.
template <>
struct DowncastTraits<LayoutGrid> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutGrid();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_LAYOUT_GRID_H_
