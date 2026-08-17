// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_LOGICAL_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_LOGICAL_RECT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_offset.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "ui/gfx/geometry/rect_f.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// LogicalRect is the position and size of a rect (typically a fragment)
// relative to the parent in the logical coordinate system.
// For more information about physical and logical coordinate systems, see:
// https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/core/layout/README.md#coordinate-spaces
struct CORE_EXPORT LogicalRect {
  constexpr LogicalRect() = default;
  constexpr LogicalRect(const LogicalOffset& offset, const LogicalSize& size)
      : offset(offset), size(size) {}
  constexpr LogicalRect(LayoutUnit inline_offset,
                        LayoutUnit block_offset,
                        LayoutUnit inline_size,
                        LayoutUnit block_size)
      : offset(inline_offset, block_offset), size(inline_size, block_size) {}

  // This is deleted to avoid unwanted lossy conversion from float or double to
  // LayoutUnit or int. Use explicit LayoutUnit constructor for each parameter
  // instead.
  LogicalRect(double, double, double, double) = delete;

  // For testing only. It's defined in core/testing/core_unit_test_helper.h.
  // 'constexpr' is to let compiler detect usage from production code.
  constexpr LogicalRect(int inline_offset,
                        int block_offset,
                        int inline_size,
                        int block_size);

  LogicalOffset offset;
  LogicalSize size;

  constexpr bool IsEmpty() const { return size.IsEmpty(); }

  constexpr LayoutUnit InlineStartOffset() const {
    return offset.inline_offset;
  }
  constexpr LayoutUnit BlockStartOffset() const { return offset.block_offset; }
  constexpr LayoutUnit InlineSize() const { return size.inline_size; }
  constexpr LayoutUnit BlockSize() const { return size.block_size; }

  LayoutUnit InlineEndOffset() const {
    return offset.inline_offset + size.inline_size;
  }
  LayoutUnit BlockEndOffset() const {
    return offset.block_offset + size.block_size;
  }
  LogicalOffset EndOffset() const { return offset + size; }

  constexpr bool operator==(const LogicalRect& other) const = default;

  LogicalRect operator+(const LogicalOffset& additional_offset) const {
    return {offset + additional_offset, size};
  }

  void Unite(const LogicalRect&);
  void UniteEvenIfEmpty(const LogicalRect&);

  // Shift up the inline-start edge and the block-start by `d`, and shift down
  // the inline-end edge and the block-end edge by `d`.
  void Inflate(LayoutUnit d) {
    offset.inline_offset -= d;
    size.inline_size += d * 2;
    offset.block_offset -= d;
    size.block_size += d * 2;
  }

  // Shift up the inline-start edge by `inline_start`, shift up the block-start
  // edge by `block_start`, shift down the inline-end edge by `inline_end`, and
  // shift down the block-end edge by `block_end`.
  void ExpandEdges(LayoutUnit block_start,
                   LayoutUnit inline_end,
                   LayoutUnit block_end,
                   LayoutUnit inline_start) {
    offset.inline_offset -= inline_start;
    offset.block_offset -= block_start;
    size.inline_size += inline_start + inline_end;
    size.block_size += block_start + block_end;
  }

  void ContractEdges(LayoutUnit block_start,
                     LayoutUnit inline_end,
                     LayoutUnit block_end,
                     LayoutUnit inline_start) {
    ExpandEdges(-block_start, -inline_end, -block_end, -inline_start);
  }

  // Update inline-start offset without changing the inline-end offset.
  void ShiftInlineStartEdgeTo(LayoutUnit edge) {
    LayoutUnit new_size = (InlineEndOffset() - edge).ClampNegativeToZero();
    offset.inline_offset = edge;
    size.inline_size = new_size;
  }

  // Update block-start offset without changing the block-end offset.
  void ShiftBlockStartEdgeTo(LayoutUnit edge) {
    LayoutUnit new_block_size = (BlockEndOffset() - edge).ClampNegativeToZero();
    offset.block_offset = edge;
    size.block_size = new_block_size;
  }

  // Update inline-end offset without changing the inline-start offset.
  void ShiftInlineEndEdgeTo(LayoutUnit edge) {
    size.inline_size = (edge - offset.inline_offset).ClampNegativeToZero();
  }

  // Update block-end offset without changing the block-start offset.
  void ShiftBlockEndEdgeTo(LayoutUnit edge) {
    size.block_size = (edge - offset.block_offset).ClampNegativeToZero();
  }

  // You can use this function only if we know `rect` is logical. See also:
  //  * `PhysicalRect::EnclosingRect() -> PhysicalRect`
  static LogicalRect EnclosingRect(const gfx::RectF& rect) {
    const LogicalOffset offset(LayoutUnit::FromFloatFloor(rect.x()),
                               LayoutUnit::FromFloatFloor(rect.y()));
    const LogicalSize size(
        LayoutUnit::FromFloatCeil(rect.right()) - offset.inline_offset,
        LayoutUnit::FromFloatCeil(rect.bottom()) - offset.block_offset);
    return LogicalRect(offset, size);
  }

  explicit LogicalRect(const gfx::Rect& r)
      : offset(LayoutUnit(r.x()), LayoutUnit(r.y())),
        size(LayoutUnit(r.width()), LayoutUnit(r.height())) {}

  WTF::String ToString() const;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const LogicalRect&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_LOGICAL_RECT_H_
