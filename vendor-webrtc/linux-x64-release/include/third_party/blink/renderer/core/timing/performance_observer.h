// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_OBSERVER_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExecutionContext;
class ExceptionState;
class Performance;
class PerformanceObserver;
class PerformanceObserverInit;
class V8PerformanceObserverCallback;

using PerformanceEntryVector = HeapVector<Member<PerformanceEntry>>;

class CORE_EXPORT PerformanceObserver final
    : public ScriptWrappable,
      public ActiveScriptWrappable<PerformanceObserver>,
      public ExecutionContextLifecycleStateObserver {
  DEFINE_WRAPPERTYPEINFO();
  friend class Performance;
  friend class PerformanceTest;
  friend class PerformanceObserverTest;

 public:
  static PerformanceObserver* Create(ScriptState*,
                                     V8PerformanceObserverCallback*);
  static Vector<AtomicString> supportedEntryTypes(ScriptState*);
  static constexpr DOMHighResTimeStamp kDefaultDurationThreshold = 104;

  PerformanceObserver(ExecutionContext*,
                      Performance*,
                      V8PerformanceObserverCallback*);

  void observe(ScriptState*, const PerformanceObserverInit*, ExceptionState&);
  void disconnect();
  PerformanceEntryVector takeRecords();
  void EnqueuePerformanceEntry(PerformanceEntry&);
  PerformanceEntryTypeMask FilterOptions() const { return filter_options_; }
  bool CanObserve(const PerformanceEntry&) const;
  bool RequiresDroppedEntries() const { return requires_dropped_entries_; }
  bool IncludeSoftNavigationObservations() const {
    return include_soft_navigation_observations_;
  }

  // ScriptWrappable
  bool HasPendingActivity() const final;

  void ContextLifecycleStateChanged(mojom::FrameLifecycleState) final;
  void ContextDestroyed() final {}

  void Trace(Visitor*) const override;

 private:
  // This describes the types of parameters that an observer can have in its
  // observe() function. An observer of type kEntryTypesObserver has already
  // made a call observe({entryTypes...}) so can only do subsequent observe()
  // calls with the 'entryTypes' parameter. An observer of type kTypeObserver
  // has already made a call observe({type...}) so it can only perform
  // subsequent observe() calls with the 'type' parameter. An observer of type
  // kUnknown has not called observe().
  enum class PerformanceObserverType {
    kEntryTypesObserver,
    kTypeObserver,
    kUnknown,
  };
  // Deliver the PerformanceObserverCallback. Receives the number of dropped
  // entries to be passed to the callback.
  void Deliver(std::optional<int> dropped_entries_count);

  static PerformanceEntryType supportedEntryTypeMask(ScriptState*);

  Member<V8PerformanceObserverCallback> callback_;
  WeakMember<Performance> performance_;
  PerformanceEntryVector performance_entries_;
  PerformanceEntryTypeMask filter_options_;
  PerformanceObserverType type_;
  bool is_registered_;
  bool requires_dropped_entries_ = false;
  bool include_soft_navigation_observations_ = false;
  // PerformanceEventTiming entries with a duration that is as long as this
  // threshold are regarded as long-latency events by the Event Timing API.
  // Shorter-latency events are ignored. Default value can be overriden via a
  // call to observe().
  DOMHighResTimeStamp duration_threshold_ = kDefaultDurationThreshold;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_OBSERVER_H_
