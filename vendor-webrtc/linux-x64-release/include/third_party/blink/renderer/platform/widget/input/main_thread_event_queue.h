// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_MAIN_THREAD_EVENT_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_MAIN_THREAD_EVENT_QUEUE_H_

#include <memory>
#include <optional>

#include "base/feature_list.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "base/timer/timer.h"
#include "cc/input/touch_action.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/input/web_input_event_attribution.h"
#include "third_party/blink/public/mojom/input/input_event_result.mojom-shared.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom-blink.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/widget_scheduler.h"
#include "third_party/blink/renderer/platform/widget/input/input_event_prediction.h"
#include "third_party/blink/renderer/platform/widget/input/main_thread_event_queue_task_list.h"
#include "ui/latency/latency_info.h"

namespace cc {
class EventMetrics;
}

namespace blink {

using HandledEventCallback =
    base::OnceCallback<void(mojom::blink::InputEventResultState ack_state,
                            const ui::LatencyInfo& latency_info,
                            mojom::blink::DidOverscrollParamsPtr,
                            std::optional<cc::TouchAction>)>;

// All interaction with the MainThreadEventQueueClient will occur
// on the main thread.
class PLATFORM_EXPORT MainThreadEventQueueClient {
 public:
  // Handle an `event` that was previously queued (possibly coalesced with
  // another event). `metrics` contains information that would be useful in
  // reporting latency metrics in case the event causes an update. Returns false
  // if the event will not be handled in which case the `handled_callback` will
  // not be run.
  virtual bool HandleInputEvent(const WebCoalescedInputEvent& event,
                                std::unique_ptr<cc::EventMetrics> metrics,
                                HandledEventCallback handled_callback) = 0;

  // Notify clients that the queued events have been dispatched. `raf_aligned`
  // determines whether the events were rAF-aligned events or non-rAF-aligned
  // ones.
  virtual void InputEventsDispatched(bool raf_aligned) = 0;

  // Requests a BeginMainFrame callback from the compositor.
  virtual void SetNeedsMainFrame(bool urgent) = 0;

  // Returns true if a main frame has been requested and has not yet run.
  virtual bool RequestedMainFramePending() = 0;
};

// MainThreadEventQueue implements a queue for events that need to be
// queued between the compositor and main threads. This queue is managed
// by a lock where events are enqueued by the compositor thread
// and dequeued by the main thread.
//
// Below some example flows are how the code behaves.
// Legend: B=Browser, C=Compositor, M=Main Thread, NB=Non-blocking
//         BL=Blocking, PT=Post Task, ACK=Acknowledgement
//
// Normal blocking event sent to main thread.
//   B        C        M
//   ---(BL)-->
//         (queue)
//            ---(PT)-->
//                  (deque)
//   <-------(ACK)------
//
// Non-blocking event sent to main thread.
//   B        C        M
//   ---(NB)-->
//         (queue)
//            ---(PT)-->
//                  (deque)
//
// Non-blocking followed by blocking event sent to main thread.
//   B        C        M
//   ---(NB)-->
//         (queue)
//            ---(PT)-->
//   ---(BL)-->
//         (queue)
//            ---(PT)-->
//                  (deque)
//                  (deque)
//   <-------(ACK)------
//
class PLATFORM_EXPORT MainThreadEventQueue
    : public ThreadSafeRefCounted<MainThreadEventQueue> {
 public:
  MainThreadEventQueue(
      MainThreadEventQueueClient* client,
      const scoped_refptr<base::SingleThreadTaskRunner>& compositor_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<scheduler::WidgetScheduler> widget_scheduler,
      bool allow_raf_aligned_input);
  MainThreadEventQueue(const MainThreadEventQueue&) = delete;
  MainThreadEventQueue& operator=(const MainThreadEventQueue&) = delete;

  // Type of dispatching of the event.
  enum class DispatchType { kBlocking, kNonBlocking };

  // Called once the compositor has handled |event| and indicated that it is
  // a non-blocking event to be queued to the main thread.
  void HandleEvent(std::unique_ptr<WebCoalescedInputEvent> event,
                   DispatchType dispatch_type,
                   mojom::blink::InputEventResultState ack_result,
                   const WebInputEventAttribution& attribution,
                   std::unique_ptr<cc::EventMetrics> metrics,
                   HandledEventCallback handled_callback,
                   bool allow_main_gesture_scroll = false);
  void DispatchRafAlignedInput(base::TimeTicks frame_time);
  void QueueClosure(base::OnceClosure closure);

  void ClearClient();
  void SetNeedsLowLatency(bool low_latency);
  void SetNeedsUnbufferedInputForDebugger(bool unbuffered);

  void SetHasPointerRawUpdateEventHandlers(bool has_handlers);

  // Request unbuffered input events until next pointerup.
  void RequestUnbufferedInputEvents();

  // Resampling event before dispatch it.
  void HandleEventResampling(
      const std::unique_ptr<MainThreadEventQueueTask>& item,
      base::TimeTicks frame_time);

  static bool IsForwardedAndSchedulerKnown(
      mojom::blink::InputEventResultState ack_state) {
    return ack_state == mojom::blink::InputEventResultState::kNotConsumed ||
           ack_state ==
               mojom::blink::InputEventResultState::kSetNonBlockingDueToFling;
  }

  // Acquires a lock but use is restricted to tests.
  bool IsEmptyForTesting();

 protected:
  friend class ThreadSafeRefCounted<MainThreadEventQueue>;
  virtual ~MainThreadEventQueue();

  void QueueEvent(std::unique_ptr<MainThreadEventQueueTask> event);
  void PostTaskToMainThread();
  void DispatchEvents();
  void PossiblyScheduleMainFrame();
  void SetNeedsMainFrame(bool urgent);
  // Returns false if the event can not be handled and the HandledEventCallback
  // will not be run.
  bool HandleEventOnMainThread(const WebCoalescedInputEvent& event,
                               const WebInputEventAttribution& attribution,
                               std::unique_ptr<cc::EventMetrics> metrics,
                               HandledEventCallback handled_callback);

  bool IsRawUpdateEvent(
      const std::unique_ptr<MainThreadEventQueueTask>& item) const;
  bool ShouldFlushQueue(
      const std::unique_ptr<MainThreadEventQueueTask>& item) const;
  bool IsRafAlignedEvent(
      const std::unique_ptr<MainThreadEventQueueTask>& item) const;
  void RafFallbackTimerFired();

  void ClearRafFallbackTimerForTesting();

  void UnblockQueuedBlockingTouchMovesIfNeeded(
      const WebInputEvent& dispatched_event,
      mojom::blink::InputEventResultState ack_result);

  friend class QueuedWebInputEvent;
  friend class MainThreadEventQueueTest;
  friend class MainThreadEventQueueInitializationTest;
  raw_ptr<MainThreadEventQueueClient> client_;
  const bool allow_raf_aligned_input_;

  // Contains data that are read and written on the main thread only.
  struct MainThreadOnly {
    bool blocking_touch_start_not_consumed = false;
    bool should_unblock_touch_moves = false;
  } main_thread_only_;
  MainThreadOnly& GetMainThreadOnly();

  // Contains data that are read and written on the compositor thread only.
  struct CompositorThreadOnly {
    bool last_touch_start_forced_nonblocking_due_to_fling = false;
  } compositor_thread_only_;
  CompositorThreadOnly& GetCompositorThreadOnly();

  // These variables are read on the compositor thread but are
  // written on the main thread, so we use atomics to keep them
  // lock free. Reading these variables off of the compositor thread
  // is best effort. It is fine that the compositor executes a slightly
  // different path for events in flight while these variables are
  // mutated via the main thread.
  //
  // As a result, use relaxed ordering for all accesses to these variables.
  std::atomic<bool> has_pointerrawupdate_handlers_ = false;
  std::atomic<bool> needs_low_latency_ = false;
  std::atomic<bool> needs_unbuffered_input_for_debugger_ = false;
  std::atomic<bool> needs_low_latency_until_pointer_up_ = false;

  // Contains data to be shared between main thread and compositor thread.
  struct SharedState {
    MainThreadEventQueueTaskList events_;
    // A BeginMainFrame has been requested but not received yet.
    bool sent_main_frame_request_ = false;
    // A PostTask to the main thread has been sent but not executed yet.
    bool sent_post_task_ = false;
    base::TimeTicks last_async_touch_move_timestamp_;
  };

  // Lock used to serialize |shared_state_|.
  base::Lock shared_state_lock_;
  SharedState shared_state_ GUARDED_BY(shared_state_lock_);

  scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;
  scoped_refptr<scheduler::WidgetScheduler> widget_scheduler_;

  // A safe guard timer to ensure input is always processed. A BeginMainFrame
  // signal might not always occur if our visibility changed.
  std::unique_ptr<base::OneShotTimer> raf_fallback_timer_;

  std::unique_ptr<InputEventPrediction> event_predictor_;

 private:
  // Returns false if we are trying to send a gesture scroll event to the main
  // thread when we shouldn't be.  Used for DCHECK in HandleEvent.
  bool Allowed(const WebInputEvent& event, bool force_allow);

  // Tracked here for DCHECK purposes only.  For cursor control we allow gesture
  // scroll events to go to main.  See CursorControlHandler (impl-side filter)
  // and WebFrameWidgetImpl::WillHandleGestureEvent (main thread consumer).
  bool cursor_control_in_progress_ = false;

#if DCHECK_IS_ON()
  scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner_;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_MAIN_THREAD_EVENT_QUEUE_H_
