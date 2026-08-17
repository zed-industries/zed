// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVER_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVER_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class LocalDOMWindow;
class ResizeObserver;

// ResizeObserverController keeps track of all ResizeObservers
// in a single window.
//
// The observation API is used to integrate ResizeObserver
// and the event loop. It delivers notification in a loop.
// In each iteration, only notifications deeper than the
// shallowest notification from previous iteration are delivered.
class CORE_EXPORT ResizeObserverController final
    : public GarbageCollected<ResizeObserverController>,
      public Supplement<LocalDOMWindow> {
 public:
  static const size_t kDepthBottom = 4096;

  static const char kSupplementName[];
  static ResizeObserverController* From(LocalDOMWindow&);
  static ResizeObserverController* FromIfExists(LocalDOMWindow&);

  explicit ResizeObserverController(LocalDOMWindow&);

  void AddObserver(ResizeObserver&);

  // observation API
  // Returns min depth of shallowest observed node, kDepthLimit if none.
  size_t GatherObservations();
  // Returns true if gatherObservations has skipped observations
  // because they were too shallow.
  bool SkippedObservations();
  void DeliverObservations();
  void ClearObservations();

  void ClearMinDepth() { min_depth_ = 0; }

  bool IsLoopLimitErrorDispatched() const {
    return loop_limit_error_dispatched;
  }
  void SetLoopLimitErrorDispatched(bool is_dispatched) {
    loop_limit_error_dispatched = is_dispatched;
  }

  void Trace(Visitor*) const override;

  // For testing only.
  const HeapLinkedHashSet<WeakMember<ResizeObserver>>& Observers() {
    return observers_;
  }

 private:
  // Active observers
  HeapLinkedHashSet<WeakMember<ResizeObserver>> observers_;
  // Minimum depth for observations to be active
  size_t min_depth_ = 0;
  // Used to prevent loop limit errors from being dispatched twice for the
  // same lifecycle update
  bool loop_limit_error_dispatched = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVER_CONTROLLER_H_
