// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_ITEM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/baseline_utils.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/constraint_space.h"
#include "third_party/blink/renderer/core/layout/grid/grid_track_collection.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

enum class AxisEdge { kStart, kCenter, kEnd, kFirstBaseline, kLastBaseline };

struct GridItemIndices {
  wtf_size_t begin{kNotFound};
  wtf_size_t end{kNotFound};
};

struct OutOfFlowItemPlacement {
  GridItemIndices range_index;
  GridItemIndices offset_in_range;
};

struct CORE_EXPORT GridItemData : public GarbageCollected<GridItemData> {
  GridItemData() = default;
  GridItemData(const GridItemData&) = default;
  GridItemData& operator=(const GridItemData&) = default;

  GridItemData(BlockNode item_node,
               const ComputedStyle& parent_grid_style,
               const ComputedStyle& root_grid_style,
               bool parent_must_consider_grid_items_for_column_sizing = false,
               bool parent_must_consider_grid_items_for_row_sizing = false);

  GridItemData(BlockNode item_node, const ComputedStyle& parent_style)
      : GridItemData(std::move(item_node), parent_style, parent_style) {}

  void SetAlignmentFallback(GridTrackSizingDirection track_direction,
                            bool has_synthesized_baseline);

  AxisEdge Alignment(GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns)
               ? column_fallback_alignment.value_or(column_alignment)
               : row_fallback_alignment.value_or(row_alignment);
  }

  bool IsOverflowSafe(GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns)
               ? column_fallback_alignment || is_overflow_safe_for_columns
               : row_fallback_alignment || is_overflow_safe_for_rows;
  }

  bool IsBaselineAligned(GridTrackSizingDirection track_direction) const {
    const auto axis_alignment = Alignment(track_direction);
    return (axis_alignment == AxisEdge::kFirstBaseline ||
            axis_alignment == AxisEdge::kLastBaseline);
  }

  bool IsBaselineSpecified(GridTrackSizingDirection track_direction) const {
    const auto& axis_alignment =
        (track_direction == kForColumns) ? column_alignment : row_alignment;
    return (axis_alignment == AxisEdge::kFirstBaseline ||
            axis_alignment == AxisEdge::kLastBaseline);
  }

  bool IsLastBaselineSpecified(GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns)
               ? column_alignment == AxisEdge::kLastBaseline
               : row_alignment == AxisEdge::kLastBaseline;
  }

  // For this item and track direction, computes the pair of indices `begin` and
  // `end` such that the item spans every set from the respective collection's
  // `sets_` with an index in the range [begin, end).
  void ComputeSetIndices(const GridLayoutTrackCollection& track_collection);

  // For this out of flow item and track collection, computes and stores its
  // first and last spanned ranges, as well as the start and end track offset.
  // `grid_placement` is used to resolve the grid lines.
  void ComputeOutOfFlowItemPlacement(
      const GridLayoutTrackCollection& track_collection,
      const GridPlacementData& placement_data,
      const ComputedStyle& grid_style);

  // Returns the total size of the tracks spanned by this item in the given
  // track collection. If `start_offset` is provided, it will be set to the
  // offset of the first track spanned by this grid item.
  LayoutUnit CalculateAvailableSize(
      const GridLayoutTrackCollection& track_collection,
      LayoutUnit* start_offset = nullptr) const;

  enum BaselineGroup BaselineGroup(
      GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns) ? column_baseline_group
                                            : row_baseline_group;
  }

  WritingDirectionMode BaselineWritingDirection(
      GridTrackSizingDirection track_direction) const {
    // NOTE: For reading the baseline from a fragment the direction doesn't
    // matter - just use the default.
    return {(track_direction == kForColumns) ? column_baseline_writing_mode
                                             : row_baseline_writing_mode,
            TextDirection::kLtr};
  }

  const GridItemIndices& SetIndices(
      GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns) ? column_set_indices
                                            : row_set_indices;
  }

  GridSizingTrackCollection::SetIterator SetIterator(
      GridSizingTrackCollection* track_collection) const {
    DCHECK(track_collection);
    const auto& set_indices = SetIndices(track_collection->Direction());
    return track_collection->GetSetIterator(set_indices.begin, set_indices.end);
  }

  GridItemIndices& RangeIndices(GridTrackSizingDirection track_direction) {
    return (track_direction == kForColumns) ? column_range_indices
                                            : row_range_indices;
  }

  void ResetPlacementIndices() {
    column_range_indices = row_range_indices = GridItemIndices();
    column_set_indices = row_set_indices = GridItemIndices();
  }

  const GridSpan& Span(GridTrackSizingDirection track_direction) const {
    return resolved_position.Span(track_direction);
  }
  wtf_size_t StartLine(GridTrackSizingDirection track_direction) const {
    return resolved_position.StartLine(track_direction);
  }
  wtf_size_t EndLine(GridTrackSizingDirection track_direction) const {
    return resolved_position.EndLine(track_direction);
  }
  wtf_size_t SpanSize(GridTrackSizingDirection track_direction) const {
    return resolved_position.SpanSize(track_direction);
  }

  bool IsSubgrid() const {
    return has_subgridded_columns || has_subgridded_rows;
  }

  bool IsConsideredForSizing(GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns) ? is_considered_for_column_sizing
                                            : is_considered_for_row_sizing;
  }

  bool IsOppositeDirectionInRootGrid(
      GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns)
               ? is_opposite_direction_in_root_grid_columns
               : is_opposite_direction_in_root_grid_rows;
  }

  bool MustCachePlacementIndices(
      GridTrackSizingDirection track_direction) const {
    return !is_subgridded_to_parent_grid ||
           IsConsideredForSizing(track_direction) ||
           MustConsiderGridItemsForSizing(track_direction);
  }

  bool MustConsiderGridItemsForSizing(
      GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns)
               ? must_consider_grid_items_for_column_sizing
               : must_consider_grid_items_for_row_sizing;
  }

  bool IsOutOfFlow() const { return node && node.IsOutOfFlowPositioned(); }

  const TrackSpanProperties& GetTrackSpanProperties(
      GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns) ? column_span_properties
                                            : row_span_properties;
  }

  void SetTrackSpanProperty(const TrackSpanProperties::PropertyId property,
                            GridTrackSizingDirection track_direction) {
    if (track_direction == kForColumns)
      column_span_properties.SetProperty(property);
    else
      row_span_properties.SetProperty(property);
  }

  bool IsSpanningFlexibleTrack(GridTrackSizingDirection track_direction) const {
    return GetTrackSpanProperties(track_direction)
        .HasProperty(TrackSpanProperties::kHasFlexibleTrack);
  }
  bool IsSpanningIntrinsicTrack(
      GridTrackSizingDirection track_direction) const {
    return GetTrackSpanProperties(track_direction)
        .HasProperty(TrackSpanProperties::kHasIntrinsicTrack);
  }
  bool IsSpanningAutoMinimumTrack(
      GridTrackSizingDirection track_direction) const {
    return GetTrackSpanProperties(track_direction)
        .HasProperty(TrackSpanProperties::kHasAutoMinimumTrack);
  }
  bool IsSpanningFixedMinimumTrack(
      GridTrackSizingDirection track_direction) const {
    return GetTrackSpanProperties(track_direction)
        .HasProperty(TrackSpanProperties::kHasFixedMinimumTrack);
  }
  bool IsSpanningFixedMaximumTrack(
      GridTrackSizingDirection track_direction) const {
    return GetTrackSpanProperties(track_direction)
        .HasProperty(TrackSpanProperties::kHasFixedMaximumTrack);
  }

  void EncompassContributionSizes(MinMaxSizes&& sizes) {
    if (contribution_sizes) {
      contribution_sizes->Encompass(sizes);
    } else {
      contribution_sizes = std::move(sizes);
    }
  }

  void Trace(Visitor* visitor) const { visitor->Trace(node); }

  BlockNode node{nullptr};
  GridArea resolved_position;

  bool has_subgridded_columns : 1 {false};
  bool has_subgridded_rows : 1 {false};
  bool is_considered_for_column_sizing : 1 {true};
  bool is_considered_for_row_sizing : 1 {true};
  bool is_opposite_direction_in_root_grid_columns : 1 {false};
  bool is_opposite_direction_in_root_grid_rows : 1 {false};
  bool is_overflow_safe_for_columns : 1 {false};
  bool is_overflow_safe_for_rows : 1 {false};
  bool is_parallel_with_root_grid : 1 {false};
  bool is_sizing_dependent_on_block_size : 1 {false};
  bool is_subgridded_to_parent_grid : 1 {false};
  bool must_consider_grid_items_for_column_sizing : 1 {false};
  bool must_consider_grid_items_for_row_sizing : 1 {false};

  FontBaseline parent_grid_font_baseline;

  AxisEdge column_alignment;
  AxisEdge row_alignment;

  std::optional<AxisEdge> column_fallback_alignment;
  std::optional<AxisEdge> row_fallback_alignment;

  AutoSizeBehavior column_auto_behavior;
  AutoSizeBehavior row_auto_behavior;

  enum BaselineGroup column_baseline_group;
  enum BaselineGroup row_baseline_group;

  WritingMode column_baseline_writing_mode;
  WritingMode row_baseline_writing_mode;

  TrackSpanProperties column_span_properties;
  TrackSpanProperties row_span_properties;

  GridItemIndices column_set_indices;
  GridItemIndices row_set_indices;

  GridItemIndices column_range_indices;
  GridItemIndices row_range_indices;

  // These fields are only for out of flow items. They are used to store their
  // start/end range indices, and offsets in range in the respective track
  // collection; see `OutOfFlowItemPlacement`.
  OutOfFlowItemPlacement column_placement;
  OutOfFlowItemPlacement row_placement;

  // Virtual masonry items don't have a node, so we cache the maximum of every
  // intrinsic contribution among the items that make up its respective group.
  std::optional<MinMaxSizes> contribution_sizes;
};

class CORE_EXPORT GridItems {
  DISALLOW_NEW();

 public:
  using GridItemDataVector = HeapVector<Member<GridItemData>, 16>;

  template <bool is_const>
  class Iterator {
    STACK_ALLOCATED();

   public:
    using value_type = typename std::
        conditional<is_const, const GridItemData, GridItemData>::type;

    using GridItemDataVectorPtr =
        typename std::conditional<is_const,
                                  const GridItemDataVector*,
                                  GridItemDataVector*>::type;

    Iterator(GridItemDataVectorPtr item_data, wtf_size_t current_index)
        : current_index_(current_index), item_data_(item_data) {
      DCHECK(item_data_);
      DCHECK_LE(current_index_, item_data_->size());
    }

    bool operator!=(const Iterator& other) {
      return current_index_ != other.current_index_ ||
             item_data_ != other.item_data_;
    }

    Iterator& operator++() {
      ++current_index_;
      return *this;
    }

    Iterator operator++(int) {
      auto current_iterator = *this;
      ++current_index_;
      return current_iterator;
    }

    value_type* operator->() const {
      DCHECK_LT(current_index_, item_data_->size());
      return item_data_->at(current_index_).Get();
    }

    value_type& operator*() const { return *operator->(); }

   private:
    wtf_size_t current_index_;
    GridItemDataVectorPtr item_data_;
  };

  template <bool is_const>
  class Range {
    STACK_ALLOCATED();

   public:
    Range(Iterator<is_const>&& begin, Iterator<is_const>&& end)
        : begin_(std::move(begin)), end_(std::move(end)) {}

    Iterator<is_const> begin() const { return begin_; }
    Iterator<is_const> end() const { return end_; }

   private:
    Iterator<is_const> begin_;
    Iterator<is_const> end_;
  };

  GridItems() = default;
  GridItems(GridItems&&) = default;
  GridItems& operator=(GridItems&&) = default;

  GridItems(const GridItems& other);

  GridItems& operator=(const GridItems& other) {
    return *this = GridItems(other);
  }

  Iterator<false> begin() { return {&item_data_, 0}; }
  Iterator<false> end() { return {&item_data_, first_subgridded_item_index_}; }

  Range<false> IncludeSubgriddedItems() {
    return {begin(), /* end */ {&item_data_, item_data_.size()}};
  }

  Iterator<true> begin() const { return {&item_data_, 0}; }
  Iterator<true> end() const {
    return {&item_data_, first_subgridded_item_index_};
  }

  Range<true> IncludeSubgriddedItems() const {
    return {begin(), /* end */ {&item_data_, item_data_.size()}};
  }

  bool IsEmpty() const { return item_data_.empty(); }
  wtf_size_t Size() const { return item_data_.size(); }

  void Append(GridItems* other);
  void SortByOrderProperty();

  void Append(GridItemData* new_item_data) {
    DCHECK(new_item_data);
    if (!new_item_data->is_subgridded_to_parent_grid) {
      // Subgridded items are appended after non-subgridded ones; keep moving
      // `first_subgridded_item_index_` while we append non-subgridded items.
      DCHECK_EQ(first_subgridded_item_index_, item_data_.size());
      ++first_subgridded_item_index_;
    }
    item_data_.emplace_back(std::move(new_item_data));
  }

  GridItemData& At(wtf_size_t index) {
    DCHECK_LT(index, item_data_.size());
    DCHECK(item_data_[index]);
    return *item_data_[index];
  }

  const GridItemData& At(wtf_size_t index) const {
    DCHECK_LT(index, item_data_.size());
    DCHECK(item_data_[index]);
    return *item_data_[index];
  }

  void ReserveInitialCapacity(wtf_size_t initial_capacity) {
    item_data_.ReserveInitialCapacity(initial_capacity);
  }

  void Trace(Visitor* visitor) const { visitor->Trace(item_data_); }

 private:
  // End index used to iterate over the non-subgridded items of the collection.
  wtf_size_t first_subgridded_item_index_{0};

  // Grid items are rearranged in order-modified document order since
  // auto-placement and painting rely on it later in the algorithm.
  GridItemDataVector item_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_ITEM_H_
