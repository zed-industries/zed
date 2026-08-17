// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_EVENT_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_EVENT_TIMING_H_

#include "third_party/blink/public/common/input/pointer_id.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/timing/performance.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"

namespace perfetto::protos::pbzero {
class EventTiming;
}  // namespace perfetto::protos::pbzero

namespace blink {

class Frame;

class CORE_EXPORT PerformanceEventTiming final : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Information used for event timing reporting purpose only.
  struct EventTimingReportingInfo {
    // Presentation promise index in which the entry in |event_timing_| was
    // added.
    uint64_t presentation_index = 0;

    // The event creation timestamp. This and the times below are the
    // exact (non-rounded) monotonic timestamps. They are used for repoerting.
    // They should not be exposed to performance observer API entries for
    // security and privacy reasons.
    base::TimeTicks creation_time;

    // This is the timestamp when the original WebInputEvent was queued on main
    // thread.
    base::TimeTicks enqueued_to_main_thread_time;

    base::TimeTicks processing_start_time;

    base::TimeTicks processing_end_time;

    // This is the timestamp when the commit has finished on compositor thread.
    // Only not null for event timings that have a next paint.
    base::TimeTicks commit_finish_time;

    // Other than reporting to UKM, this is also used by eventTiming trace
    // events to report accurate ending time.
    // Only not null for event timing that have a next paint.
    base::TimeTicks presentation_time;

    // The fallback timestamp used to calculate event duration. It is set only
    // when fallback time is used.  Some events might still have a next paint.
    base::TimeTicks fallback_time;

    // The render start time.
    // Only not null for event timings that have a next paint.
    base::TimeTicks render_start_time;

    // Keycode for the event. If the event is not a keyboard event, the keycode
    // wouldn't be set.
    std::optional<int> key_code = std::nullopt;

    // PointerId for the event. If the event is not a pointer event, the
    // PointerId wouldn't be set.
    std::optional<PointerId> pointer_id = std::nullopt;

    bool prevent_counting_as_interaction = false;

    // Set to true when the event's processing time is fully nested under
    // another event's processing time, e.g. some synthetic events are
    // dispatched with the input events, beforeinput event's processing time is
    // nested under keypress event.
    bool is_processing_fully_nested_in_another_event = false;
  };

  static PerformanceEventTiming* Create(const AtomicString& event_type,
                                        EventTimingReportingInfo reporting_info,
                                        bool cancelable,
                                        Node* target,
                                        DOMWindow* source);

  static PerformanceEventTiming* CreateFirstInputTiming(
      PerformanceEventTiming* entry);

  PerformanceEventTiming(const AtomicString& event_type,
                         const AtomicString& entry_type,
                         EventTimingReportingInfo repoerting_info,
                         bool cancelable,
                         Node* target,
                         DOMWindow* source);

  ~PerformanceEventTiming() override;

  const AtomicString& entryType() const override { return entry_type_; }
  PerformanceEntryType EntryTypeEnum() const override;

  bool cancelable() const { return cancelable_; }

  DOMHighResTimeStamp processingStart() const;
  DOMHighResTimeStamp processingEnd() const;

  Node* target() const;

  void SetTarget(Node* target);

  uint64_t interactionId() const;

  void SetInteractionId(uint64_t interaction_id);

  bool HasKnownInteractionID() const;

  bool HasKnownEndTime() const;

  bool IsReadyForReporting() const;

  base::TimeTicks GetEndTime() const;

  void UpdateFallbackTime(base::TimeTicks fallback_time);

  uint32_t interactionOffset() const;

  void SetInteractionIdAndOffset(uint32_t interaction_id,
                                 uint32_t interaction_offset);

  void SetDuration(double duration);

  void BuildJSONValue(V8ObjectBuilder&) const override;

  void Trace(Visitor*) const override;

  // TODO(sullivan): Remove the deprecated TracedValue when DevTools migrates
  // to the perfetto events.
  std::unique_ptr<TracedValue> ToTracedValue(Frame* frame) const;
  void SetPerfettoData(Frame* frame,
                       perfetto::protos::pbzero::EventTiming* traced_value,
                       base::TimeTicks time_origin);

  // Getters and setters of the EventTimingReportingInfo object.
  EventTimingReportingInfo* GetEventTimingReportingInfo() {
    return &reporting_info_;
  }

  bool NeedsNextPaintMeasurement() const;

 private:
  AtomicString entry_type_;
  mutable DOMHighResTimeStamp processing_start_ = 0;
  mutable DOMHighResTimeStamp processing_end_ = 0;
  bool cancelable_;
  WeakMember<Node> target_;
  std::optional<uint64_t> interaction_id_ = std::nullopt;
  uint32_t interaction_offset_ = 0;

  EventTimingReportingInfo reporting_info_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_EVENT_TIMING_H_
