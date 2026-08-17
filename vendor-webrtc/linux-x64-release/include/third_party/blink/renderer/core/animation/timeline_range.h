// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMELINE_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMELINE_RANGE_H_

#include "cc/animation/scroll_timeline.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_timeline_range.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

struct TimelineOffset;

// A TimelineRange represents a given scroll range within an associated
// scroller's minimum/maximum scroll. This is useful for ViewTimelines
// in particular, because they represent exactly that: a segment of a (non-view)
// ScrollTimeline.
//
// The primary job of TimelineRange is to convert offsets within an animation
// attachment range [1] (represented by TimelineOffset values) to fractional
// offsets within the TimelineRange. See TimelineRange::ToFractionalOffset.
//
// It may be helpful to think about a TimelineRange (which is timeline-specific)
// as a sub-range of the scroller's full range, and an animation attachment
// range (which is animation specific) as a sub-range of that TimelineRange.
//
// - For ViewTimelines, the start/end offsets will correspond to the scroll
//   range that would cause the scrollport to intersect with the subject
//   element's box.
// - For (non-view) ScrollTimelines, the start/end offset is always the
//   same as the minimum/maximum scroll.
// - For monotonic timelines, the TimelineRange is always empty.
//
// [1]
// https://drafts.csswg.org/scroll-animations-1/#named-range-animation-declaration
class CORE_EXPORT TimelineRange {
 public:
  using ScrollOffsets = cc::ScrollTimeline::ScrollOffsets;
  using NamedRange = V8TimelineRange::Enum;

  // For a view timeline, stores the distances of 'entry-crossing' and
  // 'exit-crossing' ranges.
  // Note that 'cover' is in TimelineRange::offsets_, and 'contain' and others
  // can be inferred (see TimelineRange::ConvertNamedRange). Usually, entry and
  // exit distance correspond to the layout size of the subject, but may differ
  // with position:sticky (crbug.com/1448294).
  struct ViewOffsets {
    ViewOffsets() = default;
    ViewOffsets(double entry, double exit) {
      entry_crossing_distance = entry;
      exit_crossing_distance = exit;
    }
    bool operator==(const ViewOffsets& other) const {
      return entry_crossing_distance == other.entry_crossing_distance &&
             exit_crossing_distance == other.exit_crossing_distance;
    }
    bool operator!=(const ViewOffsets& other) const {
      return !(*this == other);
    }
    double entry_crossing_distance = 0;
    double exit_crossing_distance = 0;
  };

  TimelineRange() = default;
  // The `limits` parameter represents the full scroll range,
  // and `offsets` a segment of that range. (See class description.)
  explicit TimelineRange(ScrollOffsets limits,
                         ScrollOffsets offsets,
                         ViewOffsets view_offsets)
      : scroll_limits_(limits),
        offsets_(offsets),
        view_offsets_(view_offsets) {}

  bool operator==(const TimelineRange& other) const {
    return offsets_ == other.offsets_ && view_offsets_ == other.view_offsets_ &&
           scroll_limits_ == other.scroll_limits_;
  }

  bool operator!=(const TimelineRange& other) const {
    return !(*this == other);
  }

  bool IsEmpty() const;

  // Converts an offset within some animation attachment range to a fractional
  // offset within this TimelineRange.
  double ToFractionalOffset(const TimelineOffset&) const;

 private:
  // https://drafts.csswg.org/scroll-animations-1/#view-timelines-ranges
  ScrollOffsets ConvertNamedRange(NamedRange) const;

  // Scroll offsets corresponding to the entire scroll range of the scroll
  // container backing the timeline in the axis of the timeline.
  // For a ScrollTimeline, it is the same as |offsets_|.
  ScrollOffsets scroll_limits_;
  // For a ScrollTimeline: this corresponds to the entire scroll range of the
  // scroll container backing the timeline and is the same as |scroll_limits_|.
  // For a ViewTimeline: this corresponds to the boundaries of the view progress
  // of the subject within the scroll container backing the timeline. It is a
  // segment of |scroll_limits_|.
  ScrollOffsets offsets_;
  ViewOffsets view_offsets_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMELINE_RANGE_H_
