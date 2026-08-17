// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_LOGICAL_OFFSET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_LOGICAL_OFFSET_H_

#include <tuple>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/text/writing_direction_mode.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

struct LogicalDelta;
struct PhysicalSize;

// LogicalOffset is the position of a rect (typically a fragment) relative to
// its parent rect in the logical coordinate system.
// For more information about physical and logical coordinate systems, see:
// https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/core/layout/README.md#coordinate-spaces
struct CORE_EXPORT LogicalOffset {
  constexpr LogicalOffset() = default;
  constexpr LogicalOffset(LayoutUnit inline_offset, LayoutUnit block_offset)
      : inline_offset(inline_offset), block_offset(block_offset) {}

  // This is deleted to avoid unwanted lossy conversion from float or double to
  // LayoutUnit or int. Use explicit LayoutUnit constructor for each parameter
  // instead.
  LogicalOffset(double, double) = delete;

  // For testing only. It's defined in core/testing/core_unit_test_helper.h.
  // 'constexpr' is to let compiler detect incorrect usages in production code.
  constexpr LogicalOffset(int inline_offset, int block_offset);

  LayoutUnit inline_offset;
  LayoutUnit block_offset;

  // Converts a logical offset to a physical offset. See:
  // https://drafts.csswg.org/css-writing-modes-3/#logical-to-physical
  // PhysicalOffset will be the physical top left point of the rectangle
  // described by offset + inner_size. Setting inner_size to 0,0 will return
  // the same point.
  // @param outer_size the size of the rect (typically a fragment).
  // @param inner_size the size of the inner rect (typically a child fragment).
  PhysicalOffset ConvertToPhysical(WritingDirectionMode writing_direction,
                                   PhysicalSize outer_size,
                                   PhysicalSize inner_size) const;

  constexpr bool operator==(const LogicalOffset& other) const = default;

  LogicalOffset operator+(const LogicalOffset& other) const {
    return {inline_offset + other.inline_offset,
            block_offset + other.block_offset};
  }

  LogicalOffset& operator+=(const LogicalOffset& other) {
    *this = *this + other;
    return *this;
  }

  LogicalOffset& operator-=(const LogicalOffset& other) {
    inline_offset -= other.inline_offset;
    block_offset -= other.block_offset;
    return *this;
  }

  // We also have +, - operators for LogicalDelta, LogicalSize and
  // LogicalOffset defined in ng_logical_size.h

  bool operator>(const LogicalOffset& other) const {
    return inline_offset > other.inline_offset &&
           block_offset > other.block_offset;
  }
  bool operator>=(const LogicalOffset& other) const {
    return inline_offset >= other.inline_offset &&
           block_offset >= other.block_offset;
  }
  bool operator<(const LogicalOffset& other) const {
    return inline_offset < other.inline_offset &&
           block_offset < other.block_offset;
  }
  bool operator<=(const LogicalOffset& other) const {
    return inline_offset <= other.inline_offset &&
           block_offset <= other.block_offset;
  }

  WTF::String ToString() const;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const LogicalOffset&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_LOGICAL_OFFSET_H_
