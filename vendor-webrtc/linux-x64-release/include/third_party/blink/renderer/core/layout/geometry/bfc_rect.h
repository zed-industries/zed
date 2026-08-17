// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_BFC_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_BFC_RECT_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/bfc_offset.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// BfcRect is the position and size of a rect (typically a fragment)
// relative to a block formatting context.
struct CORE_EXPORT BfcRect {
  BfcRect(const BfcOffset& start_offset, const BfcOffset& end_offset)
      : start_offset(start_offset), end_offset(end_offset) {
    DCHECK_GE(end_offset.line_offset, start_offset.line_offset);
    DCHECK_GE(end_offset.block_offset, start_offset.block_offset);
  }

  LayoutUnit LineStartOffset() const { return start_offset.line_offset; }
  LayoutUnit LineEndOffset() const { return end_offset.line_offset; }
  LayoutUnit BlockStartOffset() const { return start_offset.block_offset; }
  LayoutUnit BlockEndOffset() const { return end_offset.block_offset; }

  LayoutUnit BlockSize() const {
    if (end_offset.block_offset == LayoutUnit::Max())
      return LayoutUnit::Max();

    return end_offset.block_offset - start_offset.block_offset;
  }
  LayoutUnit InlineSize() const {
    if (end_offset.line_offset == LayoutUnit::Max()) {
      return start_offset.line_offset == LayoutUnit::Max() ? LayoutUnit()
                                                           : LayoutUnit::Max();
    }

    return end_offset.line_offset - start_offset.line_offset;
  }

  bool operator==(const BfcRect& other) const {
    return start_offset == other.start_offset && end_offset == other.end_offset;
  }

  bool operator!=(const BfcRect& other) const { return !(*this == other); }

  WTF::String ToString() const;

  BfcOffset start_offset;
  BfcOffset end_offset;
};

CORE_EXPORT std::ostream& operator<<(std::ostream& os, const BfcRect& rect);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_BFC_RECT_H_
