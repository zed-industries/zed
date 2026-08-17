// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_TRACK_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_TRACK_COLLECTION_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

// |GridTrackCollectionBase| provides an implementation for some shared
// functionality on grid collections, specifically binary search on the
// collection to get the range that contains a specific grid line.
class CORE_EXPORT GridTrackCollectionBase {
 public:
  virtual ~GridTrackCollectionBase() = default;

  // Returns the number of track ranges in the collection.
  virtual wtf_size_t RangeCount() const = 0;
  // Returns the start line of a given track range.
  virtual wtf_size_t RangeStartLine(wtf_size_t range_index) const = 0;
  // Returns the number of tracks in a given track range.
  virtual wtf_size_t RangeTrackCount(wtf_size_t range_index) const = 0;

  // Returns the end line of a given track range.
  wtf_size_t RangeEndLine(wtf_size_t range_index) const;
  // Gets the index of the range that contains the given grid line.
  wtf_size_t RangeIndexFromGridLine(wtf_size_t grid_line) const;
};

class CORE_EXPORT TrackSpanProperties {
 public:
  enum PropertyId : unsigned {
    kNoPropertyId = 0,
    kHasAutoMinimumTrack = 1 << 0,
    kHasFixedMaximumTrack = 1 << 1,
    kHasFixedMinimumTrack = 1 << 2,
    kHasFlexibleTrack = 1 << 3,
    kHasIntrinsicTrack = 1 << 4,
    kHasNonDefiniteTrack = 1 << 5,
    kIsCollapsed = 1 << 6,
    kIsDependentOnAvailableSize = 1 << 7,
    kIsImplicit = 1 << 8,
  };

  inline bool HasProperty(PropertyId id) const { return bitmask_ & id; }
  inline void Reset() { bitmask_ &= kIsCollapsed | kIsImplicit; }
  inline void SetProperty(PropertyId id) { bitmask_ |= id; }

  inline TrackSpanProperties& operator|=(const TrackSpanProperties& other) {
    bitmask_ |= other.bitmask_;
    return *this;
  }

 private:
  wtf_size_t bitmask_{kNoPropertyId};
};

struct CORE_EXPORT GridRange {
  bool IsCollapsed() const;
  bool IsImplicit() const;

  void SetIsCollapsed();
  void SetIsImplicit();

  wtf_size_t begin_set_index;
  wtf_size_t repeater_index;
  wtf_size_t repeater_offset;
  wtf_size_t set_count;
  wtf_size_t start_line;
  wtf_size_t track_count;

  TrackSpanProperties properties;
};

using GridRangeVector = Vector<GridRange, 16>;

class CORE_EXPORT GridRangeBuilder {
  STACK_ALLOCATED();

 public:
  GridRangeBuilder() = delete;

  GridRangeBuilder(const ComputedStyle& grid_style,
                   GridTrackSizingDirection track_direction,
                   wtf_size_t auto_repetitions,
                   wtf_size_t start_offset);

  // Ensures that after FinalizeRanges is called, a range will start at the
  // |start_line|, a range will end at |start_line| + |span_length|.
  // |grid_item_start_range_index| and |grid_item_end_range_index| will be
  // written to during |FinalizeRanges|.
  void EnsureTrackCoverage(wtf_size_t start_line,
                           wtf_size_t span_length,
                           wtf_size_t* grid_item_start_range_index,
                           wtf_size_t* grid_item_end_range_index);

  // Build the collection of ranges based on information provided through the
  // specified tracks and |EnsureTrackCoverage|.
  GridRangeVector FinalizeRanges();

 private:
  friend class GridTrackCollectionTest;

  // This structure represents the grid line boundaries of a repeater or item
  // placed on the implicit grid. For the latter, a pointer to its respective
  // range start/end index is provided to cache its value during
  // |FinalizeRanges|.
  struct TrackBoundaryToRangePair {
    explicit TrackBoundaryToRangePair(
        wtf_size_t grid_line,
        wtf_size_t* grid_item_range_index_to_cache = nullptr)
        : grid_line(grid_line),
          grid_item_range_index_to_cache(grid_item_range_index_to_cache) {}
    wtf_size_t grid_line;
    wtf_size_t* grid_item_range_index_to_cache;
  };

  GridRangeBuilder(const NGGridTrackList& explicit_tracks,
                   const NGGridTrackList& implicit_tracks,
                   wtf_size_t auto_repetitions,
                   wtf_size_t start_offset = 0);

  wtf_size_t auto_repetitions_;
  wtf_size_t start_offset_;

  bool must_sort_grid_lines_{false};

  // Stores the grid's explicit and implicit tracks.
  const NGGridTrackList& explicit_tracks_;
  const NGGridTrackList& implicit_tracks_;

  // Starting and ending tracks mark where ranges will start and end.
  // The corresponding range_index will be written to during |FinalizeRanges|.
  // Once the ranges have been built in FinalizeRanges, these are cleared.
  Vector<TrackBoundaryToRangePair, 16> start_lines_;
  Vector<TrackBoundaryToRangePair, 16> end_lines_;
};

class CORE_EXPORT GridLayoutTrackCollection : public GridTrackCollectionBase {
  USING_FAST_MALLOC(GridLayoutTrackCollection);

 public:
  struct SetGeometry {
    explicit SetGeometry(LayoutUnit offset, wtf_size_t track_count = 0)
        : offset(offset), track_count(track_count) {}

    LayoutUnit offset;
    wtf_size_t track_count;
  };

  GridLayoutTrackCollection() = delete;

  // Don't allow this class to be used for grid sizing.
  virtual bool IsForSizing() const { return false; }

  bool operator==(const GridLayoutTrackCollection& other) const;

  // GridTrackCollectionBase overrides.
  wtf_size_t RangeCount() const override { return ranges_.size(); }
  wtf_size_t RangeStartLine(wtf_size_t range_index) const override;
  wtf_size_t RangeTrackCount(wtf_size_t range_index) const override;

  // Returns the number of sets spanned by a given track range.
  wtf_size_t RangeSetCount(wtf_size_t range_index) const;
  // Return the index of the first set spanned by a given track range.
  wtf_size_t RangeBeginSetIndex(wtf_size_t range_index) const;
  // Returns the track span properties of the range at position |range_index|.
  TrackSpanProperties RangeProperties(wtf_size_t range_index) const;

  wtf_size_t EndLineOfImplicitGrid() const;
  // Returns true if |grid_line| is contained within the implicit grid.
  bool IsGridLineWithinImplicitGrid(wtf_size_t grid_line) const;

  wtf_size_t GetSetCount() const;
  LayoutUnit GetSetOffset(wtf_size_t set_index) const;
  wtf_size_t GetSetTrackCount(wtf_size_t set_index) const;

  // Returns the accumulated extra margin at the start/end of the specified set;
  // if no index is specified, returns the extra margin of the grid container.
  LayoutUnit StartExtraMargin(wtf_size_t set_index = 0) const;
  LayoutUnit EndExtraMargin(wtf_size_t set_index = kNotFound) const;

  bool HasBaselines() const { return baselines_.has_value(); }
  LayoutUnit MajorBaseline(wtf_size_t set_index) const;
  LayoutUnit MinorBaseline(wtf_size_t set_index) const;

  // Increase by |delta| the offset of every set with index > |set_index|.
  void AdjustSetOffsets(wtf_size_t set_index, LayoutUnit delta);

  // Returns the total size of all sets in the collection.
  LayoutUnit CalculateSetSpanSize() const;
  // Returns the total size of all sets with index in the range [begin, end).
  LayoutUnit CalculateSetSpanSize(wtf_size_t begin_set_index,
                                  wtf_size_t end_set_index) const;

  // Creates a track collection containing every |Range| with index in the range
  // [begin, end], including their respective |SetGeometry| and baselines.
  GridLayoutTrackCollection CreateSubgridTrackCollection(
      wtf_size_t begin_range_index,
      wtf_size_t end_range_index,
      LayoutUnit subgrid_gutter_size,
      const BoxStrut& subgrid_margin,
      const BoxStrut& subgrid_border_scrollbar_padding,
      GridTrackSizingDirection subgrid_track_direction,
      bool is_opposite_direction_in_root_grid) const;

  GridTrackSizingDirection Direction() const { return track_direction_; }
  LayoutUnit GutterSize() const { return gutter_size_; }

  bool HasFlexibleTrack() const;
  bool HasIndefiniteSet() const;
  bool HasIntrinsicTrack() const;
  bool HasNonDefiniteTrack() const;
  bool IsDependentOnAvailableSize() const;

 protected:
  struct Baselines {
    Vector<LayoutUnit, 16> major;
    Vector<LayoutUnit, 16> minor;
  };

  explicit GridLayoutTrackCollection(GridTrackSizingDirection track_direction)
      : track_direction_(track_direction) {}

  // Checks whether any set in the range [begin, end) is indefinite.
  bool IsSpanningIndefiniteSet(wtf_size_t begin_set_index,
                               wtf_size_t end_set_index) const;

  LayoutUnit gutter_size_;
  GridRangeVector ranges_;
  TrackSpanProperties properties_;
  Vector<SetGeometry, 16> sets_geometry_;
  GridTrackSizingDirection track_direction_;

  // Baselines are only created when there are items with baseline alignment.
  std::optional<Baselines> baselines_;

  // Initially we only know some of the set sizes - others will be indefinite.
  // To represent this we store a vector of the last indefinite indices for each
  // set (or `kNotFound` if every set has been definite so far). This allow us
  // to get the appropriate size if a grid item spans only fixed tracks or
  // `kIndefiniteSize` if it spans an indefinite set. E.g.:
  //
  //   grid-template-rows: auto auto 100px 100px auto 100px;
  //
  // Results in the `last_indefinite_index` vector being:
  //
  //                  | auto | auto | 100px | 100px | auto | 100px |
  //      [ kNotFound ,    0 ,    1 ,     1 ,     1 ,    4 ,     4 ]
  //
  // Various queries (start/end refer to the grid lines):
  //  (start: 0, end: 1) -> indefinite as:
  //      start <= last_indefinite_index[end]
  //  (start: 1, end: 3) -> indefinite as:
  //      start <= last_indefinite_index[end]
  //  (start: 2, end: 4) -> 200px
  //  (start: 5, end: 6) -> 100px
  //  (start: 3, end: 5) -> indefinite as:
  //      start <= last_indefinite_index[end]
  Vector<wtf_size_t, 16> last_indefinite_index_;

  LayoutUnit accumulated_gutter_size_delta_;
  LayoutUnit accumulated_start_extra_margin_;
  LayoutUnit accumulated_end_extra_margin_;
};

// |GridRangeBuilder::EnsureTrackCoverage| may introduce a range start and/or
// end at the middle of any repeater from the block collection. This will affect
// how some repeated tracks within the same repeater group resolve their track
// sizes; e.g. consider the track list 'repeat(10, auto)' with a grid item
// spanning from the 3rd to the 7th track in the repeater, every track within
// the item's range will grow to fit the content of that item first.
//
// For the track sizing algorithm we want to have separate data (e.g. base size,
// growth limit, etc.) between tracks in different ranges; instead of trivially
// expanding the repeaters, which will limit our implementation to support
// relatively small track counts, we introduce the concept of a "set".
//
// A "set" is a collection of distinct track definitions that compose a range in
// |GridSizingTrackCollection|; each set element stores the number of tracks
// within the range that share its definition. The |GridSet| class represents
// a single element from a set.
//
// As an example, consider the following grid definition:
//   - 'grid-template-columns: repeat(4, 5px 1fr)'
//   - Grid item 1 with 'grid-column: 1 / span 5'
//   - Grid item 2 with 'grid-column: 2 / span 1'
//   - Grid item 3 with 'grid-column: 6 / span 8'
//
// Expanding the track definitions above we would look at the explicit grid:
//   | 5px | 1fr | 5px | 1fr | 5px | 1fr | 5px | 1fr |
//
// This example would produce the following ranges and their respective sets:
//   Range 1:  [1-1], Set 1: {  5px (1) }
//   Range 2:  [2-2], Set 2: {  1fr (1) }
//   Range 3:  [3-5], Set 3: {  5px (2) , 1fr (1) }
//   Range 4:  [6-8], Set 4: {  1fr (2) , 5px (1) }
//   Range 5: [9-13], Set 5: { auto (5) }
//
// Note that, since |GridRangeBuilder|'s ranges are assured to span a single
// repeater and to not cross any grid item's boundary in the respective
// dimension, tracks within a set are "commutative" and can be sized evenly.
struct CORE_EXPORT GridSet {
  explicit GridSet(wtf_size_t track_count)
      : track_count(track_count), track_size(Length::Auto(), Length::Auto()) {}

  // |is_available_size_indefinite| is used to normalize percentage track
  // sizing functions; from https://drafts.csswg.org/css-grid-2/#track-sizes:
  //   "If the size of the grid container depends on the size of its tracks,
  //   then the <percentage> must be treated as 'auto'".
  GridSet(wtf_size_t track_count,
          const GridTrackSize& track_definition,
          bool is_available_size_indefinite);

  float FlexFactor() const;
  LayoutUnit BaseSize() const;
  LayoutUnit GrowthLimit() const;

  void InitBaseSize(LayoutUnit new_base_size);
  void IncreaseBaseSize(LayoutUnit new_base_size);
  void IncreaseGrowthLimit(LayoutUnit new_growth_limit);

  void EnsureGrowthLimitIsNotLessThanBaseSize();
  bool IsGrowthLimitLessThanBaseSize() const;

  wtf_size_t track_count;
  GridTrackSize track_size;

  // Fields used by the track sizing algorithm.
  LayoutUnit base_size;
  LayoutUnit growth_limit;
  LayoutUnit planned_increase;
  LayoutUnit fit_content_limit;
  LayoutUnit item_incurred_increase;

  bool is_infinitely_growable : 1;
};

class CORE_EXPORT GridSizingTrackCollection final
    : public GridLayoutTrackCollection {
  USING_FAST_MALLOC(GridSizingTrackCollection);

 public:
  template <bool is_const>
  class CORE_EXPORT SetIteratorBase {
   public:
    using TrackCollectionPtr =
        typename std::conditional<is_const,
                                  const GridSizingTrackCollection*,
                                  GridSizingTrackCollection*>::type;
    using GridSetRef =
        typename std::conditional<is_const, const GridSet&, GridSet&>::type;

    SetIteratorBase(TrackCollectionPtr track_collection,
                    wtf_size_t begin_set_index,
                    wtf_size_t end_set_index)
        : track_collection_(track_collection),
          current_set_index_(begin_set_index),
          end_set_index_(end_set_index) {
      DCHECK(track_collection_);
      DCHECK_LE(current_set_index_, end_set_index_);
      DCHECK_LE(end_set_index_, track_collection_->GetSetCount());
    }

    bool IsAtEnd() const {
      DCHECK_LE(current_set_index_, end_set_index_);
      return current_set_index_ == end_set_index_;
    }

    bool MoveToNextSet() {
      current_set_index_ = std::min(current_set_index_ + 1, end_set_index_);
      return current_set_index_ < end_set_index_;
    }

    GridSetRef CurrentSet() const {
      DCHECK_LT(current_set_index_, end_set_index_);
      return track_collection_->GetSetAt(current_set_index_);
    }

   private:
    TrackCollectionPtr track_collection_;
    wtf_size_t current_set_index_;
    wtf_size_t end_set_index_;
  };

  typedef SetIteratorBase<false> SetIterator;
  typedef SetIteratorBase<true> ConstSetIterator;

  GridSizingTrackCollection() = delete;
  GridSizingTrackCollection(GridSizingTrackCollection&&) = default;
  GridSizingTrackCollection(const GridSizingTrackCollection&) = delete;
  GridSizingTrackCollection& operator=(GridSizingTrackCollection&&) = default;
  GridSizingTrackCollection& operator=(const GridSizingTrackCollection&) =
      delete;

  explicit GridSizingTrackCollection(
      GridRangeVector&& ranges,
      GridTrackSizingDirection track_direction = kForColumns,
      bool must_create_baselines = false);

  // This class should be specifically used for grid sizing.
  bool IsForSizing() const override { return true; }

  // Returns a reference to the set located at position |set_index|.
  GridSet& GetSetAt(wtf_size_t set_index);
  const GridSet& GetSetAt(wtf_size_t set_index) const;
  // Returns an iterator for all the sets contained in this collection.
  SetIterator GetSetIterator();
  ConstSetIterator GetConstSetIterator() const;
  // Returns an iterator for every set in this collection's |sets_| located at
  // an index in the interval [begin_set_index, end_set_index).
  SetIterator GetSetIterator(wtf_size_t begin_set_index,
                             wtf_size_t end_set_index);

  wtf_size_t NonCollapsedTrackCount() const {
    return non_collapsed_track_count_;
  }
  LayoutUnit TotalTrackSize() const;

  void BuildSets(const ComputedStyle& grid_style,
                 const LogicalSize& grid_available_size);
  void SetIndefiniteGrowthLimitsToBaseSize();

  // Caches the geometry of definite sets; this is useful when building the sets
  // of a subgrid since we need to determine whether its available size (i.e.,
  // the grid area it spans on its parent grid) is definite or not.
  void CacheDefiniteSetsGeometry();
  // Caches the geometry of the initialized sets' growth limit if they're
  // definite; this will be used to measure grid item contributions.
  void CacheInitializedSetsGeometry(LayoutUnit first_set_offset);
  // Caches the final geometry used to layout grid items.
  void FinalizeSetsGeometry(LayoutUnit first_set_offset,
                            LayoutUnit override_gutter_size);

  void ResetBaselines();
  void SetMajorBaseline(wtf_size_t set_index, LayoutUnit candidate_baseline);
  void SetMinorBaseline(wtf_size_t set_index, LayoutUnit candidate_baseline);

 private:
  friend class GridLayoutAlgorithmTest;
  friend class GridTrackCollectionTest;
  friend class MasonryLayoutAlgorithmTest;

  // These methods are internal implementations also used in testing.
  void BuildSets(const NGGridTrackList& explicit_track_list,
                 const NGGridTrackList& implicit_track_list,
                 bool is_available_size_indefinite = true);
  void InitializeSets(LayoutUnit grid_available_size = kIndefiniteSize);

  wtf_size_t non_collapsed_track_count_{0};

  // A vector of every set element that compose the entire collection's ranges;
  // track definitions from the same set are stored in consecutive positions,
  // preserving the order in which the definitions appear in their range.
  Vector<GridSet, 16> sets_;
};

template <>
struct DowncastTraits<GridSizingTrackCollection> {
  static bool AllowFrom(const GridLayoutTrackCollection& layout_collection) {
    return layout_collection.IsForSizing();
  }
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::GridRange)
WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::GridLayoutTrackCollection::SetGeometry)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_TRACK_COLLECTION_H_
