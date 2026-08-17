/*
 * Copyright (C) 2012 Nokia Corporation and/or its subsidiary(-ies)
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_TOUCH_ADJUSTMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_TOUCH_ADJUSTMENT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "ui/gfx/geometry/point.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class Node;
class LocalFrame;

enum class TouchAdjustmentCandidateType {
  kClickable,
  kContextMenu,
  kStylusWritable
};

// Finds the best `candidate_node` and location `candidate_point` for touch
// adjustment for the given `candidate_type`.
bool FindBestTouchAdjustmentCandidate(
    TouchAdjustmentCandidateType candidate_type,
    Node*& candidate_node,
    gfx::Point& candidate_point,
    const gfx::Point& touch_hotspot,
    const gfx::Rect& touch_area,
    const HeapVector<Member<Node>>&);

// Applies an upper bound to the touch area as the adjustment rect. The
// touch_area is in root frame coordinates, which is in physical pixel when
// zoom-for-dsf is enabled, otherwise in dip (when page scale is 1).
CORE_EXPORT PhysicalSize
GetHitTestRectForAdjustment(LocalFrame& frame, const PhysicalSize& touch_area);

struct TouchAdjustmentResult {
  uint32_t unique_event_id;
  gfx::PointF adjusted_point;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_TOUCH_ADJUSTMENT_H_
