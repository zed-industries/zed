// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMELINE_INSET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMELINE_INSET_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/length.h"

namespace blink {

// https://drafts.csswg.org/scroll-animations-1/#view-timeline-inset
class CORE_EXPORT TimelineInset {
 public:
  TimelineInset() = default;
  TimelineInset(const Length& start, const Length& end)
      : start_(start), end_(end) {}

  // Note these represent the logical start/end sides of the source scroller,
  // not the start/end of the timeline.
  // https://drafts.csswg.org/css-writing-modes-4/#css-start
  const Length& GetStart() const { return start_; }
  const Length& GetEnd() const { return end_; }

  bool operator==(const TimelineInset& o) const {
    return start_ == o.start_ && end_ == o.end_;
  }

  bool operator!=(const TimelineInset& o) const { return !(*this == o); }

 private:
  Length start_;
  Length end_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMELINE_INSET_H_
