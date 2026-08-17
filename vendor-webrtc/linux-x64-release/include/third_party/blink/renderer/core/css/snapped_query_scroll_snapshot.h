// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SNAPPED_QUERY_SCROLL_SNAPSHOT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SNAPPED_QUERY_SCROLL_SNAPSHOT_H_

#include "third_party/blink/renderer/core/scroll/scroll_snapshot_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class Element;
class PaintLayerScrollableArea;

// Created for a scroll snap container that may have descendants that query
// the snapped scroll state for @container queries.
//
// The snapshot state is used to update the ContainerValues for the query
// container so that @container queries with scroll-state(snapped: ...)
// evaluate correctly on the subsequent style updates.
class SnappedQueryScrollSnapshot
    : public GarbageCollected<SnappedQueryScrollSnapshot>,
      public ScrollSnapshotClient {
 public:
  explicit SnappedQueryScrollSnapshot(PaintLayerScrollableArea& scroller);

  Element* GetSnappedTargetX() const { return snapped_target_x_; }
  Element* GetSnappedTargetY() const { return snapped_target_y_; }

  // ScrollSnapshotClient:
  void UpdateSnapshot() override;
  bool ValidateSnapshot() override;
  bool ShouldScheduleNextService() override;

  void Trace(Visitor*) const override;

 private:
  bool UpdateSnappedTargets();
  void InvalidateSnappedTarget(Element* target);

  Member<PaintLayerScrollableArea> scroller_;
  Member<Element> snapped_target_x_;
  Member<Element> snapped_target_y_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SNAPPED_QUERY_SCROLL_SNAPSHOT_H_
