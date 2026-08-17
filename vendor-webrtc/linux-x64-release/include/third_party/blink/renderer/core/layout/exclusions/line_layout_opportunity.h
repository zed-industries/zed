// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_LINE_LAYOUT_OPPORTUNITY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_LINE_LAYOUT_OPPORTUNITY_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This struct represents a 1D-area where a line can fit. It is only used for
// representing the inline size.
struct CORE_EXPORT LineLayoutOpportunity {
  STACK_ALLOCATED();

 public:
  LineLayoutOpportunity() = default;
  explicit LineLayoutOpportunity(LayoutUnit inline_size)
      : line_right_offset(inline_size), float_line_right_offset(inline_size) {}
  LineLayoutOpportunity(LayoutUnit line_left_offset,
                        LayoutUnit line_right_offset,
                        LayoutUnit float_line_left_offset,
                        LayoutUnit float_line_right_offset,
                        LayoutUnit bfc_block_offset,
                        LayoutUnit line_block_size)
      : line_left_offset(line_left_offset),
        line_right_offset(line_right_offset),
        float_line_left_offset(float_line_left_offset),
        float_line_right_offset(float_line_right_offset),
        bfc_block_offset(bfc_block_offset),
        line_block_size(line_block_size) {}

  // The available inline-size of the line, taking shapes into account. Both
  // offsets are relative to the BFC coordinate system.
  LayoutUnit line_left_offset;
  LayoutUnit line_right_offset;

  // The available inline-size of the line *for floats*. This is the same size
  // as the layout opportunity which generated this object. Both offsets are
  // relative to the BFC coordinate system.
  LayoutUnit float_line_left_offset;
  LayoutUnit float_line_right_offset;

  LayoutUnit bfc_block_offset;

  // The block-size this line layout opportunity was created with. Should
  // *only* be used for re-querying for a new line layout opportunity with the
  // same block-size.
  LayoutUnit line_block_size;

  LayoutUnit AvailableInlineSize() const {
    DCHECK_GE(line_right_offset, line_left_offset);
    return line_right_offset - line_left_offset;
  }

  LayoutUnit AvailableFloatInlineSize() const {
    DCHECK_GE(float_line_right_offset, float_line_left_offset);
    return float_line_right_offset - float_line_left_offset;
  }

  bool IsEqualToAvailableFloatInlineSize(LayoutUnit inline_size) const {
    DCHECK_GE(float_line_right_offset, float_line_left_offset);
    // Compare |line_right| isntead of |inline_size| to avoid returning |false|
    // when |line_left + inline_size| exceeds |LayoutUnit::Max| and clamped.
    return float_line_left_offset + inline_size == float_line_right_offset;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_LINE_LAYOUT_OPPORTUNITY_H_
