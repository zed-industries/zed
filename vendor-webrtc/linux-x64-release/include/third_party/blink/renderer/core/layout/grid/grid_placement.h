// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_PLACEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_PLACEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/grid/grid_data.h"
#include "third_party/blink/renderer/platform/wtf/doubly_linked_list.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class GridItems;

// This class encapsulates the Grid Item Placement Algorithm described by
// https://drafts.csswg.org/css-grid/#auto-placement-algo
class CORE_EXPORT GridPlacement {
  STACK_ALLOCATED();

 public:
  enum class PackingBehavior { kSparse, kDense };

  GridPlacement(const ComputedStyle& grid_style,
                const GridLineResolver& line_resolver);

  GridPlacementData RunAutoPlacementAlgorithm(const GridItems& grid_items);

  // Helper function to resolve start and end lines of out of flow items.
  static void ResolveOutOfFlowItemGridLines(
      const GridLayoutTrackCollection& track_collection,
      const GridLineResolver& line_resolver,
      const ComputedStyle& grid_style,
      const ComputedStyle& item_style,
      wtf_size_t start_offset,
      wtf_size_t* start_line,
      wtf_size_t* end_line);

 private:
  enum class CursorMovementBehavior { kAuto, kForceMajorLine, kForceMinorLine };

  struct GridPosition {
    bool operator<=(const GridPosition& other) const;
    bool operator<(const GridPosition& other) const;

    wtf_size_t major_line{0};
    wtf_size_t minor_line{0};
  };

  class PlacedGridItem final : public DoublyLinkedListNode<PlacedGridItem> {
    USING_FAST_MALLOC(PlacedGridItem);
    friend class DoublyLinkedListNode<PlacedGridItem>;

   public:
    PlacedGridItem(const GridArea& position,
                   GridTrackSizingDirection major_direction,
                   GridTrackSizingDirection minor_direction);

    bool operator<(const PlacedGridItem& rhs) const {
      return start_ < rhs.start_;
    }

    GridPosition Start() const { return start_; }
    GridPosition End() const { return end_; }
    GridPosition EndOnPreviousMajorLine() const;

    wtf_size_t MajorStartLine() const { return start_.major_line; }
    wtf_size_t MinorStartLine() const { return start_.minor_line; }
    wtf_size_t MajorEndLine() const { return end_.major_line; }
    wtf_size_t MinorEndLine() const { return end_.minor_line; }

   private:
    GridPosition start_, end_;
    PlacedGridItem* next_{nullptr};
    PlacedGridItem* prev_{nullptr};
  };

  class AutoPlacementCursor {
   public:
    explicit AutoPlacementCursor(const PlacedGridItem* first_placed_item)
        : should_move_to_next_item_major_end_line_(true),
          next_placed_item_(first_placed_item) {}

    void MoveCursorToFitGridSpan(
        const wtf_size_t major_span_size,
        const wtf_size_t minor_span_size,
        const wtf_size_t minor_max_end_line,
        const CursorMovementBehavior movement_behavior);
    void MoveToMajorLine(const wtf_size_t major_line);
    void MoveToMinorLine(const wtf_size_t minor_line);

    void InsertPlacedItemAtCurrentPosition(
        const PlacedGridItem* new_placed_item);

    wtf_size_t MajorLine() const { return current_position_.major_line; }
    wtf_size_t MinorLine() const { return current_position_.minor_line; }

    const PlacedGridItem* NextPlacedItem() const { return next_placed_item_; }

   private:
    // Comparer needed to use |items_overlapping_major_line_| as a heap.
    struct {
      bool operator()(const PlacedGridItem* lhs,
                      const PlacedGridItem* rhs) const {
        return rhs->End() < lhs->End();
      }
    } ComparePlacedGridItemsByEnd;

    void MoveToNextMajorLine(bool allow_minor_line_movement);
    void UpdateItemsOverlappingMajorLine();

    Vector<const PlacedGridItem*, 16> items_overlapping_major_line_;
    bool should_move_to_next_item_major_end_line_ : 1;
    const PlacedGridItem* next_placed_item_;
    GridPosition current_position_;
  };

  struct PlacedGridItemsList {
    void AppendCurrentItemsToOrderedList();
    PlacedGridItem* FirstPlacedItem() { return ordered_list.Head(); }

    Vector<std::unique_ptr<PlacedGridItem>, 16> item_vector;
    DoublyLinkedList<PlacedGridItem> ordered_list;
    bool needs_to_sort_item_vector : 1;
  };

  using PositionVector = Vector<GridArea*, 16>;

  // Place non auto-positioned elements from |grid_items|; returns true if any
  // item needs to resolve an automatic position. Otherwise, false.
  bool PlaceNonAutoGridItems(
      const GridItems& grid_items,
      PlacedGridItemsList* placed_items,
      PositionVector* positions_locked_to_major_axis,
      PositionVector* positions_not_locked_to_major_axis);
  // Place elements from |grid_items| that have a definite position on the major
  // axis but need auto-placement on the minor axis.
  void PlaceGridItemsLockedToMajorAxis(
      const PositionVector& positions_locked_to_major_axis,
      PlacedGridItemsList* placed_items);
  // Place an item that has a definite position on the minor axis but need
  // auto-placement on the major axis.
  void PlaceAutoMajorAxisGridItem(GridArea* position,
                                  PlacedGridItemsList* placed_items,
                                  AutoPlacementCursor* placement_cursor) const;
  // Place an item that needs auto-placement on both the major and minor axis.
  void PlaceAutoBothAxisGridItem(GridArea* position,
                                 PlacedGridItemsList* placed_items,
                                 AutoPlacementCursor* placement_cursor) const;
  // Update the list of placed grid items and auto-placement cursor using the
  // resolved position of the specified grid item.
  void PlaceGridItemAtCursor(const GridArea& position,
                             PlacedGridItemsList* placed_items,
                             AutoPlacementCursor* placement_cursor) const;
  // After the auto-placement algorithm is done, if we're placing items within a
  // subgrid, clamp their resolved positions to the subgrid's explicit grid.
  void ClampGridItemsToFitSubgridArea(GridTrackSizingDirection track_direction);

  void ClampMinorMaxToSubgridArea();

  bool HasSparsePacking() const;

  // The maximum end line for a given direction, not counting implicit tracks.
  // For subgrids, this gets clamped by the subgrid span size.
  wtf_size_t IntrinsicEndLine(GridTrackSizingDirection track_direction) const;

#if DCHECK_IS_ON()
  bool auto_placement_algorithm_called_{false};
#endif

  GridPlacementData placement_data_;
  PackingBehavior packing_behavior_;
  GridTrackSizingDirection major_direction_;
  GridTrackSizingDirection minor_direction_;
  wtf_size_t minor_max_end_line_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_PLACEMENT_H_
