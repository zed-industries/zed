// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_WRITING_MODE_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_WRITING_MODE_CONVERTER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_rect.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/text/writing_direction_mode.h"

namespace blink {

// This class represents CSS property values to convert between logical and
// physical coordinate systems. See:
// https://drafts.csswg.org/css-writing-modes-3/#logical-to-physical
class CORE_EXPORT WritingModeConverter {
  STACK_ALLOCATED();

 public:
  // @param writing_direction |WritingMode| and |TextDirection|.
  // @param outer_size the size of the rect (typically a fragment). Some
  // combinations of |WritingMode| and |TextDirection| require the size of the
  // container to make the offset relative to the right or the bottom edges.
  WritingModeConverter(WritingDirectionMode writing_direction,
                       const PhysicalSize& outer_size)
      : writing_direction_(writing_direction), outer_size_(outer_size) {}

  WritingModeConverter(WritingDirectionMode writing_direction,
                       const LogicalSize& outer_size)
      : writing_direction_(writing_direction),
        outer_size_(
            ToPhysicalSize(outer_size, writing_direction.GetWritingMode())) {}

  // Construct without |outer_size|. Caller should call |SetOuterSize| before
  // conversions.
  explicit WritingModeConverter(WritingDirectionMode writing_direction)
      : writing_direction_(writing_direction) {}

  // Conversion properties and utilities.
  WritingDirectionMode GetWritingDirection() const {
    return writing_direction_;
  }
  WritingMode GetWritingMode() const {
    return writing_direction_.GetWritingMode();
  }
  TextDirection Direction() const { return writing_direction_.Direction(); }
  bool IsLtr() const { return writing_direction_.IsLtr(); }

  const PhysicalSize OuterSize() const { return outer_size_; }
  void SetOuterSize(const PhysicalSize& outer_size) {
    outer_size_ = outer_size;
  }

  // |LogicalOffset| and |PhysicalOffset| conversions.
  // PhysicalOffset will be the physical top left point of the rectangle
  // described by offset + inner_size. Setting inner_size to 0,0 will return
  // the same point.
  // @param inner_size the size of the inner rect (typically a child fragment).
  LogicalOffset ToLogical(const PhysicalOffset& offset,
                          const PhysicalSize& inner_size) const;
  PhysicalOffset ToPhysical(const LogicalOffset& offset,
                            const PhysicalSize& inner_size) const;

  // |LogicalSize| and |PhysicalSize| conversions.
  LogicalSize ToLogical(const PhysicalSize& size) const;
  PhysicalSize ToPhysical(const LogicalSize& size) const;

  // |LogicalRect| and |PhysicalRect| conversions.
  LogicalRect ToLogical(const PhysicalRect& rect) const;
  PhysicalRect ToPhysical(const LogicalRect& rect) const;

  // gfx variants
  gfx::PointF ToLogical(const gfx::PointF& offset) const;
  gfx::SizeF ToLogical(const gfx::SizeF& size) const;
  gfx::RectF ToLogical(const gfx::RectF& rect) const;

 private:
  LogicalOffset SlowToLogical(const PhysicalOffset& offset,
                              const PhysicalSize& inner_size) const;
  PhysicalOffset SlowToPhysical(const LogicalOffset& offset,
                                const PhysicalSize& inner_size) const;

  LogicalRect SlowToLogical(const PhysicalRect& rect) const;
  PhysicalRect SlowToPhysical(const LogicalRect& rect) const;

  gfx::PointF SlowToLogical(const gfx::PointF& offset,
                            const gfx::SizeF& inner_size) const;
  gfx::RectF SlowToLogical(const gfx::RectF& rect) const;

  WritingDirectionMode writing_direction_;
  PhysicalSize outer_size_;
};

inline LogicalOffset WritingModeConverter::ToLogical(
    const PhysicalOffset& offset,
    const PhysicalSize& inner_size) const {
  if (writing_direction_.IsHorizontalLtr())
    return LogicalOffset(offset.left, offset.top);
  return SlowToLogical(offset, inner_size);
}

inline PhysicalOffset WritingModeConverter::ToPhysical(
    const LogicalOffset& offset,
    const PhysicalSize& inner_size) const {
  if (writing_direction_.IsHorizontalLtr())
    return PhysicalOffset(offset.inline_offset, offset.block_offset);
  return SlowToPhysical(offset, inner_size);
}

inline LogicalSize WritingModeConverter::ToLogical(
    const PhysicalSize& size) const {
  return ToLogicalSize(size, GetWritingMode());
}

inline PhysicalSize WritingModeConverter::ToPhysical(
    const LogicalSize& size) const {
  return ToPhysicalSize(size, GetWritingMode());
}

inline LogicalRect WritingModeConverter::ToLogical(
    const PhysicalRect& rect) const {
  if (writing_direction_.IsHorizontalLtr())
    return LogicalRect(rect.X(), rect.Y(), rect.Width(), rect.Height());
  return SlowToLogical(rect);
}

inline PhysicalRect WritingModeConverter::ToPhysical(
    const LogicalRect& rect) const {
  if (writing_direction_.IsHorizontalLtr()) {
    return PhysicalRect(rect.offset.inline_offset, rect.offset.block_offset,
                        rect.size.inline_size, rect.size.block_size);
  }
  return SlowToPhysical(rect);
}

inline gfx::PointF WritingModeConverter::ToLogical(
    const gfx::PointF& offset) const {
  if (writing_direction_.IsHorizontalLtr()) {
    return offset;
  }
  return SlowToLogical(offset, {});
}

inline gfx::SizeF WritingModeConverter::ToLogical(
    const gfx::SizeF& size) const {
  return writing_direction_.IsHorizontal() ? size : gfx::TransposeSize(size);
}

inline gfx::RectF WritingModeConverter::ToLogical(
    const gfx::RectF& rect) const {
  if (writing_direction_.IsHorizontalLtr()) {
    return rect;
  }
  return SlowToLogical(rect);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_WRITING_MODE_CONVERTER_H_
