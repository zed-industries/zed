// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be found
// in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_TRACK_SIZING_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_TRACK_SIZING_ALGORITHM_H_

#include "base/containers/span.h"
#include "base/functional/function_ref.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/core/style/grid_enums.h"

namespace blink {

class GridItems;
class GridLayoutTrackCollection;
class GridSizingTrackCollection;
struct BoxStrut;
struct GridItemData;

// This enum corresponds to each step used to accommodate grid items across
// intrinsic tracks according to their min and max track sizing functions, as
// defined in https://drafts.csswg.org/css-grid-2/#algo-spanning-items.
enum class GridItemContributionType {
  kForIntrinsicMinimums,
  kForContentBasedMinimums,
  kForMaxContentMinimums,
  kForIntrinsicMaximums,
  kForMaxContentMaximums,
  kForFreeSpace
};

enum class SizingConstraint { kLayout, kMaxContent, kMinContent };

using ContributionSizeFunctionRef =
    base::FunctionRef<LayoutUnit(GridItemContributionType, GridItemData*)>;

class GridTrackSizingAlgorithm {
  STACK_ALLOCATED();

 public:
  struct FirstSetGeometry {
    LayoutUnit gutter_size;
    LayoutUnit start_offset;
  };

  GridTrackSizingAlgorithm(const ComputedStyle& container_style,
                           const LogicalSize& container_available_size,
                           const LogicalSize& container_min_available_size,
                           SizingConstraint sizing_constraint)
      : available_size_(container_available_size),
        min_available_size_(container_min_available_size),
        sizing_constraint_(sizing_constraint),
        columns_alignment_(container_style.JustifyContent()),
        rows_alignment_(container_style.AlignContent()) {}

  // Caches the track span properties necessary for the track sizing algorithm
  // to work based on the grid items' placement within the track collection.
  static void CacheGridItemsProperties(
      const GridLayoutTrackCollection& track_collection,
      GridItems* grid_items);

  // Calculates the specified `[column|row]-gap` of the container.
  static LayoutUnit CalculateGutterSize(
      const ComputedStyle& container_style,
      const LogicalSize& container_available_size,
      GridTrackSizingDirection track_direction,
      LayoutUnit parent_gutter_size = LayoutUnit());

  // For the first track, computes the start offset and gutter size based on the
  // alignment properties and available size of the container.
  static FirstSetGeometry ComputeFirstSetGeometry(
      const GridSizingTrackCollection& track_collection,
      const ComputedStyle& container_style,
      const LogicalSize& container_available_size,
      const BoxStrut& container_border_scrollbar_padding);

  // Calculates the used track size from the min and max track sizing functions
  // as defined in https://drafts.csswg.org/css-grid-2/#algo-track-sizing.
  void ComputeUsedTrackSizes(
      const ContributionSizeFunctionRef& contribution_size,
      GridSizingTrackCollection* track_collection,
      GridItems* grid_items) const;

 private:
  // These methods implement the steps of the algorithm for intrinsic track size
  // resolution defined in https://drafts.csswg.org/css-grid-2/#algo-content.
  void ResolveIntrinsicTrackSizes(
      const ContributionSizeFunctionRef& contribution_size,
      GridSizingTrackCollection* track_collection,
      GridItems* grid_items) const;

  void IncreaseTrackSizesToAccommodateGridItems(
      const ContributionSizeFunctionRef& contribution_size,
      base::span<Member<GridItemData>>::iterator group_begin,
      base::span<Member<GridItemData>>::iterator group_end,
      GridItemContributionType contribution_type,
      bool is_group_spanning_flex_track,
      GridSizingTrackCollection* track_collection) const;

  void MaximizeTracks(GridSizingTrackCollection* track_collection) const;

  void StretchAutoTracks(GridSizingTrackCollection* track_collection) const;

  void ExpandFlexibleTracks(
      const ContributionSizeFunctionRef& contribution_size,
      GridSizingTrackCollection* track_collection,
      GridItems* grid_items) const;

  LayoutUnit DetermineFreeSpace(
      const GridSizingTrackCollection& track_collection) const;

  LogicalSize available_size_;
  LogicalSize min_available_size_;
  SizingConstraint sizing_constraint_;
  StyleContentAlignmentData columns_alignment_;
  StyleContentAlignmentData rows_alignment_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_TRACK_SIZING_ALGORITHM_H_
