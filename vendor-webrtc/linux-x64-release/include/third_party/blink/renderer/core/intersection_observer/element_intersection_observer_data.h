// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_ELEMENT_INTERSECTION_OBSERVER_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_ELEMENT_INTERSECTION_OBSERVER_DATA_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class IntersectionObservation;
class IntersectionObserver;
class IntersectionObserverController;

class CORE_EXPORT ElementIntersectionObserverData final
    : public GarbageCollected<ElementIntersectionObserverData>,
      public NameClient,
      public ElementRareDataField {
 public:
  ElementIntersectionObserverData();
  ~ElementIntersectionObserverData() final = default;

  // If the argument observer is observing this Element, this method will return
  // the observation.
  IntersectionObservation* GetObservationFor(IntersectionObserver&);

  // Add an implicit-root observation with this element as target.
  void AddObservation(IntersectionObservation&);
  // Add an explicit-root observer with this element as root.
  void AddObserver(IntersectionObserver&);
  void RemoveObservation(IntersectionObservation&);
  void RemoveObserver(IntersectionObserver&);
  bool IsEmpty() const { return observations_.empty() && observers_.empty(); }
  void TrackWithController(IntersectionObserverController&);
  void StopTrackingWithController(IntersectionObserverController&);

  // Run the IntersectionObserver algorithm for all observations for which this
  // element is target.
  void ComputeIntersectionsForTarget();
  bool NeedsOcclusionTracking() const;

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override {
    return "ElementIntersectionObserverData";
  }

 private:
  // IntersectionObservations for which the Node owning this data is target.
  HeapHashMap<Member<IntersectionObserver>, Member<IntersectionObservation>>
      observations_;
  // IntersectionObservers for which the Node owning this data is root.
  // Weak because once an observer is unreachable from javascript and has no
  // active observations, it should be allowed to die.
  HeapHashSet<WeakMember<IntersectionObserver>> observers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_ELEMENT_INTERSECTION_OBSERVER_DATA_H_
