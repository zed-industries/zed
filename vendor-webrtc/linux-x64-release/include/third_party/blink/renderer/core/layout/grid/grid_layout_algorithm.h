// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_break_token.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/grid/grid_node.h"
#include "third_party/blink/renderer/core/layout/grid/grid_sizing_tree.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

enum class GridItemContributionType;
enum class SizingConstraint;
class ConstraintSpace;
struct GridItemPlacementData;

class CORE_EXPORT GridLayoutAlgorithm
    : public LayoutAlgorithm<GridNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  explicit GridLayoutAlgorithm(const LayoutAlgorithmParams& params);

  const LayoutResult* Layout();
  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&);

  LayoutUnit ComputeSubgridIntrinsicBlockSize(
      const GridSizingSubtree& sizing_subtree) const;
  MinMaxSizes ComputeSubgridMinMaxSizes(
      const GridSizingSubtree& sizing_subtree) const;

  // Computes the containing block rect of out of flow items from stored data in
  // |GridLayoutData|.
  static LogicalRect ComputeOutOfFlowItemContainingRect(
      const GridPlacementData& placement_data,
      const GridLayoutData& layout_data,
      const ComputedStyle& grid_style,
      const BoxStrut& borders,
      const LogicalSize& border_box_size,
      GridItemData* out_of_flow_item);

  // Helper that computes tracks sizes in a given range.
  static Vector<std::div_t> ComputeTrackSizesInRange(
      const GridLayoutTrackCollection& track_collection,
      wtf_size_t range_starting_set_index,
      wtf_size_t range_set_count);

  const GapGeometry* GetGapGeometryForTest() {
    return container_builder_.GetGapGeometryForTest();
  }

 private:
  friend class GridLayoutAlgorithmTest;

  // Aggregate all direct out of flow children from the current grid container
  // to `opt_oof_children`, unless it's not provided.
  void BuildGridSizingSubtree(
      GridSizingTree* sizing_tree,
      HeapVector<Member<LayoutBox>>* opt_oof_children,
      const SubgriddedItemData& opt_subgrid_data = kNoSubgriddedItemData,
      const GridLineResolver* opt_parent_line_resolver = nullptr,
      bool must_invalidate_placement_cache = false,
      bool must_ignore_children = false) const;

  GridSizingTree BuildGridSizingTree(
      HeapVector<Member<LayoutBox>>* opt_oof_children = nullptr) const;
  GridSizingTree BuildGridSizingTreeIgnoringChildren() const;

  const LayoutResult* LayoutInternal();

  GridLayoutSubtree ComputeGridGeometry(
      GridItems* grid_items,
      LayoutUnit* intrinsic_block_size,
      HeapVector<Member<LayoutBox>>* oof_children);

  LayoutUnit ComputeIntrinsicBlockSizeIgnoringChildren() const;

  // Returns the size that a grid item will distribute across the tracks with an
  // intrinsic sizing function it spans in the relevant track direction.
  LayoutUnit ContributionSizeForGridItem(
      const GridSizingSubtree& sizing_subtree,
      GridItemContributionType contribution_type,
      GridTrackSizingDirection track_direction,
      SizingConstraint sizing_constraint,
      GridItemData* grid_item) const;

  wtf_size_t ComputeAutomaticRepetitions(
      const GridSpan& subgrid_span,
      GridTrackSizingDirection track_direction) const;

  // Subgrids compute auto repetitions differently than standalone grids.
  wtf_size_t ComputeAutomaticRepetitionsForSubgrid(
      wtf_size_t subgrid_span_size,
      GridTrackSizingDirection track_direction) const;

  // Determines the major/minor alignment baselines for each row/column based on
  // each item in `grid_items`, and stores the results in `track_collection`.
  void ComputeGridItemBaselines(const GridLayoutTreePtr& layout_tree,
                                const GridSizingSubtree& sizing_subtree,
                                GridTrackSizingDirection track_direction,
                                SizingConstraint sizing_constraint) const;

  std::unique_ptr<GridLayoutTrackCollection> CreateSubgridTrackCollection(
      const SubgriddedItemData& subgrid_data,
      GridTrackSizingDirection track_direction) const;

  // Initialize the track collections of a given grid sizing data.
  void InitializeTrackCollection(const SubgriddedItemData& opt_subgrid_data,
                                 GridTrackSizingDirection track_direction,
                                 GridLayoutData* layout_data) const;

  // Initializes the track sizes of a grid sizing subtree.
  void InitializeTrackSizes(
      const GridSizingSubtree& sizing_subtree,
      const SubgriddedItemData& opt_subgrid_data,
      const std::optional<GridTrackSizingDirection>& opt_track_direction) const;

  // Helper that calls the method above for the entire grid sizing tree.
  void InitializeTrackSizes(GridSizingTree* sizing_tree,
                            const std::optional<GridTrackSizingDirection>&
                                opt_track_direction = std::nullopt) const;

  // Helper that instances a `GridTrackSizingAlgorithm` with a wrapper for
  // `ContributionSizeForGridItem` to compute the used track sizes.
  void ComputeUsedTrackSizes(const GridSizingSubtree& sizing_subtree,
                             GridTrackSizingDirection track_direction,
                             SizingConstraint sizing_constraint) const;

  // Computes and caches the used track sizes of a grid sizing subtree.
  void CompleteTrackSizingAlgorithm(const GridSizingSubtree& sizing_subtree,
                                    const SubgriddedItemData& opt_subgrid_data,
                                    GridTrackSizingDirection track_direction,
                                    SizingConstraint sizing_constraint,
                                    bool* opt_needs_additional_pass) const;

  // Helper that calls the method above for the entire grid sizing tree.
  void CompleteTrackSizingAlgorithm(
      GridTrackSizingDirection track_direction,
      SizingConstraint sizing_constraint,
      GridSizingTree* sizing_tree,
      bool* opt_needs_additional_pass = nullptr) const;

  // Performs the final baseline alignment pass of a grid sizing subtree.
  void ComputeBaselineAlignment(
      const GridLayoutTreePtr& layout_tree,
      const GridSizingSubtree& sizing_subtree,
      const SubgriddedItemData& opt_subgrid_data,
      const std::optional<GridTrackSizingDirection>& opt_track_direction,
      SizingConstraint sizing_constraint) const;

  // Helper that calls the method above for the entire grid sizing tree.
  void CompleteFinalBaselineAlignment(GridSizingTree* sizing_tree) const;

  // Helper which iterates over the sizing tree, and instantiates a subgrid
  // algorithm to invoke the callback with.
  template <typename CallbackFunc>
  void ForEachSubgrid(const GridSizingSubtree& sizing_subtree,
                      const CallbackFunc& callback_func,
                      bool should_compute_min_max_sizes = true) const;

  LayoutUnit ComputeSubgridIntrinsicSize(
      const GridSizingSubtree& sizing_subtree,
      GridTrackSizingDirection track_direction,
      SizingConstraint sizing_constraint) const;

  ConstraintSpace CreateConstraintSpace(
      LayoutResultCacheSlot cache_slot,
      const GridItemData& grid_item,
      const LogicalSize& containing_grid_area_size,
      const LogicalSize& fixed_available_size,
      GridLayoutSubtree&& opt_layout_subtree = GridLayoutSubtree(),
      bool min_block_size_should_encompass_intrinsic_size = false,
      std::optional<LayoutUnit> opt_child_block_offset = std::nullopt) const;

  // `containing_grid_area` is an optional out parameter that holds the computed
  // grid area (offset and size) of the specified grid item.
  ConstraintSpace CreateConstraintSpaceForLayout(
      const GridItemData& grid_item,
      const GridLayoutData& layout_data,
      GridLayoutSubtree&& opt_layout_subtree = GridLayoutSubtree(),
      LogicalRect* containing_grid_area = nullptr,
      LayoutUnit unavailable_block_size = LayoutUnit(),
      bool min_block_size_should_encompass_intrinsic_size = false,
      std::optional<LayoutUnit> opt_child_block_offset = std::nullopt) const;

  ConstraintSpace CreateConstraintSpaceForMeasure(
      const SubgriddedItemData& subgridded_item,
      GridTrackSizingDirection track_direction,
      std::optional<LayoutUnit> opt_fixed_inline_size = std::nullopt) const;

  // Layout the |grid_items|, and add them to the builder.
  //
  // If |out_grid_items_placement_data| is present determine the offset for
  // each of the |grid_items| but *don't* add the resulting fragment to the
  // builder.
  //
  // This is used for fragmentation which requires us to know the final offset
  // of each item before fragmentation occurs.
  void PlaceGridItems(
      const GridItems& grid_items,
      const GridLayoutSubtree& layout_subtree,
      Vector<EBreakBetween>* out_row_break_between,
      Vector<GridItemPlacementData>* out_grid_items_placement_data = nullptr);

  // Layout the |grid_items| for fragmentation (when there is a known
  // fragmentainer size).
  //
  // This will go through all the grid_items and place fragments which belong
  // within this fragmentainer.
  void PlaceGridItemsForFragmentation(
      const GridItems& grid_items,
      const GridLayoutSubtree& layout_subtree,
      const Vector<EBreakBetween>& row_break_between,
      Vector<GridItemPlacementData>* grid_item_placement_data,
      Vector<LayoutUnit>* row_offset_adjustments,
      LayoutUnit* intrinsic_block_size,
      LayoutUnit* offset_in_stitched_container);

  // Computes the static position, grid area and its offset of out of flow
  // elements in the grid (as provided by `oof_children`).
  void PlaceOutOfFlowItems(const GridLayoutData& layout_data,
                           const LayoutUnit block_size,
                           HeapVector<Member<LayoutBox>>& oof_children);

  // Set reading flow nodes so they can be accessed by `LayoutBox`.
  void SetReadingFlowNodes(const GridItems& grid_items);

  LogicalSize grid_available_size_;
  LogicalSize grid_min_available_size_;
  LogicalSize grid_max_available_size_;

  std::optional<LayoutUnit> contain_intrinsic_block_size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_LAYOUT_ALGORITHM_H_
