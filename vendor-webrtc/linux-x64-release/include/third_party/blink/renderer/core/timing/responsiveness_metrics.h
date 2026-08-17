// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_RESPONSIVENESS_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_RESPONSIVENESS_METRICS_H_

#include <optional>

#include "base/time/time.h"
#include "base/trace_event/typed_macros.h"
#include "base/tracing/protos/chrome_track_event.pbzero.h"
#include "third_party/blink/public/common/responsiveness_metrics/user_interaction_latency.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/events/pointer_event.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"
#include "third_party/perfetto/include/perfetto/tracing/event_context.h"

namespace blink {

class PerformanceEventTiming;
class WindowPerformance;

class CORE_EXPORT ResponsivenessMetrics
    : public GarbageCollected<ResponsivenessMetrics> {
 public:
  // Timestamps for input events.
  struct EventTimestamps {
    // The duration of the event (creation --> first display update it caused).
    base::TimeDelta duration() const { return end_time - creation_time; }

    // The event creation time.
    base::TimeTicks creation_time;
    // The time when the original WebInputEvent was queued on main thread.
    base::TimeTicks queued_to_main_thread_time;
    // The time when commit was finished on compositor thread.
    base::TimeTicks commit_finish_time;
    // The time when the first display update caused by the input event was
    // performed.
    base::TimeTicks end_time;
  };

  // Wrapper class to store interactionId, interaction offset, and timestamps
  // of an entry on a HashMap.
  class InteractionInfo {
   public:
    InteractionInfo(uint32_t interaction_id,
                    uint32_t interaction_offset,
                    EventTimestamps timestamps)
        : interaction_id_(interaction_id),
          interaction_offset_(interaction_offset),
          timestamps_({timestamps}) {}

    InteractionInfo() = default;
    ~InteractionInfo() = default;
    uint32_t GetInteractionId() const { return interaction_id_; }
    uint32_t GetInteractionOffset() const { return interaction_offset_; }
    void SetInteractionIdAndOffset(uint32_t interaction_id,
                                   uint32_t interaction_offset) {
      interaction_id_ = interaction_id;
      interaction_offset_ = interaction_offset;
    }
    Vector<EventTimestamps> const& GetTimeStamps() { return timestamps_; }
    bool Empty() { return timestamps_.empty(); }
    void AddTimestamps(EventTimestamps timestamp) {
      timestamps_.push_back(timestamp);
    }
    void Clear() {
      interaction_id_ = 0;
      interaction_offset_ = 0;
      timestamps_.clear();
    }

   private:
    // InteractionId associated with the entry.
    uint32_t interaction_id_ = 0;
    uint32_t interaction_offset_ = 0;
    // Timestamps associated with the entries of the same interaction.
    Vector<EventTimestamps> timestamps_;
  };

  // Wrapper class to store PerformanceEventTiming, pointerdown and pointerup
  // timestamps, on a HeapHashMap.
  class PointerEntryAndInfo : public GarbageCollected<PointerEntryAndInfo> {
   public:
    PointerEntryAndInfo(PerformanceEventTiming* entry,
                        EventTimestamps timestamps)
        : entry_(entry), timestamps_({timestamps}) {}

    static PointerEntryAndInfo* Create(PerformanceEventTiming* entry,
                                       EventTimestamps timestamps) {
      return MakeGarbageCollected<PointerEntryAndInfo>(entry, timestamps);
    }
    ~PointerEntryAndInfo() = default;
    void Trace(Visitor*) const;
    PerformanceEventTiming* GetEntry() const { return entry_.Get(); }
    Vector<EventTimestamps>& GetTimeStamps() { return timestamps_; }

   private:
    // The PerformanceEventTiming entry that has not been sent to observers
    // yet: the event dispatch has been completed but the presentation promise
    // used to determine |duration| has not yet been resolved, , or the
    // interactionId has not yet been computed yet.
    Member<PerformanceEventTiming> entry_;
    // Timestamps associated with the entry. The first should always be
    // for a pointerdown, the second for a pointerup, and optionally the third
    // for a click.
    Vector<EventTimestamps> timestamps_;
  };

  explicit ResponsivenessMetrics(WindowPerformance*);
  ~ResponsivenessMetrics();

  void FlushAllEventsAtPageHidden();

  // Flush UKM timestamps of composition events for testing.
  void FlushAllEventsForTesting();

  // Stop UKM sampling for testing.
  void StopUkmSamplingForTesting() { sampling_ = false; }

  // Assigns an interactionId and records interaction latency for pointer
  // events. Returns true if the entry is ready to be surfaced in
  // PerformanceObservers and the Performance Timeline.
  bool SetPointerIdAndRecordLatency(PerformanceEventTiming* entry,
                                    EventTimestamps event_timestamps);

  // Assigns interactionId and records interaction latency for keyboard events.
  // We care about input, compositionstart, and compositionend events, so
  // |key_code| will be std::nullopt in those cases.
  void SetKeyIdAndRecordLatency(PerformanceEventTiming* entry,
                                EventTimestamps event_timestamps);

  // Clears all keydowns in |key_code_to_interaction_info_map_| and report to
  // UKM.
  void FlushKeydown();

  uint32_t GetInteractionCount() const;

  void Trace(Visitor*) const;

  void EmitInteractionToNextPaintTraceEvent(
      const ResponsivenessMetrics::EventTimestamps& event,
      bool is_pointer_event,
      base::TimeDelta total_event_duration);

  void SetCurrentInteractionEventQueuedTimestamp(base::TimeTicks queued_time);
  base::TimeTicks CurrentInteractionEventQueuedTimestamp() const;

  // TODO: Revisit if this is redandunt.
  struct KeycodeInfo {
    int keycode;
    uint32_t interactionId;
    uint32_t interactionOffset;
  };

 private:
  // Record UKM for user interaction latencies.
  void RecordUserInteractionUKM(
      LocalDOMWindow* window,
      UserInteractionType interaction_type,
      const WTF::Vector<ResponsivenessMetrics::EventTimestamps>& timestamps,
      uint32_t interaction_offset);

  void RecordTapOrClickUKM(LocalDOMWindow*, PointerEntryAndInfo&);

  void RecordKeyboardUKM(LocalDOMWindow* window,
                         const WTF::Vector<EventTimestamps>& event_timestamps,
                         uint32_t interaction_offset);

  // Updates the interactionId counter which is used by Event Timing.
  void UpdateInteractionId();

  uint32_t GetCurrentInteractionId() const;

  // Method called when |pointer_flush_timer_| fires. Ensures that the last
  // interaction of any given pointerId is reported, even if it does not receive
  // a click.
  void FlushPointerTimerFired(TimerBase*);

  // Method called when |contextmenu_flush_timer_| fires. Ensures that the last
  // pointerdown or keydown is reported, even if it does not receive a pointerup
  // nor keyup.
  void ContextmenuFlushTimerFired(TimerBase*);

  // Used to flush any entries in |pointer_id_entry_map_| which already have
  // pointerup. We either know there is no click happening or waited long enough
  // for a click to occur.
  void FlushAllPointerdownWithMeasuredPointerup();

  // Used to flush all entries in |pointer_id_entry_map_|.
  void FlushAllPointerdown();

  // Method called when |composition_end_| fires. Ensures that the last
  // interaction of compositoin events is reported, even if
  // there is no following keydown.
  void FlushCompositionEndTimerFired(TimerBase*);

  // Used to flush any entries in |keyboard_sequence_based_timestamps_to_UKM_|
  void FlushSequenceBasedKeyboardEvents();

  void NotifyPointerdown(PerformanceEventTiming* entry) const;

  // Indicates if a key is being held for a sustained period of time
  bool IsHoldingKey(std::optional<int> key_code);

  bool TryHandleKeyboardEventSimulatedClick(
      PerformanceEventTiming* entry,
      const std::optional<PointerId>& last_pointer_id);

  Member<WindowPerformance> window_performance_;

  // Map from keyCodes to interaction info (ID, offset, and timestamps).
  HashMap<int, InteractionInfo, IntWithZeroKeyHashTraits<int>>
      key_code_to_interaction_info_map_;

  enum CompositionState {
    kNonComposition,
    kCompositionContinueOngoingInteraction,
    kCompositionStartNewInteractionOnKeydown,
    kCompositionStartNewInteractionOnInput,
    kEndCompositionOnKeydown
  };

  CompositionState composition_state_ = kNonComposition;

  std::optional<KeycodeInfo> last_keydown_keycode_info_;
  // InteractionInfo storing interactionId, interaction offset, and timestamps
  // of entries for reporting them to UKM in 3 main cases:
  //  1) Pressing a key under composition.
  //  2) Holding a key under composition.
  //  3) Holding a key under no composition.
  InteractionInfo sequence_based_keyboard_interaction_info_;

  // Map from pointerId to the first pointer event entry seen for the user
  // interaction, and other information.
  HeapHashMap<PointerId,
              Member<PointerEntryAndInfo>,
              IntWithZeroKeyHashTraits<PointerId>>
      pointer_id_entry_map_;
  HeapTaskRunnerTimer<ResponsivenessMetrics> pointer_flush_timer_;
  HeapTaskRunnerTimer<ResponsivenessMetrics> contextmenu_flush_timer_;
  HeapTaskRunnerTimer<ResponsivenessMetrics> composition_end_flush_timer_;
  // The PointerId of the last pointerdown or pointerup event processed. Used to
  // know which interactionId to use for click events. If pointecancel or
  // keyboard events are seen, the value is reset. TODO(crbug.com/1264930):
  // remove this attribute once PointerId for clicks correctly points to the
  // same value as its corresponding pointerdown and pointerup.
  std::optional<PointerId> last_pointer_id_;

  // Indicate whether the last pointerup event had a paired pointerdown event
  // or otherwise its related pointerdown event was optimized out. This is
  // added only for the purpose of analyzing how often an orphan pointerup can
  // come with click, and should be removed once the experiment is done.
  bool is_last_pointerup_orphan_ = false;

  // Queued timestamp of current event being dispatched.
  base::TimeTicks current_interaction_event_queued_timestamp_;

  uint32_t current_interaction_id_for_event_timing_;
  uint32_t interaction_count_ = 0;

  // Whether to perform UKM sampling.
  bool sampling_ = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_RESPONSIVENESS_METRICS_H_
