// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVATION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/resize_observer/resize_observer_box_options.h"
#include "third_party/blink/renderer/core/resize_observer/resize_observer_entry.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Element;
class ResizeObserver;

// ResizeObservation represents an element that is being observed.
class CORE_EXPORT ResizeObservation final
    : public GarbageCollected<ResizeObservation> {
 public:
  ResizeObservation(Element* target,
                    ResizeObserver*,
                    ResizeObserverBoxOptions observed_box);

  Element* Target() const { return target_.Get(); }
  size_t TargetDepth();
  // True if observation_size_ differs from target's current size.
  bool ObservationSizeOutOfSync();
  void SetObservationSize(const LogicalSize&);
  ResizeObserverBoxOptions ObservedBox() const { return observed_box_; }

  LogicalSize ComputeTargetSize() const;

  void Trace(Visitor*) const;

 private:
  WeakMember<Element> target_;
  Member<ResizeObserver> observer_;
  // Target size sent in last observation notification.
  LogicalSize observation_size_;
  ResizeObserverBoxOptions observed_box_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVATION_H_
