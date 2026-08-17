// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_NON_OVERFLOWING_SCROLL_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_NON_OVERFLOWING_SCROLL_RANGE_H_

#include "third_party/blink/renderer/core/layout/geometry/scroll_offset_range.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"

namespace blink {

class Element;

// Helper structure for CSS anchor positioning's fallback positioning. Each
// fallback position has a corresponding `NonOverflowingScrollRange`. See
// https://drafts.csswg.org/css-anchor-position-1/#fallback-apply
struct NonOverflowingScrollRange {
  DISALLOW_NEW();

  // The range of the snapshotted scroll offset within which this fallback
  // position's margin box doesn't overflow the scroll-adjusted inset-modified
  // containing block rect.
  PhysicalScrollRange containing_block_range;

  // The default anchor used for the corresponding fallback position.
  Member<const Element> anchor_element;

  // Checks if the given scroll offsets are within the scroll ranges, i.e., if
  // the fallback position's margin box overflows the bounds.
  bool Contains(const PhysicalOffset& anchor_scroll_offset) const {
    return containing_block_range.Contains(anchor_scroll_offset);
  }

  bool operator==(const NonOverflowingScrollRange& other) const {
    return containing_block_range == other.containing_block_range;
  }

  void Trace(Visitor* visitor) const { visitor->Trace(anchor_element); }
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::NonOverflowingScrollRange)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_NON_OVERFLOWING_SCROLL_RANGE_H_
