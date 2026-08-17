// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be found
// in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MASONRY_MASONRY_RUNNING_POSITIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MASONRY_MASONRY_RUNNING_POSITIONS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

struct GridSpan;

// This class holds a list of running positions for each track. This will be
// used to calculate the next position that an item should be placed.
class CORE_EXPORT MasonryRunningPositions {
 public:
  MasonryRunningPositions(wtf_size_t track_count,
                          LayoutUnit initial_running_position,
                          LayoutUnit tie_threshold)
      : running_positions_(track_count, initial_running_position),
        tie_threshold_(tie_threshold) {}

  // Return the first span within `tie_threshold_` of the minimum max-position
  // that comes after the auto-placement cursor in masonry's flow.
  GridSpan GetFirstEligibleLine(wtf_size_t span_size,
                                LayoutUnit& max_position) const;

  // Update all the running positions for the tracks within the given lines to
  // have the inputted `running_position`.
  void UpdateRunningPositionsForSpan(const GridSpan& span,
                                     LayoutUnit running_position);

  // Returns the max-position for a given span.
  LayoutUnit GetMaxPositionForSpan(const GridSpan& span) const;

  void UpdateAutoPlacementCursor(wtf_size_t line) {
    auto_placement_cursor_ = line;
  }

 private:
  friend class MasonryLayoutAlgorithmTest;

  // Struct to keep track of a span of tracks' start lines and their
  // max-positions, where the max-position of a span represents the maximum
  // running position of all tracks in a span. This will always be used in
  // conjunction with a span size, so we can calculate the ending line using
  // `start_line` and a given span size.
  struct MaxPositionSpan {
    bool operator==(const MaxPositionSpan& other) const {
      return (start_line == other.start_line) && (max_pos == other.max_pos);
    }

    wtf_size_t start_line;
    LayoutUnit max_pos;
  };

  // For testing only.
  MasonryRunningPositions(const Vector<LayoutUnit>& running_positions,
                          LayoutUnit tie_threshold)
      : running_positions_(running_positions), tie_threshold_(tie_threshold) {}

  void SetAutoPlacementCursorForTesting(wtf_size_t cursor) {
    auto_placement_cursor_ = cursor;
  }

  // For each track span of size `span_size` in `running_positions_`, compute
  // its max-position and return a vector where the index corresponds to the
  // track number and the value corresponds to the max-position for that track.
  Vector<LayoutUnit> GetMaxPositionsForAllTracks(wtf_size_t span_size) const;

  // The index of the `running_positions_` vector corresponds to the track
  // number, while the value of the vector item corresponds to the current
  // running position of the track. Note that the tracks are 0-indexed.
  Vector<LayoutUnit> running_positions_;

  wtf_size_t auto_placement_cursor_{0};
  LayoutUnit tie_threshold_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MASONRY_MASONRY_RUNNING_POSITIONS_H_
