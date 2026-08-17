// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_OBSERVER_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_OBSERVER_CONTROLLER_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/intersection_observer/intersection_observer.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

// Design doc for IntersectionObserver implementation:
//   https://docs.google.com/a/google.com/document/d/1hLK0eyT5_BzyNS4OkjsnoqqFQDYCbKfyBinj94OnLiQ

namespace blink {

class ExecutionContext;
class LocalFrameView;

class CORE_EXPORT ComputeIntersectionsContext {
  STACK_ALLOCATED();

 public:
  ~ComputeIntersectionsContext() {
    // GetAndResetNextRunDelay() must have been called.
    CHECK_EQ(next_run_delay_, base::TimeDelta::Max());
  }

  base::TimeTicks GetMonotonicTime();
  DOMHighResTimeStamp GetTimeStamp(const IntersectionObserver& observer);
  std::optional<IntersectionGeometry::RootGeometry>& GetRootGeometry(
      const IntersectionObserver& observer,
      unsigned flags);
  void UpdateNextRunDelay(base::TimeDelta delay);
  base::TimeDelta GetAndResetNextRunDelay();

 private:
  base::TimeTicks monotonic_time_;
  ExecutionContext* explicit_root_execution_context_ = nullptr;
  DOMHighResTimeStamp explicit_root_timestamp_ = -1;
  ExecutionContext* implicit_root_execution_context_ = nullptr;
  DOMHighResTimeStamp implicit_root_timestamp_ = -1;

  const IntersectionObserver* explicit_root_geometry_observer_ = nullptr;
  std::optional<IntersectionGeometry::RootGeometry> explicit_root_geometry_;
  const IntersectionObserver* implicit_root_geometry_observer_ = nullptr;
  std::optional<IntersectionGeometry::RootGeometry> implicit_root_geometry_;

  base::TimeDelta next_run_delay_ = base::TimeDelta::Max();
};

class CORE_EXPORT IntersectionObserverController
    : public GarbageCollected<IntersectionObserverController>,
      public ExecutionContextClient,
      public NameClient {
 public:
  explicit IntersectionObserverController(ExecutionContext*);
  ~IntersectionObserverController() override;

  void ScheduleIntersectionObserverForDelivery(IntersectionObserver&);

  // Immediately deliver all notifications for all observers for which
  // (observer->GetDeliveryBehavior() == behavior).
  void DeliverNotifications(IntersectionObserver::DeliveryBehavior behavior);

  // The flags argument is composed of values from
  // IntersectionObservation::ComputeFlags. They are dirty bits that control
  // whether an IntersectionObserver needs to do any work. The return value
  // communicates whether observer->trackVisibility() is true for any tracked
  // observer.
  bool ComputeIntersections(
      unsigned flags,
      LocalFrameView&,
      gfx::Vector2dF accumulated_scroll_delta_since_last_update,
      ComputeIntersectionsContext&);

  void AddTrackedObserver(IntersectionObserver&);
  void AddTrackedObservation(IntersectionObservation&);
  void RemoveTrackedObserver(IntersectionObserver&);
  void RemoveTrackedObservation(IntersectionObservation&);

  bool NeedsOcclusionTracking() const { return needs_occlusion_tracking_; }

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override {
    return "IntersectionObserverController";
  }

  wtf_size_t GetTrackedObserverCountForTesting() const {
    return tracked_explicit_root_observers_.size();
  }
  wtf_size_t GetTrackedObservationCountForTesting() const;

 private:
  void PostTaskToDeliverNotifications();

  // IntersectionObserver's with a connected explicit root in this document.
  HeapHashSet<Member<IntersectionObserver>> tracked_explicit_root_observers_;
  // IntersectionObservations with an implicit root and connected target in this
  // document, grouped by IntersectionObservers.
  HeapHashMap<Member<IntersectionObserver>,
              HeapHashSet<Member<IntersectionObservation>>>
      tracked_implicit_root_observations_;
  // IntersectionObservers for which this is the execution context of the
  // callback, and with unsent notifications.
  HeapHashSet<Member<IntersectionObserver>> pending_intersection_observers_;
  // This is 'true' if any tracked node is the target of an observer for
  // which observer->trackVisibility() is true.
  bool needs_occlusion_tracking_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_OBSERVER_CONTROLLER_H_
