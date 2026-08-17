/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 * Copyright (C) 2012 Intel Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_H_

#include "base/functional/callback_forward.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_function.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/core/timing/performance_navigation_timing.h"
#include "third_party/blink/renderer/core/timing/performance_paint_timing.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/linked_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8-local-handle.h"

namespace base {
class Clock;
class TickClock;
}  // namespace base

namespace blink {

class BackgroundTracingHelper;
class EventCounts;
class ExceptionState;
class ExecutionContext;
class LargestContentfulPaint;
class LayoutShift;
class MemoryInfo;
class MemoryMeasurement;
class Node;
class PerformanceContainerTiming;
class PerformanceElementTiming;
class PerformanceEventTiming;
class PerformanceMark;
class PerformanceMarkOptions;
class PerformanceMeasure;
class PerformanceNavigation;
class PerformanceObserver;
class PerformanceTiming;
class ScriptState;
class ScriptValue;
class SoftNavigationEntry;
class UserTiming;
class V8ObjectBuilder;
class V8UnionDoubleOrString;
class V8UnionPerformanceMeasureOptionsOrString;

using PerformanceEntryVector = HeapVector<Member<PerformanceEntry>>;
using PerformanceEntryDeque = HeapDeque<Member<PerformanceEntry>>;

// Merge two sorted PerformanceEntryVectors in linear time. If a non-null name
// is provided, then items in second_entry_vector will be included in the result
// only if their names match the given name. It is expected (and DCHECKed) that
// first_entry_vector has already been filtered by name.
CORE_EXPORT PerformanceEntryVector
MergePerformanceEntryVectors(const PerformanceEntryVector& first_entry_vector,
                             const PerformanceEntryVector& second_entry_vector,
                             const AtomicString& maybe_name);

class CORE_EXPORT Performance : public EventTarget {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~Performance() override;

  const AtomicString& InterfaceName() const override;

  // Overriden by WindowPerformance but not by WorkerPerformance.
  virtual PerformanceTiming* timing() const;
  virtual PerformanceNavigation* navigation() const;
  virtual MemoryInfo* memory(ScriptState*) const;
  virtual ScriptPromise<MemoryMeasurement> measureUserAgentSpecificMemory(
      ScriptState*,
      ExceptionState& exception_state) const;
  virtual EventCounts* eventCounts();
  virtual std::uint64_t interactionCount() const = 0;
  virtual void PopulateContainerTimingEntries() {}

  // Reduce the resolution to prevent timing attacks. See:
  // http://www.w3.org/TR/hr-time-2/#privacy-security
  // This returns a DOMHighResTimeStamp (double), representing clamped, jittered
  // time in milliseconds. The actual clamping resolution varies based on the
  // provided CrossOriginIsolatedCapability.
  static DOMHighResTimeStamp ClampTimeResolution(
      base::TimeDelta time,
      bool cross_origin_isolated_capability);

  static DOMHighResTimeStamp MonotonicTimeToDOMHighResTimeStamp(
      base::TimeTicks time_origin,
      base::TimeTicks monotonic_time,
      bool allow_negative_value,
      bool cross_origin_isolated_capability);

  // Translate given platform monotonic time in seconds into a high resolution
  // DOMHighResTimeStamp in milliseconds. The result timestamp is relative to
  // document's time origin and has a time resolution that is safe for
  // exposing to web.
  DOMHighResTimeStamp MonotonicTimeToDOMHighResTimeStamp(base::TimeTicks) const;

  DOMHighResTimeStamp now() const;

  // High Resolution Time Level 3 timeOrigin.
  // (https://www.w3.org/TR/hr-time-3/#dom-performance-timeorigin)
  DOMHighResTimeStamp timeOrigin() const;

  // Internal getter method for the time origin value.
  base::TimeTicks GetTimeOriginInternal() const { return time_origin_; }

  // Get all performance entries of the main frame. This is kept until the one
  // with optional filtering options is enabled by default.
  PerformanceEntryVector getEntries();

  // This getBufferedEntriesByType method will return all entries in the buffer
  // regardless of whether they are exposed in the Performance Timeline.
  // getEntriesByType will only return all entries for existing types in
  // PerformanceEntry.IsValidTimelineEntryType.
  PerformanceEntryVector getBufferedEntriesByType(
      const AtomicString& entry_type,
      bool include_triggered_by_soft_navigation = false);

  // Get performance entries of the current frame by type, and optionally,
  // nested same-origin iframes.
  PerformanceEntryVector getEntriesByType(const AtomicString& entry_type);

  // Get performance entries of the current frame by name and/or type, and
  // optionally, nested same-origin iframes.
  PerformanceEntryVector getEntriesByName(
      const AtomicString& name,
      const AtomicString& entry_type = g_null_atom);

  void clearResourceTimings();
  void setResourceTimingBufferSize(unsigned);
  void setBackForwardCacheRestorationBufferSizeForTest(unsigned);
  void setEventTimingBufferSizeForTest(unsigned);

  V8Function* bind(V8Function* inner_function,
                   const ScriptValue this_arg,
                   const HeapVector<ScriptValue>& prepend_arguments);

  V8Function* bind(V8Function* inner_function) {
    return bind(inner_function,
                ScriptValue(inner_function->GetIsolate(),
                            v8::Undefined(inner_function->GetIsolate())),
                HeapVector<ScriptValue>());
  }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(resourcetimingbufferfull,
                                  kResourcetimingbufferfull)

  void AddLongTaskTiming(base::TimeTicks start_time,
                         base::TimeTicks end_time,
                         const AtomicString& name,
                         const AtomicString& container_type,
                         const AtomicString& container_src,
                         const AtomicString& container_id,
                         const AtomicString& container_name);

  // Generates and add a performance entry for the given ResourceTimingInfo.
  void AddResourceTiming(mojom::blink::ResourceTimingInfoPtr,
                         const AtomicString& initiator_type);

  void NotifyNavigationTimingToObservers();

  bool IsContainerTimingBufferFull() const;
  void AddToContainerTimingBuffer(PerformanceContainerTiming&);
  void NotifyObserversOfContainerTiming();

  bool IsElementTimingBufferFull() const;
  void AddToElementTimingBuffer(PerformanceElementTiming&);

  bool IsEventTimingBufferFull() const;
  void AddToEventTimingBuffer(PerformanceEventTiming&);

  bool IsLongAnimationFrameBufferFull() const;

  void AddToLayoutShiftBuffer(LayoutShift&);

  void AddLargestContentfulPaint(LargestContentfulPaint*);

  void AddSoftNavigationToPerformanceTimeline(SoftNavigationEntry*);

  PerformanceMark* mark(ScriptState*,
                        const AtomicString& mark_name,
                        PerformanceMarkOptions* mark_options,
                        ExceptionState&);

  void clearMarks(const AtomicString& mark_name);
  void clearMarks() { return clearMarks(AtomicString()); }

  void AddBackForwardCacheRestoration(base::TimeTicks start_time,
                                      base::TimeTicks pageshow_start_time,
                                      base::TimeTicks pageshow_end_time);


  // This enum is used to index different possible strings for for UMA enum
  // histogram. New enum values can be added, but existing enums must never be
  // renumbered or deleted and reused.
  // This enum should be consistent with MeasureParameterType
  // in tools/metrics/histograms/enums.xml.
  enum class MeasureParameterType {
    kObjectObject = 0,
    // 1 to 8, 13 to 25 are navigation-timing types.
    kUnloadEventStart = 1,
    kUnloadEventEnd = 2,
    kDomInteractive = 3,
    kDomContentLoadedEventStart = 4,
    kDomContentLoadedEventEnd = 5,
    kDomComplete = 6,
    kLoadEventStart = 7,
    kLoadEventEnd = 8,
    kOther = 9,
    kUndefinedOrNull = 10,
    // Intentionally leaves out kNumber = 11 since number has been casted to
    // string when users pass number into the API.
    kUnprovided = 12,
    kNavigationStart = 13,
    kRedirectStart = 14,
    kRedirectEnd = 15,
    kFetchStart = 16,
    kDomainLookupStart = 17,
    kDomainLookupEnd = 18,
    kConnectStart = 19,
    kConnectEnd = 20,
    kSecureConnectionStart = 21,
    kRequestStart = 22,
    kResponseStart = 23,
    kResponseEnd = 24,
    kDomLoading = 25,
    kMaxValue = kDomLoading
  };

  UserTiming& GetUserTiming();
  PerformanceMeasure* measure(ScriptState*,
                              const AtomicString& measure_name,
                              ExceptionState&);

  PerformanceMeasure* measure(
      ScriptState* script_state,
      const AtomicString& measure_name,
      const V8UnionPerformanceMeasureOptionsOrString* start_or_options,
      ExceptionState& exception_state);

  PerformanceMeasure* measure(
      ScriptState* script_state,
      const AtomicString& measure_name,
      const V8UnionPerformanceMeasureOptionsOrString* start_or_options,
      const String& end,
      ExceptionState& exception_state);

  void clearMeasures(const AtomicString& measure_name);
  void clearMeasures() { return clearMeasures(AtomicString()); }

  void UnregisterPerformanceObserver(PerformanceObserver&);
  void RegisterPerformanceObserver(PerformanceObserver&);
  void UpdatePerformanceObserverFilterOptions();
  void ActivateObserver(PerformanceObserver&);
  void SuspendObserver(PerformanceObserver&);

  bool HasObserverFor(PerformanceEntry::EntryType) const;
  // Determine whether a given Node can be exposed via a Web Perf API.
  static bool CanExposeNode(Node*);

  ScriptObject toJSONForBinding(ScriptState*) const;

  enum Metrics { kRecordSwaps = 0, kDoNotRecordSwaps = 1 };

  // Insert a PerformanceEntry into a Vector sorted by StartTime. By Default,
  // record the number of 'swaps' per function call in a histogram.
  void InsertEntryIntoSortedBuffer(PerformanceEntryVector& vector,
                                   PerformanceEntry& entry,
                                   Metrics record);

  void Trace(Visitor*) const override;

  // The caller owns the |clock|.
  void SetClocksForTesting(const base::Clock* clock,
                           const base::TickClock* tick_clock);
  void ResetTimeOriginForTesting(base::TimeTicks time_origin);

  void SetCrossOriginIsolatedCapabilityForTesting(bool is_isolated) {
    cross_origin_isolated_capability_ = is_isolated;
  }

  bool CrossOriginIsolatedCapability() const {
    return cross_origin_isolated_capability_;
  }

  // TODO(https://crbug.com/1457049): remove this once visited links are
  // partitioned.
  bool softNavPaintMetricsSupported() const;

  base::SingleThreadTaskRunner& GetTaskRunner() { return *task_runner_; }

 private:
  PerformanceMeasure* MeasureInternal(
      ScriptState* script_state,
      const AtomicString& measure_name,
      const V8UnionPerformanceMeasureOptionsOrString* start_or_options,
      std::optional<String> end_mark,
      ExceptionState& exception_state);

  PerformanceMeasure* MeasureWithDetail(ScriptState* script_state,
                                        const AtomicString& measure_name,
                                        const V8UnionDoubleOrString* start,
                                        const std::optional<double>& duration,
                                        const V8UnionDoubleOrString* end,
                                        const ScriptValue& detail,
                                        ExceptionState& exception_state);

  void CopySecondaryBuffer();

  PerformanceEntryVector getEntriesByTypeInternal(
      PerformanceEntry::EntryType type,
      const AtomicString& maybe_name = g_null_atom,
      bool include_triggered_by_soft_navigation = false);

  // Get performance entries of the current frame, with an optional name filter.
  PerformanceEntryVector GetEntriesForCurrentFrame(
      const AtomicString& maybe_name = g_null_atom);

  // Get performance entries of the current frame by type, with an optional name
  // filter.
  PerformanceEntryVector GetEntriesByTypeForCurrentFrame(
      const AtomicString& entry_type,
      const AtomicString& maybe_name = g_null_atom);

  void ProcessUserFeatureMark(const PerformanceMarkOptions* mark_options);

 protected:
  Performance(base::TimeTicks time_origin,
              bool cross_origin_isolated_capability,
              scoped_refptr<base::SingleThreadTaskRunner>,
              ExecutionContext* context = nullptr);

  bool CanAddResourceTimingEntry();
  void FireResourceTimingBufferFull(TimerBase*);

  void NotifyObserversOfEntry(PerformanceEntry&) const;
  void NotifyObserversOfContainerEntry(PerformanceEntry&) const;

  void DeliverObservationsTimerFired(TimerBase*);

  // Returns the number of dropped entries for the given integer representing a
  // mask of entry types.
  int GetDroppedEntriesForTypes(PerformanceEntryTypeMask);

  virtual void BuildJSONValue(V8ObjectBuilder&) const;

  void AddPaintTiming(PerformancePaintTiming::PaintType,
                      const DOMPaintTimingInfo& paint_timing_info,
                      bool is_triggered_by_soft_navigation);

  PerformanceEntryVector resource_timing_buffer_;
  // The secondary RT buffer, used to store incoming entries after the main
  // buffer is full, until the resourcetimingbufferfull event fires.
  PerformanceEntryDeque resource_timing_secondary_buffer_;
  unsigned resource_timing_buffer_size_limit_;
  unsigned back_forward_cache_restoration_buffer_size_limit_;
  // A flag indicating that the buffer became full, the appropriate event was
  // queued, but haven't yet fired.
  bool resource_timing_buffer_full_event_pending_ = false;
  PerformanceEntryVector event_timing_buffer_;
  unsigned event_timing_buffer_max_size_;
  PerformanceEntryVector container_timing_buffer_;
  unsigned container_timing_buffer_max_size_;
  PerformanceEntryVector element_timing_buffer_;
  unsigned element_timing_buffer_max_size_;
  PerformanceEntryVector layout_shift_buffer_;
  PerformanceEntryVector largest_contentful_paint_buffer_;
  PerformanceEntryVector longtask_buffer_;
  PerformanceEntryVector visibility_state_buffer_;
  PerformanceEntryVector back_forward_cache_restoration_buffer_;
  PerformanceEntryVector soft_navigation_buffer_;
  PerformanceEntryVector long_animation_frame_buffer_;
  Member<PerformanceNavigationTiming> navigation_timing_;
  Member<UserTiming> user_timing_;
  PerformanceEntryVector paint_entries_timing_;
  Member<PerformanceEventTiming> first_input_timing_;

  base::TimeTicks time_origin_;
  base::TimeDelta unix_at_zero_monotonic_;
  const base::TickClock* tick_clock_;
  bool cross_origin_isolated_capability_;

  PerformanceEntryTypeMask observer_filter_options_;
  HeapLinkedHashSet<Member<PerformanceObserver>> observers_;
  HeapLinkedHashSet<Member<PerformanceObserver>> active_observers_;
  HeapLinkedHashSet<Member<PerformanceObserver>> suspended_observers_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  HeapTaskRunnerTimer<Performance> deliver_observations_timer_;
  HeapTaskRunnerTimer<Performance> resource_timing_buffer_full_timer_;

  // A map from entry types to the number of dropped entries of that given entry
  // type. Entries are dropped when the buffer from that entry type is full.
  WTF::HashMap<PerformanceEntry::EntryType, int> dropped_entries_count_map_;

  // See crbug.com/1181774.
  Member<BackgroundTracingHelper> background_tracing_helper_;

  // Running counter for LongTask observations.
  size_t long_task_counter_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_H_
