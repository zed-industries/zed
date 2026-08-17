// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_BOX_STRUT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_BOX_STRUT_H_

#include <utility>

#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/box_sides.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_offset.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "ui/gfx/geometry/outsets_f.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

struct LineBoxStrut;
struct LogicalRect;
struct LogicalSize;
struct PhysicalBoxStrut;
struct PhysicalRect;
struct PhysicalSize;

// This struct is used for storing margins, borders or padding of a box on all
// four edges.
struct CORE_EXPORT BoxStrut {
  BoxStrut() = default;
  BoxStrut(LayoutUnit inline_start,
           LayoutUnit inline_end,
           LayoutUnit block_start,
           LayoutUnit block_end)
      : inline_start(inline_start),
        inline_end(inline_end),
        block_start(block_start),
        block_end(block_end) {}
  BoxStrut(const LineBoxStrut&, bool is_flipped_lines);

  // Create a strut based on an inner rectangle positioned within an area.
  BoxStrut(const LogicalSize& outer_size, const LogicalRect& inner_rect);

  // Update each of data members with std::min(this->member, other.member).
  // This function returns `*this`.
  BoxStrut& Intersect(const BoxStrut& other);

  LayoutUnit LineLeft(TextDirection direction) const {
    return IsLtr(direction) ? inline_start : inline_end;
  }
  LayoutUnit LineRight(TextDirection direction) const {
    return IsLtr(direction) ? inline_end : inline_start;
  }

  LayoutUnit InlineSum() const { return inline_start + inline_end; }
  LayoutUnit BlockSum() const { return block_start + block_end; }

  LogicalOffset StartOffset() const { return {inline_start, block_start}; }

  bool IsEmpty() const { return *this == BoxStrut(); }

  inline PhysicalBoxStrut ConvertToPhysical(WritingDirectionMode) const;

  // The following two operators exist primarily to have an easy way to access
  // the sum of border and padding.
  BoxStrut& operator+=(const BoxStrut& other) {
    inline_start += other.inline_start;
    inline_end += other.inline_end;
    block_start += other.block_start;
    block_end += other.block_end;
    return *this;
  }

  BoxStrut operator+(const BoxStrut& other) const {
    BoxStrut result(*this);
    result += other;
    return result;
  }

  BoxStrut& operator-=(const BoxStrut& other) {
    inline_start -= other.inline_start;
    inline_end -= other.inline_end;
    block_start -= other.block_start;
    block_end -= other.block_end;
    return *this;
  }

  BoxStrut operator-(const BoxStrut& other) const {
    BoxStrut result(*this);
    result -= other;
    return result;
  }

  bool operator==(const BoxStrut& other) const {
    return std::tie(other.inline_start, other.inline_end, other.block_start,
                    other.block_end) ==
           std::tie(inline_start, inline_end, block_start, block_end);
  }
  bool operator!=(const BoxStrut& other) const { return !(*this == other); }

  WTF::String ToString() const;

  LayoutUnit inline_start;
  LayoutUnit inline_end;
  LayoutUnit block_start;
  LayoutUnit block_end;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const BoxStrut&);

// A variant of BoxStrut in the line-relative coordinate system.
//
// 'line-over' is 'block-start' and 'line-under' is 'block-end' unless it is in
// flipped-lines writing-mode (i.e., 'vertical-lr'), in which case they are
// swapped.
//
// https://drafts.csswg.org/css-writing-modes-3/#line-mappings
struct CORE_EXPORT LineBoxStrut {
  LineBoxStrut() = default;
  LineBoxStrut(LayoutUnit inline_start,
               LayoutUnit inline_end,
               LayoutUnit line_over,
               LayoutUnit line_under)
      : inline_start(inline_start),
        inline_end(inline_end),
        line_over(line_over),
        line_under(line_under) {}
  LineBoxStrut(const BoxStrut&, bool is_flipped_lines);

  LayoutUnit InlineSum() const { return inline_start + inline_end; }
  LayoutUnit BlockSum() const { return line_over + line_under; }

  bool IsEmpty() const {
    return !inline_start && !inline_end && !line_over && !line_under;
  }

  bool operator==(const LineBoxStrut& other) const {
    return inline_start == other.inline_start &&
           inline_end == other.inline_end && line_over == other.line_over &&
           line_under == other.line_under;
  }

  LayoutUnit inline_start;
  LayoutUnit inline_end;
  LayoutUnit line_over;
  LayoutUnit line_under;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const LineBoxStrut&);

// Struct to store physical dimensions, independent of writing mode and
// direction.
// See https://drafts.csswg.org/css-writing-modes-3/#abstract-box
struct CORE_EXPORT PhysicalBoxStrut {
  PhysicalBoxStrut() = default;
  explicit PhysicalBoxStrut(LayoutUnit value)
      : top(value), right(value), bottom(value), left(value) {}
  PhysicalBoxStrut(LayoutUnit top,
                   LayoutUnit right,
                   LayoutUnit bottom,
                   LayoutUnit left)
      : top(top), right(right), bottom(bottom), left(left) {}

  // Arguments are clamped to [LayoutUnix::Min(), LayoutUnit::Max()].
  PhysicalBoxStrut(int t, int r, int b, int l)
      : top(LayoutUnit(t)),
        right(LayoutUnit(r)),
        bottom(LayoutUnit(b)),
        left(LayoutUnit(l)) {}

  // Create a strut based on an inner rectangle positioned within an area.
  PhysicalBoxStrut(const PhysicalSize& outer_size,
                   const PhysicalRect& inner_rect);

  // Creates new PhysicalBoxStrut instance from the specified `outsets`.
  // A data member of `outsets` is rounded up to the minimum LayoutUnit value
  // which is equal or lager than the data member.
  static PhysicalBoxStrut Enclosing(const gfx::OutsetsF& outsets) {
    return {LayoutUnit::FromFloatCeil(outsets.top()),
            LayoutUnit::FromFloatCeil(outsets.right()),
            LayoutUnit::FromFloatCeil(outsets.bottom()),
            LayoutUnit::FromFloatCeil(outsets.left())};
  }

  PhysicalOffset Offset() const { return {left, top}; }

  void TruncateSides(const PhysicalBoxSides& sides_to_include) {
    top = sides_to_include.top ? top : LayoutUnit();
    bottom = sides_to_include.bottom ? bottom : LayoutUnit();
    left = sides_to_include.left ? left : LayoutUnit();
    right = sides_to_include.right ? right : LayoutUnit();
  }

  // Converts physical dimensions to logical ones per
  // https://drafts.csswg.org/css-writing-modes-3/#logical-to-physical
  BoxStrut ConvertToLogical(WritingDirectionMode writing_direction) const {
    BoxStrut strut;
    switch (writing_direction.GetWritingMode()) {
      case WritingMode::kHorizontalTb:
        strut = {left, right, top, bottom};
        break;
      case WritingMode::kVerticalRl:
      case WritingMode::kSidewaysRl:
        strut = {top, bottom, right, left};
        break;
      case WritingMode::kVerticalLr:
        strut = {top, bottom, left, right};
        break;
      case WritingMode::kSidewaysLr:
        strut = {bottom, top, left, right};
        break;
    }
    if (writing_direction.IsRtl())
      std::swap(strut.inline_start, strut.inline_end);
    return strut;
  }

  // Converts physical dimensions to line-relative logical ones per
  // https://drafts.csswg.org/css-writing-modes-3/#line-directions
  LineBoxStrut ConvertToLineLogical(
      WritingDirectionMode writing_direction) const {
    return LineBoxStrut(ConvertToLogical(writing_direction),
                        writing_direction.IsFlippedLines());
  }

  LayoutUnit HorizontalSum() const { return left + right; }
  LayoutUnit VerticalSum() const { return top + bottom; }

  PhysicalBoxStrut& Inflate(LayoutUnit diff) {
    top += diff;
    right += diff;
    bottom += diff;
    left += diff;
    return *this;
  }

  // Update each of data members with std::max(this->member, other.member).
  // This function returns `*this`.
  PhysicalBoxStrut& Unite(const PhysicalBoxStrut& other);

  PhysicalBoxStrut& operator+=(const PhysicalBoxStrut& other) {
    top += other.top;
    right += other.right;
    bottom += other.bottom;
    left += other.left;
    return *this;
  }

  PhysicalBoxStrut& operator-=(const PhysicalBoxStrut& other) {
    top -= other.top;
    right -= other.right;
    bottom -= other.bottom;
    left -= other.left;
    return *this;
  }

  PhysicalBoxStrut operator+(const PhysicalBoxStrut& other) const {
    PhysicalBoxStrut result(*this);
    result += other;
    return result;
  }

  PhysicalBoxStrut operator-(const PhysicalBoxStrut& other) const {
    PhysicalBoxStrut result(*this);
    result -= other;
    return result;
  }

  bool operator==(const PhysicalBoxStrut& other) const {
    return top == other.top && right == other.right && bottom == other.bottom &&
           left == other.left;
  }

  explicit operator gfx::OutsetsF() const {
    return gfx::OutsetsF()
        .set_left(left.ToFloat())
        .set_right(right.ToFloat())
        .set_top(top.ToFloat())
        .set_bottom(bottom.ToFloat());
  }

  bool IsZero() const { return !top && !right && !bottom && !left; }

  LayoutUnit top;
  LayoutUnit right;
  LayoutUnit bottom;
  LayoutUnit left;
};

inline PhysicalBoxStrut BoxStrut::ConvertToPhysical(
    WritingDirectionMode writing_direction) const {
  LayoutUnit direction_start = inline_start;
  LayoutUnit direction_end = inline_end;
  if (writing_direction.IsRtl())
    std::swap(direction_start, direction_end);
  switch (writing_direction.GetWritingMode()) {
    case WritingMode::kHorizontalTb:
      return PhysicalBoxStrut(block_start, direction_end, block_end,
                              direction_start);
    case WritingMode::kVerticalRl:
    case WritingMode::kSidewaysRl:
      return PhysicalBoxStrut(direction_start, block_start, direction_end,
                              block_end);
    case WritingMode::kVerticalLr:
      return PhysicalBoxStrut(direction_start, block_end, direction_end,
                              block_start);
    case WritingMode::kSidewaysLr:
      return PhysicalBoxStrut(direction_end, block_end, direction_start,
                              block_start);
    default:
      NOTREACHED();
  }
}

inline PhysicalBoxStrut operator-(const PhysicalBoxStrut& a) {
  return {-a.top, -a.right, -a.bottom, -a.left};
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_BOX_STRUT_H_
