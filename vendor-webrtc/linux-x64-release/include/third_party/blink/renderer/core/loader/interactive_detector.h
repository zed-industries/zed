// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_INTERACTIVE_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_INTERACTIVE_DETECTOR_H_

#include <optional>

#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/loader/long_task_detector.h"
#include "third_party/blink/renderer/core/page/page_hidden_state.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/traced_value.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/pod_interval.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
class TickClock;
}  // namespace base

namespace blink {

class Document;
class Event;

// Detects when a page reaches First Idle and Time to Interactive. See
// https://goo.gl/SYt55W for detailed description and motivation of First Idle
// and Time to Interactive.
// TODO(crbug.com/631203): This class currently only detects Time to
// Interactive. Implement First Idle.
class CORE_EXPORT InteractiveDetector
    : public GarbageCollected<InteractiveDetector>,
      public Supplement<Document>,
      public ExecutionContextLifecycleObserver,
      public LongTaskObserver {
 public:
  static const char kSupplementName[];

  // This class can be easily switched out to allow better testing of
  // InteractiveDetector.
  class CORE_EXPORT NetworkActivityChecker {
   public:
    explicit NetworkActivityChecker(Document* document) : document_(document) {}
    NetworkActivityChecker(const NetworkActivityChecker&) = delete;
    NetworkActivityChecker& operator=(const NetworkActivityChecker&) = delete;

    virtual int GetActiveConnections();
    virtual ~NetworkActivityChecker() = default;

   private:
    WeakPersistent<Document> document_;
  };

  static InteractiveDetector* From(Document&);
  // Exposed for tests. See crbug.com/810381. We must use a consistent address
  // for the supplement name.
  static const char* SupplementName();

  explicit InteractiveDetector(Document&,
                               std::unique_ptr<NetworkActivityChecker>);
  InteractiveDetector(const InteractiveDetector&) = delete;
  InteractiveDetector& operator=(const InteractiveDetector&) = delete;
  ~InteractiveDetector() override = default;

  // Calls to base::TimeTicks::Now().since_origin().InSecondsF() is expensive,
  // so we try not to call it unless we really have to. If we already have the
  // event time available, we pass it in as an argument.
  void OnResourceLoadBegin(std::optional<base::TimeTicks> load_begin_time);
  void OnResourceLoadEnd(std::optional<base::TimeTicks> load_finish_time);

  void SetNavigationStartTime(base::TimeTicks navigation_start_time);
  void OnFirstContentfulPaint(base::TimeTicks first_contentful_paint);
  void OnDomContentLoadedEnd(base::TimeTicks dcl_time);
  void OnInvalidatingInputEvent(base::TimeTicks invalidation_time);
  void OnPageHiddenChanged(bool is_hidden);

  // The duration between the hardware timestamp and being queued on the main
  // thread for the first click, tap, key press, cancelable touchstart, or
  // pointer down followed by a pointer up.
  std::optional<base::TimeDelta> GetFirstInputDelay() const;

  WTF::Vector<std::optional<base::TimeDelta>>
  GetFirstInputDelaysAfterBackForwardCacheRestore() const;

  // The timestamp of the event whose delay is reported by GetFirstInputDelay().
  std::optional<base::TimeTicks> GetFirstInputTimestamp() const;

  // The duration between the user's first scroll and display update.
  std::optional<base::TimeTicks> GetFirstScrollTimestamp() const;

  // The hardware timestamp of the first scroll after a navigation.
  std::optional<base::TimeDelta> GetFirstScrollDelay() const;

  // Process an input event, updating first_input_delay and
  // first_input_timestamp if needed. The event types we care about are
  // pointerdown, pointerup, mousedown, keydown, click, mouseup. And we
  // check the event types in the caller of this function.
  void HandleForInputDelay(const Event&,
                           base::TimeTicks event_platform_timestamp,
                           base::TimeTicks processing_start);

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  void Trace(Visitor*) const override;

  void SetTaskRunnerForTesting(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner_for_testing);
  // The caller owns the |clock| which must outlive the InteractiveDetector.
  void SetTickClockForTesting(const base::TickClock* clock);

  void RecordInputEventTimingUMA(base::TimeDelta processing_time,
                                 base::TimeDelta time_to_next_paint);

  void DidObserveFirstScrollDelay(base::TimeDelta first_scroll_delay,
                                  base::TimeTicks first_scroll_timestamp);

  void OnRestoredFromBackForwardCache();

 private:
  friend class InteractiveDetectorTest;

  const base::TickClock* clock_;

  base::TimeTicks interactive_time_;
  base::TimeTicks interactive_detection_time_;

  // Page event times that Interactive Detector depends on.
  // Null base::TimeTicks values indicate the event has not been detected yet.
  struct {
    base::TimeTicks first_contentful_paint;
    base::TimeTicks dom_content_loaded_end;
    base::TimeTicks nav_start;
    // The timestamp of the first input that would invalidate a Time to
    // Interactive computation. This is used when reporting Time To Interactive
    // on a trace event.
    base::TimeTicks first_invalidating_input;
    std::optional<base::TimeDelta> first_input_delay;
    std::optional<base::TimeTicks> first_input_timestamp;
    std::optional<base::TimeTicks> first_scroll_timestamp;
    std::optional<base::TimeDelta> frist_scroll_delay;

    WTF::Vector<std::optional<base::TimeDelta>>
        first_input_delays_after_back_forward_cache_restore;
  } page_event_times_;

  struct VisibilityChangeEvent {
    base::TimeTicks timestamp;
    bool was_hidden;
  };

  // Stores sufficiently long quiet windows on the network.
  Vector<WTF::PODInterval<base::TimeTicks>> network_quiet_windows_;

  // Stores long tasks in order to compute Total Blocking Time (TBT) once Time
  // To Interactive (TTI) is known.
  Vector<WTF::PODInterval<base::TimeTicks>> long_tasks_;

  // Start time of currently active network quiet windows.
  // Null base::TimeTicks values indicate network is not quiet at the moment.
  base::TimeTicks active_network_quiet_window_start_;

  // Adds currently active quiet network quiet window to the
  // vector. Should be called before calling FindInteractiveCandidate.
  void AddCurrentlyActiveNetworkQuietInterval(base::TimeTicks current_time);
  // Undoes AddCurrentlyActiveNetworkQuietInterval.
  void RemoveCurrentlyActiveNetworkQuietInterval();

  std::unique_ptr<NetworkActivityChecker> network_activity_checker_;
  int ActiveConnections();
  void BeginNetworkQuietPeriod(base::TimeTicks current_time);
  void EndNetworkQuietPeriod(base::TimeTicks current_time);
  // Updates current network quietness tracking information. Opens and closes
  // network quiet windows as necessary.
  void UpdateNetworkQuietState(double request_count,
                               std::optional<base::TimeTicks> current_time);

  HeapTaskRunnerTimer<InteractiveDetector> time_to_interactive_timer_;
  base::TimeTicks time_to_interactive_timer_fire_time_;
  void StartOrPostponeCITimer(base::TimeTicks timer_fire_time);
  void TimeToInteractiveTimerFired(TimerBase*);
  void CheckTimeToInteractiveReached();
  void OnTimeToInteractiveDetected();
  base::TimeDelta ComputeTotalBlockingTime();

  Vector<VisibilityChangeEvent> visibility_change_events_;
  bool initially_hidden_;
  // Returns true if page was ever backgrounded in the range
  // [event_time, base::TimeTicks::Now()].
  bool PageWasBackgroundedSinceEvent(base::TimeTicks event_time);

  // Finds a window of length kTimeToInteractiveWindowSeconds after lower_bound
  // such that both main thread and network are quiet. Returns the end of last
  // long task before that quiet window, or lower_bound, whichever is bigger -
  // this is called the Interactive Candidate. Returns 0.0 if no such quiet
  // window is found.
  base::TimeTicks FindInteractiveCandidate(base::TimeTicks lower_bound,
                                           base::TimeTicks current_time);

  // LongTaskObserver implementation
  void OnLongTaskDetected(base::TimeTicks start_time,
                          base::TimeTicks end_time) override;

  // The duration between the hardware timestamp and when we received the event
  // for the previous pointer down. Only non-zero if we've received a pointer
  // down event, and haven't yet reported the first input delay.
  base::TimeDelta pending_pointerdown_delay_;
  base::TimeDelta pending_mousedown_delay_;
  // The timestamp of a pending pointerdown event. Valid in the same cases as
  // pending_pointerdown_delay_.
  base::TimeTicks pending_pointerdown_timestamp_;
  base::TimeTicks pending_mousedown_timestamp_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_INTERACTIVE_DETECTOR_H_
