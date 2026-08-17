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
#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_WINDOW_PERFORMANCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_WINDOW_PERFORMANCE_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/events/pointer_event.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/performance_monitor.h"
#include "third_party/blink/renderer/core/page/page_visibility_observer.h"
#include "third_party/blink/renderer/core/timing/event_counts.h"
#include "third_party/blink/renderer/core/timing/memory_info.h"
#include "third_party/blink/renderer/core/timing/performance.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/core/timing/performance_event_timing.h"
#include "third_party/blink/renderer/core/timing/performance_navigation.h"
#include "third_party/blink/renderer/core/timing/performance_timing.h"
#include "third_party/blink/renderer/core/timing/performance_timing_for_reporting.h"
#include "third_party/blink/renderer/core/timing/responsiveness_metrics.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace viz {
struct FrameTimingDetails;
}

namespace blink {

class AnimationFrameTimingInfo;

class CORE_EXPORT WindowPerformance final : public Performance,
                                            public PerformanceMonitor::Client,
                                            public ExecutionContextClient,
                                            public PageVisibilityObserver {
  friend class WindowPerformanceTest;
  friend class ResponsivenessMetrics;

 public:
  explicit WindowPerformance(LocalDOMWindow*);
  ~WindowPerformance() override;

  static base::TimeTicks GetTimeOrigin(LocalDOMWindow* window);

  ExecutionContext* GetExecutionContext() const override;

  PerformanceTiming* timing() const override;
  PerformanceTimingForReporting* timingForReporting() const;
  PerformanceNavigation* navigation() const override;

  MemoryInfo* memory(ScriptState*) const override;

  EventCounts* eventCounts() override;
  uint64_t interactionCount() const override;

  void PopulateContainerTimingEntries() override;
  void SetHasContainerTimingChanges();

  bool FirstInputDetected() const { return !!first_input_timing_; }

  void WillShowModalDialog();

  // EventTimingProcessingStart and EventTimingProcessingEnd are together used
  // to measure the processing duration of a new Event Timing.
  // There might be nested events being dispatched (e.g. `input` event nested
  // inside a raw pointer event), but the RAII class `EventTiming` uses the
  // stack to manage calling these functions (from constructor/destructor).
  // This means that calls to End will be in LIFO often with Start.
  //
  // Will create a `PerformanceEventTiming`, and if needed, requests the next
  // presentation time to calculate the full |duration| to next paint.
  void EventTimingProcessingStart(const Event& event,
                                  base::TimeTicks processing_start,
                                  EventTarget* hit_test_target);
  void EventTimingProcessingEnd(const Event& event,
                                base::TimeTicks processing_end);

  // Set commit finish time for all pending events that have finished processing
  // and are watiting for presentation promise to resolve.
  void SetCommitFinishTimeStampForPendingEvents(
      base::TimeTicks commit_finish_time);

  void UpdatePendingEventTimingsWithFallbackTime(base::TimeTicks fallback_time);

  // Set render start time for all pending events that have finished processing.
  void SetRenderStartTimeForPendingEvents(base::TimeTicks render_start_time);

  void OnPaintFinished();
  void OnBeginMainFrame(viz::BeginFrameId frame_id);

  void AddElementTiming(const AtomicString& name,
                        const String& url,
                        const gfx::RectF& rect,
                        const DOMPaintTimingInfo&,
                        base::TimeTicks load_time,
                        const AtomicString& identifier,
                        const gfx::Size& intrinsic_size,
                        const AtomicString& id,
                        Element*);

  void AddContainerTiming(const DOMPaintTimingInfo& paint_timing_info,
                          const gfx::Rect& rect,
                          uint64_t size,
                          const AtomicString& identifier,
                          Element* last_painted_element,
                          const DOMPaintTimingInfo& first_paint_timing_info);

  void OnBodyLoadFinished(int64_t encoded_body_size, int64_t decoded_body_size);
  void QueueLongAnimationFrameTiming(
      AnimationFrameTimingInfo*,
      std::optional<DOMPaintTimingInfo> paint_timing_info = std::nullopt);
  void AddFirstPaintTiming(const DOMPaintTimingInfo& paint_timing_info,
                           bool is_triggered_by_soft_navigation);

  void AddFirstContentfulPaintTiming(
      const DOMPaintTimingInfo& paint_timing_info,
      bool is_triggered_by_soft_navigation);

  // PerformanceMonitor::Client implementation.
  void ReportLongTask(base::TimeTicks start_time,
                      base::TimeTicks end_time,
                      ExecutionContext* task_context,
                      bool has_multiple_contexts) override;

  void AddLayoutShiftEntry(LayoutShift*);
  void AddVisibilityStateEntry(bool is_visible, base::TimeTicks start_time);
  void AddSoftNavigationEntry(const AtomicString& name,
                              base::TimeTicks start_time);

  // PageVisibilityObserver
  void PageVisibilityChanged() override;
  void PageVisibilityChangedWithTimestamp(
      base::TimeTicks visibility_change_timestamp);

  void OnLargestContentfulPaintUpdated(
      std::optional<DOMPaintTimingInfo> paint_timing_info,
      uint64_t paint_size,
      base::TimeTicks load_time,
      const AtomicString& id,
      const String& url,
      Element*,
      bool is_triggered_by_soft_navigation);

  void Trace(Visitor*) const override;

  ResponsivenessMetrics& GetResponsivenessMetrics() {
    return *responsiveness_metrics_;
  }

  const Event* GetCurrentEventTimingEvent() { return current_event_.Get(); }

  void CreateNavigationTimingInstance(
      mojom::blink::ResourceTimingInfoPtr navigation_resource_timing);

  void OnPageScroll();
  bool IsAutoscrollActive();
  void ResetAutoscroll() { autoscroll_active_ = false; }

 private:
  static std::pair<AtomicString, DOMWindow*> SanitizedAttribution(
      ExecutionContext*,
      bool has_multiple_contexts,
      LocalFrame* observer_frame);

  void BuildJSONValue(V8ObjectBuilder&) const override;

  void ReportAllPendingEventTimingsOnPageHidden();

  void FlushEventTimingsOnPageHidden();
  void AddLongAnimationFrameEntry(PerformanceEntry*);

  void OnPresentationPromiseResolved(
      uint64_t presentation_index,
      uint64_t expected_frame_source_id,
      const viz::FrameTimingDetails& presentation_details);
  // Report buffered events with presentation time following their registered
  // order; stop as soon as seeing an event with pending presentation promise.
  void ReportEventTimings();
  void ReportEvent(InteractiveDetector* interactive_detector,
                   Member<PerformanceEventTiming> event_timing_entry);

  void DispatchFirstInputTiming(PerformanceEventTiming* entry);

  // Assign an interaction id to an event timing entry if needed. Also records
  // the interaction latency. Returns true if the entry is ready to be surfaced
  // in PerformanceObservers and the Performance Timeline
  bool SetInteractionIdAndRecordLatency(
      PerformanceEventTiming* entry,
      ResponsivenessMetrics::EventTimestamps event_timestamps);

  // Notify observer that an event timing entry is ready and add it to the event
  // timing buffer if needed.
  void NotifyAndAddEventTimingBuffer(PerformanceEventTiming* entry);

  void ReportFirstInputTiming(PerformanceEventTiming* event_timing_entry);

  void SchedulePendingRenderCoarsenedEntries(base::TimeTicks target_time);
  void FlushPendingRenderCoarsenedEntries();

  // The last time the page visibility was changed.
  base::TimeTicks last_hidden_timestamp_;

  // A list of timestamps that javascript modal dialogs was showing. These are
  // timestamps right before start showing each dialog.
  Deque<base::TimeTicks> show_modal_dialog_timestamps_;

  // Frame source id from BeginMainFrame args. Event Timing compares it with
  // frame source id from presentation feedback to identify GPU crashes.
  // crbug.com/324877581
  uint64_t begin_main_frame_source_id_ = 0;
  // Controls if we register a new presentation promise upon events arrival.
  bool need_new_promise_for_event_presentation_time_ = true;
  // Counts the total number of presentation promises we've registered for
  // events' presentation feedback since the beginning.
  uint64_t event_presentation_promise_count_ = 0;

  // Store all event timing and latency related data, including
  // PerformanceEventTiming, presentation_index, keycode and pointerId.
  // We use the data to calculate events latencies.
  HeapVector<Member<PerformanceEventTiming>> event_timing_entries_;
  Member<PerformanceEventTiming> first_pointer_down_event_timing_;
  Member<EventCounts> event_counts_;
  mutable Member<PerformanceNavigation> navigation_;
  mutable Member<PerformanceTiming> timing_;
  mutable Member<PerformanceTimingForReporting> timing_for_reporting_;
  base::TimeTicks pending_pointer_down_start_time_;
  std::optional<base::TimeDelta> pending_pointer_down_processing_time_;
  std::optional<base::TimeDelta> pending_pointer_down_time_to_next_paint_;

  // Set to true when text selection causes scrolling in the page. Reset when
  // the mouse button is released and autoscroll stops. Used to ignore
  // recording interaction metrics for all the events during the text
  // selection autoscroll.
  // We do this because the interactions following a scroll can cause a lot of
  // work to be done (intersection observers, etc.) but this doesn't
  // necessarily result in a degraded user experience.
  // When users are actively scrolling a page, it is much harder to visualize
  // the latency for any one specific animation frame, not in the same way as a
  // typical discrete interaction, which are measured in INP only.
  // The interactions causing text selection autoscroll are generally rare and
  // not typically "designed by the site UI". It's more of user agent or
  // accessibility use case. We don't want any pages to fail INP because of
  // these interactions.
  bool autoscroll_active_ = false;

  bool has_container_timing_changes_ = false;

  // Calculate responsiveness metrics and record UKM for them.
  Member<ResponsivenessMetrics> responsiveness_metrics_;
  // The event we are currently processing.
  WeakMember<const Event> current_event_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_WINDOW_PERFORMANCE_H_
