// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_INTERSECTION_OBSERVER_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_INTERSECTION_OBSERVER_DELEGATE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/intersection_observer/intersection_observer_delegate.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/bindings/scoped_persistent.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class V8IntersectionObserverCallback;

class V8IntersectionObserverDelegate final
    : public IntersectionObserverDelegate,
      public ExecutionContextClient {

 public:
  CORE_EXPORT V8IntersectionObserverDelegate(V8IntersectionObserverCallback*,
                                             ScriptState*);
  ~V8IntersectionObserverDelegate() override;

  ExecutionContext* GetExecutionContext() const override;

  void Trace(Visitor*) const override;

  IntersectionObserver::DeliveryBehavior GetDeliveryBehavior() const override {
    return IntersectionObserver::kPostTaskToDeliver;
  }

  // The IntersectionObserver spec requires that at least one observation be
  // recorded after observe() is called, even if the target is detached.
  bool NeedsInitialObservationWithDetachedTarget() const override {
    return true;
  }

  void Deliver(const HeapVector<Member<IntersectionObserverEntry>>&,
               IntersectionObserver&) override;

 private:
  Member<V8IntersectionObserverCallback> callback_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_INTERSECTION_OBSERVER_DELEGATE_H_
