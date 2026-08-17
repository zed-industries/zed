// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_SCROLL_OFFSET_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_SCROLL_OFFSET_RANGE_H_

#include <optional>

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/text/writing_direction_mode.h"

namespace blink {

// Represents a range of scroll translation offsets that can be applied to a box
// in both axes. A missing value means an unbounded range in that direction.
struct PhysicalScrollRange {
  std::optional<LayoutUnit> x_min;
  std::optional<LayoutUnit> x_max;
  std::optional<LayoutUnit> y_min;
  std::optional<LayoutUnit> y_max;

  bool Contains(const PhysicalOffset& offset) const {
    return (!x_min || offset.left >= *x_min) &&
           (!x_max || offset.left <= *x_max) &&
           (!y_min || offset.top >= *y_min) && (!y_max || offset.top <= *y_max);
  }

  bool operator==(const PhysicalScrollRange& other) const {
    return x_min == other.x_min && x_max == other.x_max &&
           y_min == other.y_min && y_max == other.y_max;
  }
};

// Similar to PhysicalScrollRange, but using the logical axes of the box.
struct LogicalScrollRange {
  std::optional<LayoutUnit> inline_min;
  std::optional<LayoutUnit> inline_max;
  std::optional<LayoutUnit> block_min;
  std::optional<LayoutUnit> block_max;

  PhysicalScrollRange ToPhysical(WritingDirectionMode mode) const {
    if (mode.IsHorizontalLtr()) {
      return PhysicalScrollRange{inline_min, inline_max, block_min, block_max};
    }
    return SlowToPhysical(mode);
  }

 private:
  PhysicalScrollRange SlowToPhysical(WritingDirectionMode mode) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_SCROLL_OFFSET_RANGE_H_
