// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_OBSERVER_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_OBSERVER_DELEGATE_H_

#include "third_party/blink/renderer/core/intersection_observer/intersection_observer.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExecutionContext;
class IntersectionObserver;
class IntersectionObserverEntry;

class IntersectionObserverDelegate
    : public GarbageCollected<IntersectionObserverDelegate>,
      public NameClient {
 public:
  ~IntersectionObserverDelegate() override = default;

  virtual IntersectionObserver::DeliveryBehavior GetDeliveryBehavior()
      const = 0;

  // The IntersectionObserver spec requires that at least one observation be
  // recorded after observe() is called, even if the target is detached.
  virtual bool NeedsInitialObservationWithDetachedTarget() const = 0;

  virtual void Deliver(const HeapVector<Member<IntersectionObserverEntry>>&,
                       IntersectionObserver&) = 0;
  virtual ExecutionContext* GetExecutionContext() const = 0;
  virtual void Trace(Visitor* visitor) const {}
  const char* NameInHeapSnapshot() const override {
    return "IntersectionObserverDelegate";
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_OBSERVER_DELEGATE_H_
