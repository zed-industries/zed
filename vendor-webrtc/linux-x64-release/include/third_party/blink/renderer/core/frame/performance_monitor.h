// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PERFORMANCE_MONITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PERFORMANCE_MONITOR_H_

#include "base/task/sequence_manager/task_time_observer.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "v8/include/v8.h"

namespace blink {

namespace probe {
class CallFunction;
class ExecuteScript;
class RecalculateStyle;
class UpdateLayout;
class UserCallback;
class V8Compile;
}  // namespace probe

class DOMWindow;
class Document;
class ExecutionContext;
class Frame;
class LocalFrame;
class WindowPerformance;
class SourceLocation;

// Performance monitor for Web Performance APIs and logging.
// The monitor is maintained per local root.
// Long task notifications are delivered to observing WindowPerformance*
// instances (in the local frame tree) in m_webPerformanceObservers.
class CORE_EXPORT PerformanceMonitor final
    : public GarbageCollected<PerformanceMonitor>,
      public base::sequence_manager::TaskTimeObserver {
  USING_PRE_FINALIZER(PerformanceMonitor, Dispose);

 public:
  enum Violation : size_t {
    kLongTask = 0,
    kLongLayout,
    kBlockedEvent,
    kBlockedParser,
    kDiscouragedAPIUse,
    kHandler,
    kRecurringHandler,
    kAfterLast
  };

  class CORE_EXPORT Client : public GarbageCollectedMixin {
   public:
    virtual void ReportLongTask(base::TimeTicks start_time,
                                base::TimeTicks end_time,
                                ExecutionContext* task_context,
                                bool has_multiple_contexts) {}
    virtual void ReportLongLayout(base::TimeDelta duration) {}
    virtual void ReportGenericViolation(Violation,
                                        const String& text,
                                        base::TimeDelta time,
                                        SourceLocation*) {}
    void Trace(Visitor* visitor) const override {}
  };

  static void ReportGenericViolation(ExecutionContext*,
                                     Violation,
                                     const String& text,
                                     base::TimeDelta time,
                                     std::unique_ptr<SourceLocation>);
  static base::TimeDelta Threshold(ExecutionContext*, Violation);

  // Instrumenting methods.
  void Will(const probe::RecalculateStyle&);
  void Did(const probe::RecalculateStyle&);

  void Will(const probe::UpdateLayout&);
  void Did(const probe::UpdateLayout&);

  void Will(const probe::ExecuteScript&);
  void Did(const probe::ExecuteScript&);

  void Will(const probe::CallFunction&);
  void Did(const probe::CallFunction&);

  void Will(const probe::UserCallback&);
  void Did(const probe::UserCallback&);

  void Will(const probe::V8Compile&);
  void Did(const probe::V8Compile&);

  void DomContentLoadedEventFired(LocalFrame*);

  void DocumentWriteFetchScript(Document*);

  // Direct API for core.
  void Subscribe(Violation, base::TimeDelta threshold, Client*);
  void UnsubscribeAll(Client*);
  void Shutdown();

  PerformanceMonitor(LocalFrame*, v8::Isolate*);
  PerformanceMonitor(const PerformanceMonitor&) = delete;
  PerformanceMonitor& operator=(const PerformanceMonitor&) = delete;
  ~PerformanceMonitor() override;

  void Trace(Visitor*) const;

 private:
  friend class PerformanceMonitorTest;
  friend class WindowPerformanceTest;

  static PerformanceMonitor* Monitor(const ExecutionContext*);
  // Returns the monitor of the ExecutionContext if its
  // |enabled_| is set, nullptr otherwise.
  static PerformanceMonitor* InstrumentingMonitorExcludingLongTasks(
      const ExecutionContext*);

  void UpdateInstrumentation();

  void InnerReportGenericViolation(ExecutionContext*,
                                   Violation,
                                   const String& text,
                                   base::TimeDelta time,
                                   std::unique_ptr<SourceLocation>);

  // TaskTimeObserver implementation
  void WillProcessTask(base::TimeTicks start_time) override;
  void DidProcessTask(base::TimeTicks start_time,
                      base::TimeTicks end_time) override;

  void WillExecuteScript(ExecutionContext*);
  void DidExecuteScript();

  void UpdateTaskAttribution(ExecutionContext*);
  void UpdateTaskShouldBeReported(LocalFrame*);

  std::pair<String, DOMWindow*> SanitizedAttribution(
      const HeapHashSet<Member<Frame>>& frame_contexts,
      Frame* observer_frame);

  void Dispose();

  // This boolean is used to track whether there is any subscription to any
  // Violation other than longtasks.
  bool enabled_ = false;
  base::TimeDelta per_task_style_and_layout_time_;
  unsigned script_depth_ = 0;
  unsigned layout_depth_ = 0;
  unsigned user_callback_depth_ = 0;
  const void* user_callback_;

  std::array<base::TimeDelta, kAfterLast> thresholds_;

  Member<LocalFrame> local_root_;
  Member<ExecutionContext> task_execution_context_;
  // This is needed for calling v8::metrics::LongTaskStats::Reset.
  v8::Isolate* const isolate_;
  bool task_has_multiple_contexts_ = false;
  bool task_should_be_reported_ = false;
  using ClientThresholds = GCedHeapHashMap<WeakMember<Client>, base::TimeDelta>;
  HeapHashMap<Violation,
              Member<ClientThresholds>,
              IntWithZeroKeyHashTraits<size_t>>
      subscriptions_;
  bool was_shutdown_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PERFORMANCE_MONITOR_H_
