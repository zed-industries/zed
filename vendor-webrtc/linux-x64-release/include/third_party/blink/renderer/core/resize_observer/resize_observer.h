// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVER_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_resize_observer_box_options.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/resize_observer/resize_observer_box_options.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Element;
class LocalDOMWindow;
class ResizeObserverController;
class ResizeObserverEntry;
class ResizeObservation;
class ResizeObserverOptions;
class ScriptState;
class V8ResizeObserverCallback;

// ResizeObserver represents ResizeObserver javascript api:
// https://github.com/WICG/ResizeObserver/
class CORE_EXPORT ResizeObserver final
    : public ScriptWrappable,
      public ActiveScriptWrappable<ResizeObserver>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class DeliveryTime {
    kInsertionOrder,
    kBeforeOthers,
  };

  // This delegate is an internal (non-web-exposed) version of ResizeCallback.
  class Delegate : public GarbageCollected<Delegate> {
   public:
    virtual ~Delegate() = default;
    virtual void OnResize(
        const HeapVector<Member<ResizeObserverEntry>>& entries) = 0;
    virtual void Trace(Visitor* visitor) const {}
    virtual DeliveryTime Delivery() const {
      return DeliveryTime::kInsertionOrder;
    }
    virtual bool SkipNonAtomicInlineObservations() const { return false; }
  };

  static ResizeObserver* Create(ScriptState*, V8ResizeObserverCallback*);
  static ResizeObserver* Create(LocalDOMWindow*, Delegate*);

  ResizeObserver(V8ResizeObserverCallback*, LocalDOMWindow*);
  ResizeObserver(Delegate*, LocalDOMWindow*);
  ~ResizeObserver() override = default;

  // API methods
  void observe(Element*, const ResizeObserverOptions* options);
  void observe(Element*);
  void unobserve(Element*);
  void disconnect();

  // Returns depth of shallowest observed node, kDepthLimit if none.
  size_t GatherObservations(size_t deeper_than);
  bool SkippedObservations() { return skipped_observations_; }
  void DeliverObservations();
  void ClearObservations();

  ResizeObserverBoxOptions V8EnumToBoxOptions(
      V8ResizeObserverBoxOptions::Enum box_options);

  // ScriptWrappable override:
  bool HasPendingActivity() const override;

  void Trace(Visitor*) const override;

  DeliveryTime Delivery() const {
    return delegate_ ? delegate_->Delivery() : DeliveryTime::kInsertionOrder;
  }
  bool SkipNonAtomicInlineObservations() const {
    return delegate_ && delegate_->SkipNonAtomicInlineObservations();
  }

 private:
  void observeInternal(Element* target, ResizeObserverBoxOptions box_option);

  using ObservationList = HeapLinkedHashSet<WeakMember<ResizeObservation>>;

  // Either of |callback_| and |delegate_| should be non-null.
  const Member<V8ResizeObserverCallback> callback_;
  const Member<Delegate> delegate_;

  // List of Elements we are observing
  ObservationList observations_;
  // List of elements that have changes
  HeapVector<Member<ResizeObservation>> active_observations_;
  // True if observations were skipped gatherObservations
  bool skipped_observations_;

  WeakMember<ResizeObserverController> controller_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVER_H_
