// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_REGISTRY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/dom/abort_signal_composition_type.h"
#include "third_party/blink/renderer/core/dom/events/event_listener.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {
class EventListener;
class ExecutionContext;

// `AbortSignalRegistry` manages the lifetime of `AbortSignal::AlgorithmHandle`s
// associated with `EventListener`s.
class CORE_EXPORT AbortSignalRegistry
    : public GarbageCollected<AbortSignalRegistry>,
      public Supplement<ExecutionContext>,
      public ExecutionContextLifecycleObserver {
 public:
  static const char kSupplementName[];

  static AbortSignalRegistry* From(ExecutionContext&);

  explicit AbortSignalRegistry(ExecutionContext&);
  AbortSignalRegistry(const AbortSignalRegistry&) = delete;
  AbortSignalRegistry& operator=(const AbortSignalRegistry&) = delete;
  ~AbortSignalRegistry() override;

  // Registers and stores a strong reference to the handle, tying the lifetime
  // of the handle to the lifetime of the event listener.
  void RegisterAbortAlgorithm(EventListener*, AbortSignal::AlgorithmHandle*);

  // Registers and stores a strong reference to the signal for the given type.
  // Does nothing if the signal is already registered.
  void RegisterSignal(const AbortSignal&, AbortSignalCompositionType);
  // Unregisters the signal for the given type. Does nothing if the signal is
  // already registered.
  void UnregisterSignal(const AbortSignal&, AbortSignalCompositionType);

  void Trace(Visitor*) const override;
  void ContextDestroyed() override;

 private:
  // Map holding abort algorithm handlers for event listeners that have them,
  // tying the lifetime of the abort algorithm to the `EventListener`. This is
  // cleared when context is destroyed since we won't run event listeners after
  // detach for targets in the detached context.
  HeapHashMap<WeakMember<EventListener>,
              Member<const AbortSignal::AlgorithmHandle>>
      event_listener_signals_;

  // These sets are similarly cleared on detach, and individual signals are
  // removed when they're settled (can no longer fire relevant events).
  HeapHashSet<Member<const AbortSignal>> signals_registered_for_abort_;
  HeapHashSet<Member<const AbortSignal>> signals_registered_for_priority_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_REGISTRY_H_
