// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_SNAP_COORDINATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_SNAP_COORDINATOR_H_

#include "cc/input/scroll_snap_data.h"
#include "cc/input/snap_selection_strategy.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Element;
class LayoutBox;

// Snap Coordinator keeps track of snap containers and all of their associated
// snap areas.
//
// Snap container:
//   A scroll container that has 'scroll-snap-type' value other
//   than 'none'.
//   However, we maintain a snap container entry for a scrollable area even if
//   its snap type is 'none'. This is because while the scroller does not snap,
//   it still captures the snap areas in its subtree.
// Snap area:
//   A snap container's descendant that contributes snap positions. An element
//   only contributes snap positions to its nearest ancestor (on the element’s
//   containing block chain) scroll container.
//
// For more information see spec: https://drafts.csswg.org/css-snappoints/
class CORE_EXPORT SnapCoordinator final {
  STATIC_ONLY(SnapCoordinator);

 public:
  // Calculate the SnapAreaData for the specific snap area in its snap
  // container.
  static cc::SnapAreaData CalculateSnapAreaData(
      Element& snap_area,
      const LayoutBox& snap_container);

  // Returns true if the SnapContainerData actually changed.
  static bool UpdateSnapContainerData(LayoutBox&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_SNAP_COORDINATOR_H_
