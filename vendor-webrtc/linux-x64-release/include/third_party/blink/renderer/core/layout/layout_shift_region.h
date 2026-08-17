// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_SHIFT_REGION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_SHIFT_REGION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

// Represents a per-frame layout shift region for LayoutShiftTracker.
//
// This class uses a sweep line algorithm to compute the area in O(n log n) time
// where n is the number of rects recorded by AddRect. For complex layout shift
// regions, this is more efficient than using cc::Region, which is worst-case
// O(n^2) from the repeated calls to cc::Region::Union.
//
// The high-level approach is described here:
// http://jeffe.cs.illinois.edu/open/klee.html
//
// The sweep line moves from left to right. (TODO: compare performance against a
// top-to-bottom sweep.)
//
// The sweep line's current intersection with the layout shift region ("active
// length") is tracked by a segment tree, similar to what is described at:
// https://en.wikipedia.org/wiki/Segment_tree
//
// There are some subtleties to the segment tree, which are described by the
// comments in the implementation.

class CORE_EXPORT LayoutShiftRegion {
  DISALLOW_NEW();

 public:
  void AddRect(const gfx::Rect& rect) {
    if (!rect.IsEmpty())
      rects_.push_back(rect);
  }

  const Vector<gfx::Rect>& GetRects() const { return rects_; }
  bool IsEmpty() const { return rects_.empty(); }
  void Reset() { rects_.clear(); }

  uint64_t Area() const;

 private:
  Vector<gfx::Rect> rects_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_SHIFT_REGION_H_
