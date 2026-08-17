// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BOX_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BOX_UTILS_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"

namespace blink {

class BlockBreakToken;
class DeprecatedLayoutPoint;
class LayoutBox;
class PhysicalBoxFragment;

// The inline-size of the first fragment.
LayoutUnit BoxInlineSize(const LayoutBox& box);

// The total block-size of all fragments.
LayoutUnit BoxTotalBlockSize(const LayoutBox& box);

// Convert a physical offset for a physical fragment to a physical legacy
// DeprecatedLayoutPoint, to be used in LayoutBox. There are special
// considerations for vertical-rl writing-mode, and also for block
// fragmentation (the block-offset should include consumed space in previous
// fragments).
DeprecatedLayoutPoint ComputeBoxLocation(
    const PhysicalBoxFragment& child_fragment,
    PhysicalOffset offset,
    const PhysicalBoxFragment& container_fragment,
    const BlockBreakToken* previous_container_break_token);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BOX_UTILS_H_
