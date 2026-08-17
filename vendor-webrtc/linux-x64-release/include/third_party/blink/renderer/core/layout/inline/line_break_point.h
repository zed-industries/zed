// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_BREAK_POINT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_BREAK_POINT_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/inline_item_text_index.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

//
// Represents a determined break point.
//
struct CORE_EXPORT LineBreakPoint {
  DISALLOW_NEW();

 public:
  LineBreakPoint() = default;
  LineBreakPoint(const InlineItemTextIndex& offset,
                 const InlineItemTextIndex& end,
                 bool is_hyphenated = false)
      : offset(offset), end(end), is_hyphenated(is_hyphenated) {}
  explicit LineBreakPoint(const InlineItemTextIndex& offset,
                          bool is_hyphenated = false)
      : LineBreakPoint(offset, offset, is_hyphenated) {}

  explicit operator bool() const { return offset.text_offset; }

  bool operator==(const LineBreakPoint& other) const {
    return offset == other.offset && end == other.end &&
           is_hyphenated == other.is_hyphenated;
  }

  // The line breaks before `offset`. The `offset` is also the start of the next
  // line, includes trailing spaces, while `end` doesn't.
  //
  // In the following example, the line should break before `next`:
  // ```
  // end <span> </span> next
  // ```
  // Then `offset` is at `n`, while `end` is at the next space of `end`.
  InlineItemTextIndex offset;
  InlineItemTextIndex end;

  // True when this break point has a hyphen.
  bool is_hyphenated = false;

#if EXPENSIVE_DCHECKS_ARE_ON()
  LayoutUnit line_width;
#endif  // EXPENSIVE_DCHECKS_ARE_ON()
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_BREAK_POINT_H_
